use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::Result;
use getset::Getters;

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct Delete<R> {
    repo: R,
}

impl<R> Delete<R>
where
    R: ContactRepo,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn handle(&self, username: &str, id: i64) -> Result<()> {
        self.repo
            .delete_contact_with_username(username, id)
            .map(|_| ())
    }
}
