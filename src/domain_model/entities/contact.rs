use crate::domain_model::phone_number::PhoneNumber;
use getset::MutGetters;
use serde::Serialize;
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Serialize, Clone, MutGetters, Debug)]
#[getset(get_mut = "pub")]
pub struct Contact {
    id: i64,
    contact_name: String,
    phone_number: PhoneNumber<String>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find contact")]
    #[allow(dead_code)]
    NotFound,
    #[error("error occurred while working with repo: {0}")]
    RepoError(Box<dyn StdError + Send + Sync>),
}
