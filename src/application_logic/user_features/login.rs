use crate::domain_logic::repository::UserRepo;
use crate::domain_logic::security::{hasher::Hasher, token_handler::TokenHandler};
use crate::domain_model::claims::Claims;
use crate::domain_model::entities::user::{Error, Result};
use boolinator::Boolinator;
use chrono::Duration;
use getset::{Getters, MutGetters};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use shaku::Provider;
use std::sync::Arc;

#[derive(Deserialize, Clone, Getters, MutGetters, JsonSchema, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct LoginData<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize, Clone, Getters, MutGetters, JsonSchema, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct AuthData {
    username: String,
    token: String,
}

pub trait Loginer: 'static {
    fn login(&self, login_data: LoginData<'_>) -> Result<AuthData>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Loginer)]
#[getset(get = "pub")]
pub struct LoginerImpl {
    #[shaku(provide)]
    repo: Box<dyn UserRepo>,
    #[shaku(inject)]
    hasher: Arc<dyn Hasher>,
    #[shaku(inject)]
    token_handler: Arc<dyn TokenHandler<Claims = Claims>>,
}

impl LoginerImpl {
    pub fn new(
        repo: Box<dyn UserRepo>,
        hasher: Arc<dyn Hasher>,
        token_handler: Arc<dyn TokenHandler<Claims = Claims>>,
    ) -> Self {
        Self {
            repo,
            hasher,
            token_handler,
        }
    }
}

impl Loginer for LoginerImpl {
    fn login(&self, login_data: LoginData<'_>) -> Result<AuthData> {
        let user = self
            .repo
            .find_user_by_username(login_data.username)?
            .ok_or(Error::InvalidCredentials)?;

        let hash = base64::decode(user.password_hash())?;
        let salt = base64::decode(user.password_salt())?;
        self.hasher
            .verify(login_data.password, hash.as_slice(), salt.as_slice())?
            .as_result((), Error::InvalidCredentials)?;

        let duration = Duration::minutes(15); // TODO: move to config
        let claims = Claims::new(user.username().clone(), duration);
        self.token_handler
            .generate_token(claims)
            .map(|token| AuthData {
                username: user.username().clone(),
                token,
            })
    }
}
