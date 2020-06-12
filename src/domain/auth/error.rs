use jsonwebtoken::errors::Error as JwtError;
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to create token: {0}")]
    CreateTokenError(Box<dyn StdError + Send + Sync>),
    #[error("failed to extract token: {0}")]
    InvalidTokenError(Box<dyn StdError + Send + Sync>),
    #[error("the token has expired")]
    ExpiredTokenError,
}
