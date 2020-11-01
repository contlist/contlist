use crate::domain::user::Result;

/// Hashes 'str' with salt
pub trait Hasher {
    fn hash<S: AsRef<[u8]>>(self, src: &str, salt: S) -> Result<Vec<u8>>;
}