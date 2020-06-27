use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find contact")]
    NotFound,
    #[error("contact with same name already exists")]
    AlreadyExists,
    #[error("error occurred while working with repo: {0}")]
    RepoError(Box<dyn StdError + Send + Sync>),
}
