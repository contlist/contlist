use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::{entities::contact::Result, phone_number::PhoneNumber};
use getset::{Getters, MutGetters};
use rocket_okapi::JsonSchema;
use serde::Deserialize;
use shaku::Provider;

#[derive(Deserialize, Clone, Getters, MutGetters, JsonSchema, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct UpdateData<'a> {
    contact_name: &'a str,
    phone_number: PhoneNumber<&'a str>,
}

pub trait Updater: 'static {
    fn update(&self, username: &str, id: i64, update_data: UpdateData<'_>) -> Result<()>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Updater)]
#[getset(get = "pub")]
pub struct UpdaterImpl {
    #[shaku(provide)]
    repo: Box<dyn ContactRepo>,
}

impl UpdaterImpl {
    pub fn new(repo: Box<dyn ContactRepo>) -> Self {
        Self { repo }
    }
}

impl Updater for UpdaterImpl {
    fn update(&self, username: &str, id: i64, update_data: UpdateData<'_>) -> Result<()> {
        self.repo
            .update_contact_with_username(
                username,
                id,
                update_data.contact_name,
                update_data.phone_number,
            )
            .map(|_| ())
    }
}
