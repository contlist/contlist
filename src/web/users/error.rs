use crate::domain::user::Error as DomainError;
use rocket::response::{Responder, Response, Result as ResponseResult};
use rocket::{http::Status, Request};
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find token")]
    MissingTokenError,
    #[error("the token has expired")]
    ExpiredTokenError,
    #[error("invalid token: {0}")]
    InvalidTokenError(Box<dyn StdError + Send + Sync>),
}

impl From<DomainError> for Error {
    fn from(src: DomainError) -> Self {
        match &src {
            DomainError::TokenError(e) => Error::InvalidTokenError(Box::new(src).into()),
            DomainError::ExpiredTokenError => Error::ExpiredTokenError,
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, request: &Request) -> ResponseResult<'r> {
        let status = match self {
            Error::MissingTokenError | Error::ExpiredTokenError => Status::Unauthorized,
            Error::InvalidTokenError(_) => Status::Forbidden,
        };

        let response = Response::build()
            .status(status)
            .raw_header("WWW-Authenticate", "Bearer")
            .finalize();

        Ok(response)
    }
}
