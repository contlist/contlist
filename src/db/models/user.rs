use crate::schema::users;

#[derive(Queryable, Identifiable, Clone, Debug)]
#[table_name = "users"]
#[primary_key(username)]
pub struct QueryUser {
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
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
