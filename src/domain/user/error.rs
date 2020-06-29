use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find user")]
    NotFound,
    #[error("user with same login already exists")]
    AlreadyExistsError,
    #[error("invalid user credentials")]
    InvalidCredentials,
    #[error("the token has expired")]
    ExpiredTokenError,
    #[error("error occured while working with token: {0}")]
    TokenError(Box<dyn StdError + Send + Sync>),
    #[error("error occurred while working with repo: {0}")]
    RepoError(Box<dyn StdError + Send + Sync>),
}
