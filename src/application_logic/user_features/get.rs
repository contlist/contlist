use crate::domain_logic::repository::UserRepo;
use crate::domain_model::entities::user::{Error, Result, User};
use getset::Getters;
use shaku::Provider;
use std::sync::Arc;

pub trait Getter: 'static {
    fn get(&self, username: &str) -> Result<User>;
    fn list(&self) -> Result<Vec<User>>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Getter)]
pub struct GetterImpl {
    #[shaku(inject)]
    repo: Arc<dyn UserRepo>,
}

impl GetterImpl {
    pub fn new(repo: Arc<dyn UserRepo>) -> Self {
        Self { repo }
    }
}

impl Getter for GetterImpl {
    fn get(&self, username: &str) -> Result<User> {
        self.repo
            .find_user_by_username(username)?
            .ok_or(Error::NotFound)
    }

    fn list(&self) -> Result<Vec<User>> {
        self.repo.list_users()
    }
}
