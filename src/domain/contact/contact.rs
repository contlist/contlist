use super::Result;
use crate::domain::{phone_number::PhoneNumber, user::CurrentUser};
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

pub fn create_contact(
    user: &CurrentUser,
    create_contact: CreateContact<'_>,
    repo: &impl ContactRepo,
) -> Result<()> {
    repo.save_new_contact(user.username.as_str(), create_contact)
        .map(|_| ())
}

pub fn get_contacts(user: &CurrentUser, repo: &impl ContactRepo) -> Result<Vec<Contact>> {
    repo.find_contact_by_username(user.username.as_str())
}

pub fn update_contact(
    user: &CurrentUser,
    id: i64,
    update_contact: UpdateContact<'_>,
    repo: &impl ContactRepo,
) -> Result<()> {
    repo.update_contact_with_username(user.username.as_str(), id, update_contact)
        .map(|_| ())
}

pub fn delete_contact(user: &CurrentUser, id: i64, repo: &impl ContactRepo) -> Result<()> {
    repo.delete_contact_with_username(user.username.as_str(), id)
        .map(|_| ())
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
    fn update_contact_with_username(
        &self,
        username: &str,
        id: i64,
        contact: UpdateContact<'_>,
    ) -> Result<usize>;
    fn delete_contact(&self, id: i64) -> Result<usize>;
    fn delete_contact_with_username(&self, username: &str, id: i64) -> Result<usize>;
}
