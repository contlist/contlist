use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::Result;
use getset::Getters;
use shaku::Provider;

pub trait Delitor: 'static {
    fn delete(&self, username: &str, id: i64) -> Result<()>;
}

#[derive(Provider, Getters)]
#[shaku(interface = Delitor)]
#[getset(get = "pub")]
pub struct DelitorImpl {
    #[shaku(provide)]
    repo: Box<dyn ContactRepo>,
}

impl DelitorImpl {
    pub fn new(repo: Box<dyn ContactRepo>) -> Self {
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
