use argon2::Error as ArgError;
use base64::DecodeError as Base64Error;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unespected duplicate")]
    UnexpectedDuplicateError,
    #[error("failed to prepare data: {0}")]
    DataPrepareError(Box<dyn StdError + Send + Sync>),
    #[error("error occurred in internal storage: {0}")]
    InternalStorageError(Box<dyn StdError + Send + Sync>),
}

impl From<DieselError> for Error {
    fn from(src: DieselError) -> Self {
        match src {
            DieselError::DatabaseError(
                DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::ForeignKeyViolation,
                ..,
            ) => Error::UnexpectedDuplicateError,
            DieselError::DatabaseError(_, info) => {
                let msg = info.message().to_owned();
                let error = anyhow::Error::msg(msg);
                Error::InternalStorageError(error.into())
            }
            e => Error::DataPrepareError(Box::new(e).into()),
        }
    }
}

impl From<ArgError> for Error {
    fn from(src: ArgError) -> Self {
        let error = anyhow::Error::msg(src);
        Error::DataPrepareError(error.into())
    }
}

impl From<Base64Error> for Error {
    fn from(src: Base64Error) -> Self {
        let error = anyhow::Error::new(src);
        Error::DataPrepareError(error.into())
    }
}
