use crate::domain_model::entities::contact::{Contact, Result};
use crate::domain_model::phone_number::PhoneNumber;

pub trait ContactRepo: 'static {
    fn save_new_contact(
        &self,
        username: &str,
        contact_name: &str,
        phone_number: PhoneNumber<&'_ str>,
    ) -> Result<i64>;
    fn find_contact(&self, id: i64) -> Result<Option<Contact>>;
    fn find_contacts_by_username(&self, username: &str) -> Result<Vec<Contact>>;
    fn find_contacts_by_name(&self, username: &str, name: &str) -> Result<Vec<Contact>>;
    fn find_contacts_by_number(
        &self,
        username: &str,
        number: PhoneNumber<&'_ str>,
    ) -> Result<Vec<Contact>>;
    fn update_contact_with_username(
        &self,
        username: &str,
        id: i64,
        contact_name: &str,
        phone_number: PhoneNumber<&'_ str>,
    ) -> Result<usize>;
    fn delete_contact_with_username(&self, username: &str, id: i64) -> Result<usize>;
}
