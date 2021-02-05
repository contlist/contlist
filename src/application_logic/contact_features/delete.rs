use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::Result;
use getset::Getters;
use std::sync::Arc;

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct Delete {
    repo: Arc<dyn ContactRepo>,
}

impl Delete {
    pub fn new(repo: Arc<dyn ContactRepo>) -> Self {
        Self { repo }
    }

    pub fn handle(&self, username: &str, id: i64) -> Result<()> {
        self.repo
            .delete_contact_with_username(username, id)
            .map(|_| ())
    }
}
