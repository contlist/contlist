use super::auth::{error::Result as AuthResult, token::Claims};
use crate::db::repo::error::Error as DbError;
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct AuthUser<'a> {
    pub username: &'a str,
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CurrentUser {
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct RegisterUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Clone, Debug)]
pub struct LoginUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateUser<'a> {
    pub password: &'a str,
}

impl User {
    pub fn as_token(&self) -> AuthResult<String> {
        let duration = Duration::minutes(15); // TODO: move to config
        let claims = Claims::new(self.username.clone(), duration);
        claims.as_token()
    }
}

impl CurrentUser {
    pub fn from_token(token: &str) -> AuthResult<Self> {
        let claims = Claims::from_token(token)?;
        let current_user = Self {
            username: claims.username,
        };

        Ok(current_user)
    }
}

pub trait UserRepo {
    fn register_user(&self, user: &RegisterUser) -> Result<usize, DbError>;
    fn find_user_by_username(&self, username: &str) -> Result<Option<User>, DbError>;
    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>, DbError>;
    fn update_user(&self, username: &str, user: &UpdateUser) -> Result<usize, DbError>;
}
