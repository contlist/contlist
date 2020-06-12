use jsonwebtoken::errors::Error as JwtError;
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to create token: {0}")]
    CreateTokenError(Box<dyn StdError + Send + Sync>),
    #[error("failed to extract token: {0}")]
    ExtaractTokenError(Box<dyn StdError + Send + Sync>),
    #[error("error occurred while working with the token: {0}")]
    OtherTokenError(Box<dyn StdError + Send + Sync>),
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        let error = anyhow::Error::new(error);
        Error::OtherTokenError(error.into())
    }
}
