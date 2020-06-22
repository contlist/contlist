use crate::domain::user::Error as DomainError;
use rocket::response::{Responder, Response, Result as ResponseResult};
use rocket::{http::Status, Request};
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find user")]
    NotFound,
    #[error("failed to find token")]
    MissingTokenError,
    #[error("the token has expired")]
    ExpiredTokenError,
    #[error("invalid token: {0}")]
    InvalidTokenError(Box<dyn StdError + Send + Sync>),
    #[error(transparent)]
    RepoError(Box<dyn StdError + Send + Sync>),
    #[error("error occured while working whith user: {0}")]
    UserError(Box<dyn StdError + Send + Sync>),
}

impl From<DomainError> for Error {
    fn from(src: DomainError) -> Self {
        match &src {
            DomainError::NotFound => Error::NotFound,
            DomainError::TokenError(_) => Error::InvalidTokenError(Box::new(src).into()),
            DomainError::ExpiredTokenError => Error::ExpiredTokenError,
            DomainError::RepoError(_) => Error::RepoError(Box::new(src).into()),
            _ => Error::UserError(Box::new(src).into()),
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _request: &Request) -> ResponseResult<'r> {
        let mut builder = Response::build();
        let response = match self {
            Error::NotFound => builder.status(Status::NotFound),
            Error::MissingTokenError | Error::ExpiredTokenError => builder
                .status(Status::Unauthorized)
                .raw_header("WWW-Authenticate", "Bearer"),
            Error::InvalidTokenError(_) => builder.status(Status::Forbidden),
            Error::RepoError(_) => builder.status(Status::InternalServerError),
            Error::UserError(_) => builder.status(Status::BadRequest),
        }
        .finalize();

        Ok(response)
    }
}
