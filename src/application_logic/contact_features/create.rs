use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::{entities::contact::Result, phone_number::PhoneNumber};
use getset::{Getters, MutGetters};
use rocket_okapi::JsonSchema;
use serde::Deserialize;
use shaku::Provider;

#[derive(Deserialize, Clone, Getters, MutGetters, JsonSchema, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CreateData<'a> {
    contact_name: &'a str,
    phone_number: PhoneNumber<&'a str>,
}

pub trait Creator: 'static {
    fn create(&self, username: &str, create_data: CreateData<'_>) -> Result<i64>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Creator)]
#[getset(get = "pub")]
pub struct CreatorImpl {
    #[shaku(provide)]
    repo: Box<dyn ContactRepo>,
}

impl CreatorImpl {
    pub fn new(repo: Box<dyn ContactRepo>) -> Self {
        Self { repo }
    }
}

impl Creator for CreatorImpl {
    fn create(&self, username: &str, create_data: CreateData<'_>) -> Result<i64> {
        self.repo
            .save_new_contact(username, create_data.contact_name, create_data.phone_number)
    }
}
