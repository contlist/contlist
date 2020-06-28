use super::Result;
use crate::domain::phone_number::PhoneNumber;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct Contact {
    pub id: i64,
    pub contact_name: String,
    pub phone_number: PhoneNumber<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateContact<'a> {
    pub contact_name: &'a str,
    pub phone_number: PhoneNumber<&'a str>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateContact<'a> {
    pub contact_name: &'a str,
    pub phone_number: PhoneNumber<&'a str>,
}

pub trait ContactRepo {
    fn save_new_contact(&self, username: &str, contact: CreateContact<'_>) -> Result<usize>;
    fn find_contact(&self, id: i64) -> Result<Option<Contact>>;
    fn find_contact_by_username(&self, username: &str) -> Result<Vec<Contact>>;
    fn find_contacts_by_name(&self, username: &str, name: &str) -> Result<Vec<Contact>>;
    fn find_contacts_by_number(
        &self,
        username: &str,
        number: PhoneNumber<&'_ str>,
    ) -> Result<Vec<Contact>>;
    fn update_contact(&self, id: i64, contact: UpdateContact<'_>) -> Result<usize>;
    fn delete_contact(&self, id: i64) -> Result<usize>;
}
