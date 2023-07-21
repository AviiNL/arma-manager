use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, Query, State},
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    model::*,
    repository::{UserRepository, UserTokenRepository},
    response::{ApiResult, ErrorResponse},
    Config,
};

#[derive(serde::Deserialize, Debug)]
pub struct TokenQuery {
    pub token: Option<String>,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(config): State<Config>,
    Extension(user_repository): Extension<UserRepository>,
    Extension(token_repository): Extension<UserTokenRepository>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(token_query): Query<TokenQuery>,
    mut req: Request<B>,
    next: Next<B>,
) -> ApiResult<impl IntoResponse> {
    let token = cookie_jar.get("token").map_or_else(
        || {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        let token = auth_value[7..].to_string();
                        Some(token)
                    } else {
                        return None;
                    }
                })
                .or_else(|| token_query.token.clone())
        },
        |c| {
            let value = c.value().to_string();
            if value.is_empty() {
                return None;
            }
            Some(value)
        },
    );

    let token = token.ok_or_else(|| ErrorResponse::new("Unauthorized").with_status_code(StatusCode::UNAUTHORIZED))?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(&config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| ErrorResponse::new("Invalid Token").with_status_code(StatusCode::UNAUTHORIZED))?
    .claims;

    let token_user = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| ErrorResponse::new("Invalid Token").with_status_code(StatusCode::UNAUTHORIZED))?;

    let token_user = token_repository
        .get_by_token(token_user)
        .await
        .map_err(|e| ErrorResponse::new(format!("Error fetching token from database: {}", e)))?;

    let token_user =
        token_user.ok_or_else(|| ErrorResponse::new("Invalid Token").with_status_code(StatusCode::UNAUTHORIZED))?;

    // this ip address doesnt take proxy into account
    let user_ip = token_user
        .ip
        .parse::<IpAddr>()
        .map_err(|_| ErrorResponse::new("Invalid Token").with_status_code(StatusCode::UNAUTHORIZED))?;

    let ip_address = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|ip| Some(ip.to_str().ok()?.parse::<IpAddr>().ok()?))
        .unwrap_or_else(|| addr.ip());

    if user_ip != ip_address {
        return Err(ErrorResponse::new("Invalid Token")
            .with_status_code(StatusCode::UNAUTHORIZED)
            .into());
    }

    let user_id = token_user.user_id;

    let user = user_repository
        .get_by_id(user_id)
        .await
        .map_err(|e| ErrorResponse::new(format!("Error fetching user from database: {}", e)))?;

    let user = user.ok_or_else(|| {
        ErrorResponse::new("The user belonging to this token no longer exists")
            .with_status_code(StatusCode::UNAUTHORIZED)
    })?;

    if !user.verified {
        return Err(ErrorResponse::new("User not verified")
            .with_status_code(StatusCode::UNAUTHORIZED)
            .into());
    }

    req.extensions_mut().insert(token_user);
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

// Err(ErrorResponse::new("Unauthorized").with_status_code(StatusCode::UNAUTHORIZED).into())
