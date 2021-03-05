use super::salt;
use crate::domain_logic::repository::UserRepo;
use crate::domain_logic::security::hasher::Hasher;
use crate::domain_model::entities::user::Result;
use getset::{Getters, MutGetters};
use rocket_okapi::JsonSchema;
use serde::Deserialize;
use shaku::Provider;
use std::sync::Arc;

#[derive(Deserialize, Clone, Getters, MutGetters, JsonSchema, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct RegisterData<'a> {
    username: &'a str,
    password: &'a str,
}

pub trait Registar: 'static {
    fn register(&self, register_data: RegisterData<'_>) -> Result<()>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Registar)]
#[getset(get = "pub")]
pub struct RegistarImpl {
    #[shaku(provide)]
    repo: Box<dyn UserRepo>,
    #[shaku(inject)]
    hasher: Arc<dyn Hasher>,
}

impl RegistarImpl {
    pub fn new(repo: Box<dyn UserRepo>, hasher: Arc<dyn Hasher>) -> Self {
        Self { repo, hasher }
    }
}

impl Registar for RegistarImpl {
    fn register(&self, register_data: RegisterData<'_>) -> Result<()> {
        let mut rng = rand::thread_rng();
        let salt = salt::generate(&mut rng);
        let hash = self.hasher.hash(register_data.password, &salt[..])?;

        let hash = base64::encode(hash);
        let salt = base64::encode(salt);
        self.repo
            .save_new_user(register_data.username, hash.as_str(), salt.as_str())
            .map(|_| ())
    }
}
