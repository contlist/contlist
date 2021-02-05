use super::salt;
use crate::domain_logic::repository::UserRepo;
use crate::domain_logic::security::hasher::Hasher;
use crate::domain_model::entities::user::Result;
use getset::{Getters, MutGetters};
use serde::Deserialize;

#[derive(Deserialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct UpdateData<'a> {
    password: &'a str,
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct Update<R, H> {
    repo: R,
    hasher: H,
}

impl<R, H> Update<R, H>
where
    R: UserRepo,
    H: Hasher + 'static,
{
    pub fn new(repo: R, hasher: H) -> Self {
        Self { repo, hasher }
    }

    pub fn handle(&self, username: &str, update_data: UpdateData<'_>) -> Result<()> {
        let mut rng = rand::thread_rng();
        let salt = salt::generate(&mut rng);
        let hash = self.hasher.hash(update_data.password, salt)?;

        let hash = base64::encode(hash);
        let salt = base64::encode(salt);
        self.repo
            .update_user(username, hash.as_str(), salt.as_str())
            .map(|_| ())
    }
}
