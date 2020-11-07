use crate::domain_model::entities::user::Error;
use base64::DecodeError as Base64Error;

impl From<Base64Error> for Error {
    fn from(berror: Base64Error) -> Self {
        let error = anyhow::Error::new(berror);
        Error::RepoError(error.into())
    }
}
