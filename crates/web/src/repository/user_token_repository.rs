use std::net::IpAddr;

use sqlx::SqlitePool;
use uuid::Uuid;

use crate::model::*;

use super::RepositoryResult;

#[derive(Clone)]
pub struct UserTokenRepository {
    pool: SqlitePool,
}

impl UserTokenRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl UserTokenRepository {
    pub async fn get_by_token(&self, token: uuid::Uuid) -> RepositoryResult<Option<UserToken>> {
        let token = sqlx::query_as!(
            UserToken,
            r#"
            UPDATE user_tokens SET last_used = CURRENT_TIMESTAMP WHERE token = ?
            RETURNING 
                token AS "token: Uuid",
                user_id AS "user_id: Uuid",
                ip,
                created_at,
                last_used
            "#,
            token,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }
}

impl UserTokenRepository {
    pub async fn all_by_user_id(&self, user_id: uuid::Uuid) -> RepositoryResult<Vec<UserToken>> {
        let tokens = sqlx::query_as!(
            UserToken,
            r#"
            SELECT
                token AS "token: Uuid",
                user_id AS "user_id: Uuid",
                ip,
                created_at,
                last_used
            FROM user_tokens WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }
}

impl UserTokenRepository {
    pub async fn create(&self, user_id: Uuid, ip: IpAddr) -> RepositoryResult<UserToken> {
        let id = uuid::Uuid::new_v4();
        let ip = ip.to_string();
        let token = sqlx::query_as!(
            UserToken,
            r#"
            INSERT INTO user_tokens (token, user_id, ip) VALUES (?, ?, ?)
            RETURNING 
                token AS "token: Uuid",
                user_id AS "user_id: Uuid",
                ip,
                created_at,
                last_used
            "#,
            id,
            user_id,
            ip
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }
}

impl UserTokenRepository {
    pub async fn delete(&self, user_id: Uuid, token: UserToken) -> RepositoryResult<()> {
        let token = token.token;
        sqlx::query!(
            r#"
            DELETE FROM user_tokens WHERE token = ? AND user_id = ?
            "#,
            token,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
