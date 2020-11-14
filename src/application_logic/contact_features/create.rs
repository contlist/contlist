use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::{entities::contact::Result, phone_number::PhoneNumber};
use getset::{Getters, MutGetters};
use serde::Deserialize;

#[derive(Deserialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct CreateData<'a> {
    contact_name: &'a str,
    phone_number: PhoneNumber<&'a str>,
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct Create<R> {
    repo: R,
}

impl<R> Create<R>
where
    R: ContactRepo,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn handle(&self, username: &str, create_data: CreateData<'_>) -> Result<i64> {
        self.repo
            .save_new_contact(username, create_data.contact_name, create_data.phone_number)
    }
}