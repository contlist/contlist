use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::{entities::contact::Result, phone_number::PhoneNumber};
use getset::{Getters, MutGetters};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CreateData<'a> {
    contact_name: &'a str,
    phone_number: PhoneNumber<&'a str>,
}

pub trait Creator {
    fn create(&self, username: &str, create_data: CreateData<'_>) -> Result<i64>;
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct CreatorImpl {
    repo: Arc<dyn ContactRepo>,
}

impl CreatorImpl {
    pub fn new(repo: Arc<dyn ContactRepo>) -> Self {
        Self { repo }
    }
}

impl Creator for CreatorImpl {
    fn create(&self, username: &str, create_data: CreateData<'_>) -> Result<i64> {
        self.repo
            .save_new_contact(username, create_data.contact_name, create_data.phone_number)
    }
}
