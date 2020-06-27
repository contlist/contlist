use super::Result;
use crate::domain::phone_number::PhoneNumber;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct Contact {
    pub id: u64,
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
    fn save_new_contct(&self, contact: &CreateContact<'_>) -> Result<usize>;
    fn find_contact(&self, id: u64) -> Result<Option<Contact>>;
    fn find_contacts_by_name(&self, name: &str) -> Result<Vec<Contact>>;
    fn find_contacts_by_number(&self, number: &PhoneNumber<&'_ str>) -> Result<Vec<Contact>>;
    fn update_contact(&self, username: &str, contact: &UpdateContact<'_>) -> Result<usize>;
    fn delete_contact(&self, id: u64) -> Result<usize>;
}
