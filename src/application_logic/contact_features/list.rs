use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::{Contact, Result};
use getset::Getters;

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct List<R> {
    repo: R,
}

impl<R> List<R>
where
    R: ContactRepo,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn handle(&self, username: &str) -> Result<Vec<Contact>> {
        self.repo.find_contacts_by_username(username)
    }
}
