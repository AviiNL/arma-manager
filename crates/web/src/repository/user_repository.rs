use api_schema::request::{RegisterUserSchema, UpdateUserSchema};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;
use sqlx::{QueryBuilder, SqlitePool};
use uuid::Uuid;

use crate::model::*;

use super::RepositoryResult;

#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl UserRepository {
    pub async fn get_by_id(&self, id: uuid::Uuid) -> RepositoryResult<Option<User>> {
        let user = sqlx::query_as!(
            crate::model::User,
            r#"
            SELECT
                id AS "id: Uuid",
                name,
                email,
                verified,
                password,
                roles AS "roles: Roles",
                created_at,
                updated_at
            FROM users WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let user = sqlx::query_as!(
            crate::model::User,
            r#"
            SELECT
                id AS "id: Uuid",
                name,
                email,
                verified,
                password,
                roles AS "roles: Roles",
                created_at,
                updated_at
            FROM users WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn exists(&self, email: &str) -> RepositoryResult<bool> {
        let exists = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = ?) AS "exists!: bool"
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.exists)
    }
}

impl UserRepository {
    // create
    pub async fn create(&self, user: &RegisterUserSchema) -> RepositoryResult<User> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|_| "Failed to hash password")?;

        let hashed_password = hashed_password.to_string();

        let roles = Roles::from(vec!["ROLE_USER"]);

        let id = Uuid::new_v4();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, name, email, password, roles)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id AS "id: Uuid",
                name,
                email,
                verified,
                password,
                roles AS "roles: Roles",
                created_at,
                updated_at
            "#,
            id,
            user.name,
            user.email,
            hashed_password,
            roles
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}

impl UserRepository {
    // update password
    pub async fn update(&self, id: Uuid, body: &UpdateUserSchema) -> RepositoryResult<User> {
        let mut query = QueryBuilder::new(
            r#"
            UPDATE users
            SET "#,
        );

        query.push("name = ").push_bind(body.name.clone());
        query.push(", email = ").push_bind(body.email.clone());

        if let Some(password) = &body.password {
            let salt = SaltString::generate(&mut OsRng);
            let hashed_password = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|_| "Failed to hash password")?;

            let hashed_password = hashed_password.to_string();

            query.push(", password = ").push_bind(hashed_password);
        }

        query.push(" WHERE id = ").push_bind(id);

        query.push(r#" RETURNING id, name, email, verified, password, roles, created_at, updated_at "#);

        Ok(query.build_query_as::<User>().fetch_one(&self.pool).await?)
    }
}
