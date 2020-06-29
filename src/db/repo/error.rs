use crate::domain::user::Error;
use argon2::Error as ArgError;
use base64::DecodeError as Base64Error;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

impl From<DieselError> for Error {
    fn from(derror: DieselError) -> Self {
        match derror {
            DieselError::DatabaseError(
                DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::ForeignKeyViolation,
                ..,
            ) => Error::AlreadyExistsError,
            e => {
                let error = anyhow::Error::new(e);
                Error::RepoError(error.into())
            }
        }
    }
}

impl From<ArgError> for Error {
    fn from(aerror: ArgError) -> Self {
        let error = anyhow::Error::msg(aerror);
        Error::RepoError(error.into())
    }
}

impl From<Base64Error> for Error {
    fn from(berror: Base64Error) -> Self {
        let error = anyhow::Error::new(berror);
        Error::RepoError(error.into())
    }
}
