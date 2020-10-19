use crate::domain::security::hasher::Hasher;
use crate::domain::user::{Error, Result};

/// Hasker uses Argon 2 algorithm
pub struct ArgonHasher;

impl Hasher for ArgonHasher {
    fn hash<S: AsRef<[u8]>>(src: &str, salt: S) -> Result<Vec<u8>> {
        let config = argon2::Config::default();
        argon2::hash_raw(src.as_bytes(), salt.as_ref(), &config)
            .map_err(Error::from)
    } 
}
