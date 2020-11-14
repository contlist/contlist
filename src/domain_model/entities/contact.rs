use crate::domain_model::phone_number::PhoneNumber;
use getset::{Getters, MutGetters};
use serde::Serialize;
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Serialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Contact {
    id: i64,
    contact_name: String,
    phone_number: PhoneNumber<String>,
}

impl Contact {
    pub fn new(id: i64, contact_name: String, phone_number: PhoneNumber<String>) -> Self {
        Self {
            id,
            contact_name,
            phone_number,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find contact")]
    NotFound,
    #[error("error occurred while working with repo: {0}")]
    RepoError(Box<dyn StdError + Send + Sync>),
}
