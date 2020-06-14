use super::error::{Error, Result};
use crate::domain::user::CurrentUser;
use boolinator::Boolinator;
use rocket::request::{self, FromRequest};
use rocket::{http::Status, outcome::IntoOutcome, Request};
use std::env;

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = Error;

    /// Extract Claims token from the "Authorization" header.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let secret_key = env::var("JWT_SECRET_KEY").expect("failed to read environment variable"); // TODO: move to config

        extract_claims_from_request(request, secret_key.as_bytes()).into_outcome(Status::Forbidden)
    }
}

fn extract_claims_from_request(request: &Request, secret_key: &[u8]) -> Result<CurrentUser> {
    request
        .headers()
        .get_one("Authorization")
        .and_then(extract_token_from_header)
        .ok_or(Error::MissingTokenError)
        .and_then(|token| CurrentUser::from_token(token).map_err(Error::from))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    let prefix = "Bearer ";
    header.starts_with(prefix).as_some(&header[prefix.len()..])
}
