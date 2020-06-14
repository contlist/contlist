use crate::db::Error as RepoError;
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find user")]
    NotFound,
    #[error("user with same login already exists")]
    AlreadyExitsError,
    #[error("invalid user credentials: {0}")]
    InvalidCredentials(Box<dyn StdError + Send + Sync>),
    #[error("the token has expired")]
    ExpiredTokenError,
    #[error("error occured while working with token: {0}")]
    TokenError(Box<dyn StdError + Send + Sync>),
    #[error("error occurred while working with repo: {0}")]
    RepoError(Box<dyn StdError + Send + Sync>),
}

impl From<RepoError> for Error {
    fn from(src: RepoError) -> Self {
        match src {
            RepoError::UnexpectedDuplicateError => Error::AlreadyExitsError,
            e => Error::RepoError(Box::new(e).into()),
        }
    }
}

impl From<JwtError> for Error {
    fn from(src: JwtError) -> Self {
        match src.kind() {
            JwtErrorKind::ExpiredSignature => Error::ExpiredTokenError,
            _ => Error::TokenError(Box::new(src).into()),
        }
    }
}
