use super::{Error, Result};
use crate::db::models::user::{InsertUser, QueryUser, UpdateUser};
use crate::db::pool::PooledConnection;
use crate::domain::user::{LoginUser, RegisterUser, User, UserRepo};
use crate::schema::users;
use argonautica::{Hasher, Verifier};
use boolinator::Boolinator;
use diesel::prelude::*;
use std::env;

pub struct UserPgRepo {
    connection: PooledConnection,
}

impl UserPgRepo {
    pub fn new(connection: PooledConnection) -> Self {
        Self { connection }
    }

    fn find_query_user_by_username(&self, username: &str) -> Result<Option<QueryUser>> {
        use diesel::result::Error as DieselError;

        users::table
            .find(username)
            .first::<QueryUser>(&self.connection)
            .map(Some)
            .or_else(|e| match e {
                DieselError::NotFound => Ok(None),
                e => Err(Error::from(e)),
            })
    }
}

impl UserRepo for UserPgRepo {
    fn save_new_user(&self, user: &RegisterUser) -> Result<usize> {
        let password_hash = hash_password(user.password)?;

        let insert_user = InsertUser {
            username: user.username,
            password_hash: password_hash.as_str(),
        };

        diesel::insert_into(users::table)
            .values(&insert_user)
            .execute(&self.connection)
            .map_err(Error::from)
    }

    fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = self
            .find_query_user_by_username(username)?
            .map(|query_user| User {
                username: query_user.username,
            });

        Ok(user)
    }

    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>> {
        let secret_key = env::var("ARGON_SECRET_KEY").expect("failed to read environment variable"); // TODO: move to config

        let quser = if let Some(quser) = self.find_query_user_by_username(credentials.username)? {
            quser
        } else {
            return Ok(None);
        };

        let user = Verifier::new()
            .with_password(credentials.password)
            .with_hash(quser.password_hash)
            .with_secret_key(secret_key)
            .verify()?
            .as_some(User {
                username: quser.username,
            });

        Ok(user)
    }

    fn update_user(&self, username: &str, user: &crate::domain::user::UpdateUser) -> Result<usize> {
        let password_hash = hash_password(user.password)?;

        let update_user = UpdateUser {
            password_hash: password_hash.as_str(),
        };

        diesel::update(users::table.find(username))
            .set(&update_user)
            .execute(&self.connection)
            .map_err(Error::from)
    }
}

fn hash_password(password: &str) -> Result<String> {
    let secret_key = env::var("ARGON_SECRET_KEY").expect("failed to read environment variable"); // TODO: move to config

    Hasher::default()
        .with_password(password)
        .with_secret_key(secret_key)
        .hash()
        .map_err(Error::from)
}
