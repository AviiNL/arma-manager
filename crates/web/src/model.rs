use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use sqlx::{
    database::HasValueRef, encode::IsNull, error::BoxDynError, sqlite::SqliteArgumentValue, Decode, Encode, Sqlite,
    Type,
};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Roles,
    pub verified: bool,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UserToken {
    pub token: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub ip: String,
    pub created_at: Option<time::OffsetDateTime>,
    pub last_used: Option<time::OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roles(Vec<String>);

impl Deref for Roles {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Roles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<String>> for Roles {
    fn from(roles: Vec<String>) -> Self {
        Self(roles)
    }
}

impl From<Roles> for Vec<String> {
    fn from(roles: Roles) -> Self {
        roles.0
    }
}

impl From<Vec<&str>> for Roles {
    fn from(roles: Vec<&str>) -> Self {
        Self(roles.iter().map(|s| s.to_string()).collect())
    }
}

impl Type<Sqlite> for Roles {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }
}

impl<'r> Decode<'r, Sqlite> for Roles {
    fn decode(value: <Sqlite as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<Sqlite>>::decode(value)?;

        Ok(Self(value.split(',').map(|s| s.to_string()).collect()))
    }
}

// impl Encode
impl<'q> Encode<'q, Sqlite> for Roles {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        let s = self.0.join(",");
        args.push(SqliteArgumentValue::Text(Cow::Owned(s.clone())));

        IsNull::No
    }
}
