use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::{entities::contact::Result, phone_number::PhoneNumber};
use getset::{Getters, MutGetters};
use serde::Deserialize;

#[derive(Deserialize, Clone, Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct UpdateData<'a> {
    contact_name: &'a str,
    phone_number: PhoneNumber<&'a str>,
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct Update<R> {
    repo: R,
}

impl<R> Update<R>
where
    R: ContactRepo,
{
    pub fn new(repo: R) -> Self {
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
