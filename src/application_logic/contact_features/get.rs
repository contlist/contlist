use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::{Contact, Result};
use crate::domain_model::phone_number::PhoneNumber;
use getset::Getters;
use shaku::Provider;

pub trait Getter: 'static {
    fn get_by_name(&self, username: &str, name: &str) -> Result<Vec<Contact>>;
    fn get_by_number(&self, username: &str, number: PhoneNumber<&str>) -> Result<Vec<Contact>>;
    fn list(&self, username: &str) -> Result<Vec<Contact>>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Getter)]
#[getset(get = "pub")]
pub struct GetterImpl {
    #[shaku(provide)]
    repo: Box<dyn ContactRepo>,
}

impl GetterImpl {
    pub fn new(repo: Box<dyn ContactRepo>) -> Self {
        Self { repo }
    }
}

impl Getter for GetterImpl {
    fn get_by_name(&self, username: &str, name: &str) -> Result<Vec<Contact>> {
        self.repo.find_contacts_by_name(username, name)
    }

    fn get_by_number(&self, username: &str, number: PhoneNumber<&str>) -> Result<Vec<Contact>> {
        self.repo.find_contacts_by_number(username, number)
    }

    fn list(&self, username: &str) -> Result<Vec<Contact>> {
        self.repo.find_contacts_by_username(username)
    }
}
