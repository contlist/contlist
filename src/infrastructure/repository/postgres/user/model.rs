use crate::schema::users;
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable, Clone, Debug)]
#[table_name = "users"]
#[primary_key(username)]
pub struct QueryUser {
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
    pub create_timestamp: DateTime<Utc>,
    pub change_timestamp: Option<DateTime<Utc>>,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "users"]
pub struct InsertUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub password_salt: &'a str,
}

#[derive(AsChangeset, Clone, Debug)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub password_hash: &'a str,
    pub password_salt: &'a str,
}
