use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::{Contact, Result};
use getset::Getters;
use shaku::Provider;

pub trait Lister: 'static {
    fn list(&self, username: &str) -> Result<Vec<Contact>>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Lister)]
#[getset(get = "pub")]
pub struct ListerImpl {
    #[shaku(provide)]
    repo: Box<dyn ContactRepo>,
}

impl ListerImpl {
    pub fn new(repo: Box<dyn ContactRepo>) -> Self {
        Self { repo }
    }
}

impl Lister for ListerImpl {
    fn list(&self, username: &str) -> Result<Vec<Contact>> {
        self.repo.find_contacts_by_username(username)
    }
}
