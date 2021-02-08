use crate::domain_model::entities::user::Result;
use shaku::Interface;
use std::fmt::Debug;

/// A service to hashing and verification passwords
pub trait Hasher: Interface + Debug {
    /// hashes 'pwd' with 'salt'
    fn hash(&self, pwd: &str, salt: &[u8]) -> Result<Vec<u8>>;

    /// checks if 'hash' obtained from 'pwd'
    fn verify(&self, pwd: &str, hash: &[u8], salt: &[u8]) -> Result<bool>;
}
