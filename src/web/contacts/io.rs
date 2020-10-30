use crate::domain::contact::Error as CError;
use crate::infrastructure::repository::postgres::{ContactPgRepo, Pool};
use crate::web::error::Error;
use rocket::request::{self, FromRequest};
use rocket::{http::Status, outcome::IntoOutcome, Request, State};

impl FromRequest<'_, '_> for ContactPgRepo {
    type Error = Error;

    fn from_request(request: &Request<'_>) -> request::Outcome<Self, Self::Error> {
        request
            .guard::<State<Pool>>()
            .unwrap()
            .get()
            .map_err(|e| Box::new(e).into())
            .map_err(CError::RepoError)
            .map_err(Error::ContactError)
            .into_outcome(Status::InternalServerError)
            .map(ContactPgRepo::new)
    }
}
