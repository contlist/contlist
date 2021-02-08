use super::salt;
use crate::domain_logic::repository::UserRepo;
use crate::domain_logic::security::hasher::Hasher;
use crate::domain_model::entities::user::Result;
use getset::{Getters, MutGetters};
use serde::Deserialize;
use shaku::Provider;
use std::sync::Arc;

#[derive(Deserialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct UpdateData<'a> {
    password: &'a str,
}

pub trait Updater: 'static {
    fn update(&self, username: &str, update_data: UpdateData<'_>) -> Result<()>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Updater)]
#[getset(get = "pub")]
pub struct UpdaterImpl {
    #[shaku(provide)]
    repo: Box<dyn UserRepo>,
    #[shaku(inject)]
    hasher: Arc<dyn Hasher>,
}

impl UpdaterImpl {
    pub fn new(repo: Box<dyn UserRepo>, hasher: Arc<dyn Hasher>) -> Self {
        Self { repo, hasher }
    }
}

impl Updater for UpdaterImpl {
    fn update(&self, username: &str, update_data: UpdateData<'_>) -> Result<()> {
        let mut rng = rand::thread_rng();
        let salt = salt::generate(&mut rng);
        let hash = self.hasher.hash(update_data.password, &salt[..])?;

        let hash = base64::encode(hash);
        let salt = base64::encode(salt);
        self.repo
            .update_user(username, hash.as_str(), salt.as_str())
            .map(|_| ())
    }
}
