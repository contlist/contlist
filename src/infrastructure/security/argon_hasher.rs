use crate::domain_logic::security::hasher::Hasher;
use crate::domain_model::entities::user::{Error, Result};
use argon2::Error as ArgonError;
use shaku::Component;

/// Hasker uses Argon 2 algorithm
#[derive(Component, Debug)]
#[shaku(interface = Hasher)]
pub struct ArgonHasher;

impl Hasher for ArgonHasher {
    fn hash(&self, src: &str, salt: &[u8]) -> Result<Vec<u8>> {
        let config = argon2::Config::default();
        argon2::hash_raw(src.as_bytes(), salt.as_ref(), &config).map_err(Error::from)
    }

    fn verify(&self, pwd: &str, hash: &[u8], salt: &[u8]) -> Result<bool> {
        let config = argon2::Config::default();
        argon2::verify_raw(pwd.as_bytes(), salt.as_ref(), hash.as_ref(), &config)
            .map_err(Error::from)
    }
}

impl From<ArgonError> for Error {
    fn from(aerror: ArgonError) -> Self {
        let error = anyhow::Error::msg(aerror);
        Error::RepoError(error.into())
    }
}
