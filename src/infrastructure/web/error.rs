use crate::domain_model::entities::{contact::Error as CError, user::Error as UError};
use rocket::response::{Responder, Response, Result as ResponseResult};
use rocket::{http::Status, Request};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find token")]
    MissingTokenError,
    #[error(transparent)]
    UserError(#[from] UError),
    #[error(transparent)]
    ContactError(#[from] CError),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _request: &Request) -> ResponseResult<'r> {
        let mut builder = Response::build();
        let response = match self {
            Error::UserError(UError::NotFound) | Error::ContactError(CError::NotFound) => {
                builder.status(Status::NotFound)
            }
            Error::MissingTokenError | Error::UserError(UError::ExpiredTokenError) => builder
                .status(Status::Unauthorized)
                .raw_header("WWW-Authenticate", "Bearer"),
            Error::UserError(UError::AlreadyExistsError)
            | Error::UserError(UError::InvalidCredentials) => builder.status(Status::BadRequest),
            Error::UserError(UError::TokenError(_)) => builder.status(Status::Forbidden),
            Error::UserError(_) | Error::ContactError(_) => {
                builder.status(Status::InternalServerError)
            }
        }
        .finalize();

        Ok(response)
    }
}
