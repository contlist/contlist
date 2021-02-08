use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::Result;
use getset::Getters;
use std::sync::Arc;

pub trait Delitor {
    fn delete(&self, username: &str, id: i64) -> Result<()>;
}

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct DelitorImpl {
    repo: Arc<dyn ContactRepo>,
}

impl DelitorImpl {
    pub fn new(repo: Arc<dyn ContactRepo>) -> Self {
        Self { repo }
    }
}

impl Delitor for DelitorImpl {
    fn delete(&self, username: &str, id: i64) -> Result<()> {
        self.repo
            .delete_contact_with_username(username, id)
            .map(|_| ())
    }
}
