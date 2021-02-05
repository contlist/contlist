use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::{entities::contact::Result, phone_number::PhoneNumber};
use getset::{Getters, MutGetters};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct UpdateData<'a> {
    contact_name: &'a str,
    phone_number: PhoneNumber<&'a str>,
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct Update {
    repo: Arc<dyn ContactRepo>,
}

impl Update {
    pub fn new(repo: Arc<dyn ContactRepo>) -> Self {
        Self { repo }
    }

    pub fn handle(&self, username: &str, id: i64, update_data: UpdateData<'_>) -> Result<()> {
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
