use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::{Contact, Result};
use getset::Getters;
use std::sync::Arc;

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct List {
    repo: Arc<dyn ContactRepo>,
}

impl List {
    pub fn new(repo: Arc<dyn ContactRepo>) -> Self {
        Self { repo }
    }

    pub fn handle(&self, username: &str) -> Result<Vec<Contact>> {
        self.repo.find_contacts_by_username(username)
    }
}
