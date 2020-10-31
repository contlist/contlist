use crate::domain::user::{Result, User};

pub trait UserRepo {
    fn save_new_user(&self, username: &str, password: &str) -> Result<usize>;
    fn find_user_by_username(&self, username: &str) -> Result<Option<User>>;
    fn find_user_by_credentials(&self, username: &str, password: &str) -> Result<Option<User>>;
    fn update_user(&self, username: &str, password: &str) -> Result<usize>;
}
