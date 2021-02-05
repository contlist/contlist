use crate::domain_model::entities::user::{Result, User};
use std::fmt::Debug;

pub trait UserRepo: Debug {
    fn save_new_user(
        &self,
        username: &str,
        password_hash: &str,
        password_salt: &str,
    ) -> Result<usize>;
    fn find_user_by_username(&self, username: &str) -> Result<Option<User>>;
    fn update_user(
        &self,
        username: &str,
        password_hash: &str,
        password_salt: &str,
    ) -> Result<usize>;
}
