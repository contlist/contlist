use crate::db::repo::error::Error as DbError;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct RegisterUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Clone, Debug)]
pub struct LoginUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateUser<'a> {
    pub password: &'a str,
}

pub trait UserRepo {
    fn register_user(&self, user: &RegisterUser) -> Result<usize, DbError>;
    fn find_user_by_username(&self, username: &str) -> Result<Option<User>, DbError>;
    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>, DbError>;
    fn update_user(&self, username: &str, user: &UpdateUser) -> Result<usize, DbError>;
}
