use rand::{CryptoRng, Rng};

pub type Salt = [u8; 32];

pub fn generate<R: Rng + CryptoRng>(rng: &mut R) -> Salt {
    let mut salt = Salt::default();
    rng.fill(&mut salt);
    salt
}
