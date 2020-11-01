use crate::domain::user::{Error, Result};
use crate::domain_logic::security::hasher::Hasher;

/// Hasker uses Argon 2 algorithm
pub struct ArgonHasher;

impl Hasher for &ArgonHasher {
    fn hash<S: AsRef<[u8]>>(self, src: &str, salt: S) -> Result<Vec<u8>> {
        let config = argon2::Config::default();
        argon2::hash_raw(src.as_bytes(), salt.as_ref(), &config).map_err(Error::from)
    }

    fn verify<H, S>(self, pwd: &str, hash: H, salt: S) -> Result<bool>
    where
        H: AsRef<[u8]>,
        S: AsRef<[u8]>,
    {
        let config = argon2::Config::default();
        argon2::verify_raw(pwd.as_bytes(), salt.as_ref(), hash.as_ref(), &config)
            .map_err(Error::from)
    }
}
