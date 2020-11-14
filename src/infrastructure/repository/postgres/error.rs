use crate::domain_model::entities::contact::Error as CError;
use crate::domain_model::entities::user::Error as UError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

impl From<DieselError> for UError {
    fn from(derror: DieselError) -> Self {
        match derror {
            DieselError::DatabaseError(
                DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::ForeignKeyViolation,
                ..,
            ) => UError::AlreadyExistsError,
            e => {
                let error = anyhow::Error::new(e);
                UError::RepoError(error.into())
            }
        }
    }
}

impl From<DieselError> for CError {
    fn from(derror: DieselError) -> Self {
        let error = anyhow::Error::new(derror);
        CError::RepoError(error.into())
    }
}
