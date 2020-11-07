use super::error::{Error, Result};
use crate::domain_logic::security::token_handler::TokenHandler;
use crate::domain_model::claims::Claims;
use crate::module::MainModule;
use boolinator::Boolinator;
use getset::Getters;
use rocket::request::{self, FromRequest};
use rocket::{http::Status, outcome::IntoOutcome, Request, State};
use serde::Deserialize;
use shaku::HasComponent;

#[derive(Deserialize, Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct CurrentUser {
    username: String,
}

impl CurrentUser {
    fn new(username: String) -> Self {
        Self { username }
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
            .and_then(|token| current_user_from_token(token, request))
            .into_outcome(Status::Forbidden)
    }
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    let prefix = "Bearer ";
    header.starts_with(prefix).as_some(&header[prefix.len()..])
}

fn current_user_from_token(token: &str, request: &Request<'_>) -> Result<CurrentUser> {
    let module = request
        .guard::<State<Box<MainModule>>>()
        .expect("failed to retrive module");

    let token_handler: &dyn TokenHandler<Claims = Claims> = module.resolve_ref();

    token_handler
        .extract_claims(token)
        .map_err(Error::from)
        .map(|claims| CurrentUser::new(claims.username().clone()))
}
