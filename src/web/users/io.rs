use crate::infrastructure::repository::postgres::{Pool, UserPgRepo};
use crate::domain::user::{CurrentUser, Error as UError};
use crate::web::error::{Error, Result};
use boolinator::Boolinator;
use rocket::request::{self, FromRequest};
use rocket::{http::Status, outcome::IntoOutcome, Request, State};

impl FromRequest<'_, '_> for UserPgRepo {
    type Error = Error;

    fn from_request(request: &Request<'_>) -> request::Outcome<Self, Self::Error> {
        request
            .guard::<State<Pool>>()
            .unwrap()
            .get()
            .map_err(|e| Box::new(e).into())
            .map_err(UError::RepoError)
            .map_err(Error::UserError)
            .into_outcome(Status::InternalServerError)
            .map(UserPgRepo::new)
    }
}

impl FromRequest<'_, '_> for CurrentUser {
    type Error = Error;

    /// Extract Claims token from the "Authorization" header.
    fn from_request(request: &Request<'_>) -> request::Outcome<Self, Self::Error> {
        request
            .headers()
            .get_one("Authorization")
            .and_then(extract_token_from_header)
            .ok_or(Error::MissingTokenError)
            .and_then(current_user_from_token)
            .into_outcome(Status::Forbidden)
    }
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    let prefix = "Bearer ";
    header.starts_with(prefix).as_some(&header[prefix.len()..])
}

fn current_user_from_token(token: &str) -> Result<CurrentUser> {
    CurrentUser::from_token(token).map_err(Error::from)
}
