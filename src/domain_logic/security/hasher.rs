use crate::domain::user::Result;

/// A service to hashing and verification passwords
pub trait Hasher {
    /// hashes 'pwd' with 'salt'
    fn hash<S: AsRef<[u8]>>(self, pwd: &str, salt: S) -> Result<Vec<u8>>;

    /// checks if 'hash' obtained from 'pwd'
    fn verify<H, S>(self, pwd: &str, hash: H, salt: S) -> Result<bool>
    where
        H: AsRef<[u8]>,
        S: AsRef<[u8]>;
}
