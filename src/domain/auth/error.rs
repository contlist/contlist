use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error occurs when dealing with token: {0}")]
    TokenError(Box<dyn StdError + Send + Sync>),
    #[error("the token has expired")]
    ExpiredTokenError,
}

impl From<JwtError> for Error {
    fn from(src: JwtError) -> Self {
        match src.kind() {
            JwtErrorKind::ExpiredSignature => Error::ExpiredTokenError,
            _ => Error::TokenError(Box::new(src).into()),
        }
    }
}
