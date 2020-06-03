use diesel::result::{DatabaseErrorKind, Error as DieselError};
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unespected duplicate")]
    UnexpectedDuplicateError,
    #[error("failed to prepare data: {0}")]
    DataPrepareError(Box<dyn StdError + Send + Sync>),
    #[error("error occurred in internal storage: {0}")]
    InternalStorageError(#[from] Box<dyn StdError + Send + Sync>),
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(
                DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::ForeignKeyViolation,
                ..,
            ) => Error::UnexpectedDuplicateError,
            DieselError::DatabaseError(_, info) => {
                let msg = info.message().to_owned();
                let error = anyhow::Error::msg(msg);
                Error::InternalStorageError(error.into())
            }
            e => Error::DataPrepareError(anyhow::Error::msg(e).into()),
        }
    }
}
