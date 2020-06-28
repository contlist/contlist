use super::{auth::Claims, Error, Result};
use crate::db::Result as RepoResult;
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AuthUser {
    pub username: String,
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CurrentUser {
    pub username: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RegisterUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LoginUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateUser<'a> {
    pub password: &'a str,
}

impl User {
    pub fn as_token(&self) -> Result<String> {
        let duration = Duration::minutes(15); // TODO: move to config
        let claims = Claims::new(self.username.clone(), duration);
        claims.as_token()
    }
}

impl CurrentUser {
    pub fn from_token(token: &str) -> Result<Self> {
        let claims = Claims::from_token(token)?;
        let current_user = Self {
            username: claims.username,
        };

        Ok(current_user)
    }
}

pub fn register_user(register_user: RegisterUser<'_>, repo: &impl UserRepo) -> Result<()> {
    repo.save_new_user(register_user)
        .map_err(Error::from)
        .map(|_| ())
}

pub fn get_user(username: &str, repo: &impl UserRepo) -> Result<User> {
    repo.find_user_by_username(username)?.ok_or(Error::NotFound)
}

pub fn login_user(login_user: LoginUser<'_>, repo: &impl UserRepo) -> Result<AuthUser> {
    let user = repo
        .find_user_by_credentials(login_user)?
        .ok_or(Error::InvalidCredentials(
            anyhow::Error::msg("invalid login or password").into(),
        ))?;

    let auth_user = AuthUser {
        token: user.as_token()?,
        username: user.username,
    };

    Ok(auth_user)
}

pub fn update_user(
    username: &str, // FIXMI: CurrentUser instead
    update_user: UpdateUser<'_>,
    repo: &impl UserRepo,
) -> Result<()> {
    repo.update_user(username, update_user)
        .map_err(Error::from)
        .map(|_| ())
}

pub trait UserRepo {
    fn save_new_user(&self, user: RegisterUser<'_>) -> RepoResult<usize>;
    fn find_user_by_username(&self, username: &str) -> RepoResult<Option<User>>;
    fn find_user_by_credentials(&self, credentials: LoginUser<'_>) -> RepoResult<Option<User>>;
    fn update_user(&self, username: &str, user: UpdateUser<'_>) -> RepoResult<usize>;
}
