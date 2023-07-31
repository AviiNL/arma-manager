use std::net::{IpAddr, SocketAddr};

use api_schema::{request::*, response::*};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{ConnectInfo, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use crate::{
    model::{TokenClaims, User, UserToken},
    repository::*,
    response::{ApiResponse, ApiResult, ErrorResponse},
    AppState,
};

pub async fn register_user_handler(
    Extension(user_repository): Extension<UserRepository>,
    Extension(token_repository): Extension<UserTokenRepository>,
    Json(body): Json<RegisterUserSchema>,
) -> ApiResult<impl IntoResponse> {
    let email = body.email.to_owned().to_ascii_lowercase();

    let user_exists = user_repository
        .exists(&email)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    if user_exists {
        return Err(ErrorResponse::new("Email already in use").into());
    }

    let user = user_repository
        .create(&body)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    let tokens = token_repository
        .all_by_user_id(user.id)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(filter_user_record(&user, &tokens)).with_root_key_name("user"))
}

pub async fn login_user_handler(
    headers: HeaderMap,
    State(data): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(user_repository): Extension<UserRepository>,
    Extension(token_repository): Extension<UserTokenRepository>,
    Json(body): Json<LoginUserSchema>,
) -> ApiResult<impl IntoResponse> {
    let email = body.email.to_ascii_lowercase();
    let user = user_repository
        .get_by_email(&email)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?
        .ok_or_else(|| {
            // Invalid email. We don't want to leak that *only* the email is valid.
            ErrorResponse::new("Invalid email or password").with_status_code(StatusCode::UNAUTHORIZED)
        })?;

    if !user.verified {
        return Err(ErrorResponse::new("User not verified")
            .with_status_code(StatusCode::UNAUTHORIZED)
            .into());
    }

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        // Invalid password. We don't want to leak that *only* the password is invalid.
        return Err(ErrorResponse::new("Invalid email or password")
            .with_status_code(StatusCode::UNAUTHORIZED)
            .into());
    }

    let ip_address = headers
        .get("X-Forwarded-For")
        .and_then(|ip| ip.to_str().ok()?.parse::<IpAddr>().ok())
        .unwrap_or_else(|| addr.ip());

    let user_token = token_repository
        .create(user.id, ip_address)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    let user_token = user_token.token;

    let now = time::OffsetDateTime::now_utc();
    let iat = now.unix_timestamp() as usize;
    let exp = (now + time::Duration::days(30)).unix_timestamp() as usize;

    let claims = TokenClaims {
        sub: user_token.to_string(),
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.config.jwt_secret.as_bytes()),
    )
    .map_err(|e| ErrorResponse::new(format!("JWT Error: {}", e)))?;

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::minutes(60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish()
        .to_string()
        .parse()
        .map_err(|e| ErrorResponse::new(format!("Cookie Error: {}", e)))?;

    let mut response = ApiResponse::new(ApiToken { token })
        .with_root_key_name("token")
        .into_response();
    response.headers_mut().insert(header::SET_COOKIE, cookie);

    Ok(response)
}

pub async fn update_user_handler(
    Extension(user_repository): Extension<UserRepository>,
    Extension(token_repository): Extension<UserTokenRepository>,
    Extension(user): Extension<User>,
    Json(body): Json<UpdateUserSchema>,
) -> ApiResult<impl IntoResponse> {
    let email = body.email.to_owned().to_ascii_lowercase();

    let id = if let Some(id) = body.id.as_ref() {
        Uuid::parse_str(id).map_err(|_| ErrorResponse::new("Invalid id").with_status_code(StatusCode::BAD_REQUEST))?
    } else {
        user.id
    };

    let user = user_repository
        .get_by_id(id)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?
        .ok_or_else(|| ErrorResponse::new("User not found").with_status_code(StatusCode::NOT_FOUND))?;

    // only test email if it changed
    if email != user.email {
        let user_exists = user_repository
            .exists(&email)
            .await
            .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

        if user_exists {
            return Err(ErrorResponse::new("Email already in use").into());
        }
    }

    let user = user_repository
        .update(id, &body)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    let tokens = token_repository
        .all_by_user_id(user.id)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(filter_user_record(&user, &tokens)).with_root_key_name("user"))
}

pub async fn revoke_token(
    Extension(token_repository): Extension<UserTokenRepository>,
    Extension(user): Extension<User>,
    Json(body): Json<RevokeTokenSchema>,
) -> ApiResult<impl IntoResponse> {
    let token = uuid::Uuid::parse_str(&body.token)
        .map_err(|_| ErrorResponse::new("Invalid token").with_status_code(StatusCode::BAD_REQUEST))?;
    token_repository
        .delete(user.id, token)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    let tokens = token_repository
        .all_by_user_id(user.id)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(filter_user_record(&user, &tokens)).with_root_key_name("user"))
}

pub async fn logout_handler(
    Extension(token_repository): Extension<UserTokenRepository>,
    Extension(user): Extension<User>,
    Extension(token): Extension<UserToken>,
) -> ApiResult<impl IntoResponse> {
    token_repository
        .delete(user.id, token.token)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .expires(time::OffsetDateTime::now_utc() - time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    })
    .into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn get_me_handler(
    Extension(user): Extension<User>,
    Extension(token_repository): Extension<UserTokenRepository>,
) -> ApiResult<impl IntoResponse> {
    let tokens = token_repository
        .all_by_user_id(user.id)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(filter_user_record(&user, &tokens)).with_root_key_name("user"))
}

pub async fn get_users_without_tokens(
    Extension(user_repository): Extension<UserRepository>,
) -> ApiResult<impl IntoResponse> {
    let users = user_repository
        .all()
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(
        users
            .into_iter()
            .map(|user| filter_user_record(&user, &vec![]))
            .collect::<Vec<_>>(),
    ))
}

fn filter_user_record(user: &User, tokens: &Vec<UserToken>) -> FilteredUser {
    let mut filtered_tokens = Vec::new();
    for token in tokens {
        filtered_tokens.push(FilteredUserToken {
            token: token.token.to_string(),
            ip: token.ip.to_owned(),
            created_at: token.created_at.unwrap().unix_timestamp(),
            last_used: token.last_used.unwrap().unix_timestamp(),
        });
    }

    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        roles: user.roles.clone().into(),
        verified: user.verified,
        tokens: filtered_tokens,
        created_at: user.created_at.unwrap().unix_timestamp(),
        updated_at: user.updated_at.unwrap().unix_timestamp(),
    }
}
