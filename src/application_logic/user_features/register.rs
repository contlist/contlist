use super::salt;
use crate::domain_logic::repository::UserRepo;
use crate::domain_logic::security::hasher::Hasher;
use crate::domain_model::entities::user::Result;
use getset::{Getters, MutGetters};
use serde::Deserialize;

#[derive(Deserialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct RegisterData<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct Regisetr<R, H> {
    repo: R,
    hasher: H,
}

impl<R, H> Regisetr<R, H>
where
    R: UserRepo,
    H: Hasher,
{
    pub fn new(repo: R, hasher: H) -> Self {
        Self { repo, hasher }
    }

    pub fn handle(self, register_data: RegisterData<'_>) -> Result<()> {
        let mut rng = rand::thread_rng();
        let salt = salt::generate(&mut rng);
        let hash = self.hasher.hash(register_data.password, salt)?;

        let hash = base64::encode(hash);
        let salt = base64::encode(salt);
        self.repo
            .save_new_user(register_data.username, hash.as_str(), salt.as_str())
            .map(|_| ())
    }
}
