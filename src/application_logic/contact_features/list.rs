use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::{Contact, Result};
use getset::Getters;
use std::sync::Arc;

pub trait Lister {
    fn list(&self, username: &str) -> Result<Vec<Contact>>;
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct ListerImpl {
    repo: Arc<dyn ContactRepo>,
}

impl ListerImpl {
    pub fn new(repo: Arc<dyn ContactRepo>) -> Self {
        Self { repo }
    }
}

impl Lister for ListerImpl {
    fn list(&self, username: &str) -> Result<Vec<Contact>> {
        self.repo.find_contacts_by_username(username)
    }
}
