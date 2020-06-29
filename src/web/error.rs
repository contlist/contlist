use crate::domain::user::Error as UError;
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
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _request: &Request) -> ResponseResult<'r> {
        let mut builder = Response::build();
        let response = match self {
            Error::UserError(UError::NotFound) => builder.status(Status::NotFound),
            Error::MissingTokenError | Error::UserError(UError::ExpiredTokenError) => builder
                .status(Status::Unauthorized)
                .raw_header("WWW-Authenticate", "Bearer"),
            Error::UserError(UError::TokenError(_)) => builder.status(Status::Forbidden),
            Error::UserError(UError::RepoError(_)) => builder.status(Status::InternalServerError),
            Error::UserError(_) => builder.status(Status::BadRequest),
        }
        .finalize();

        Ok(response)
    }
}
