use super::error::Error;
use crate::db::models::user::{InsertUser, QueryUser, UpdateUser};
use crate::db::pool::PgConnManager;
use crate::domain::user::{LoginUser, RegisterUser, User, UserRepo};
use crate::schema::users;
use argonautica::{Hasher, Verifier};
use diesel::{prelude::*, r2d2::PooledConnection};
use std::env;

pub struct UserPgRepo<'a> {
    connection: &'a PooledConnection<PgConnManager>,
}

impl<'a> UserPgRepo<'a> {
    pub fn new(connection: &'a PooledConnection<PgConnManager>) -> Self {
        Self { connection }
    }
    
    fn find_query_user_by_username(&self, username: &str) -> Result<Option<QueryUser>, Error> {
        use diesel::result::Error as DieselError;

        users::table
            .find(username)
            .first::<QueryUser>(self.connection)
            .map(Some)
            .or_else(|e| match e {
                DieselError::NotFound => Ok(None),
                e => Err(Error::from(e)),
            })
    }
}

impl<'a> UserRepo for UserPgRepo<'a> {
    fn register_user(&self, user: &RegisterUser) -> Result<usize, Error> {
        let password_hash = hash_password(user.password)?;

        let insert_user = InsertUser {
            username: user.username,
            password_hash: password_hash.as_str(),
        };

        diesel::insert_into(users::table)
            .values(&insert_user)
            .execute(self.connection)
            .map_err(Error::from)
    }

    fn find_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let user = self
            .find_query_user_by_username(username)?
            .map(|query_user| {
                User {
                    username: query_user.username,
                }
            });

        Ok(user)
    }

    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>, Error> {
        let secret_key = env::var("SECRET_KEY").expect("failed to read environment variable");

        let quser = if let Some(quser) = self.find_query_user_by_username(credentials.username)? {
            quser
        } else {
            return Ok(None);
        };

        let verified = Verifier::new()
            .with_hash(quser.password_hash)
            .with_secret_key(secret_key)
            .verify()?;

        if verified {
            let user = User {
                username: quser.username,
            };

            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    fn update_user(&self, username: &str, user: &crate::domain::user::UpdateUser) -> Result<usize, Error> {
        let password_hash = hash_password(user.password)?;

        let update_user = UpdateUser {
            password_hash: password_hash.as_str(),
        };

        diesel::update(users::table.find(username))
            .set(&update_user)
            .execute(self.connection)
            .map_err(Error::from)
    }
}

fn hash_password(password: &str) -> Result<String, Error> {
    let secret_key = env::var("SECRET_KEY").expect("failed to read environment variable");

    Hasher::default()
        .with_password(password)
        .with_secret_key(secret_key)
        .hash()
        .map_err(Error::from)
}
