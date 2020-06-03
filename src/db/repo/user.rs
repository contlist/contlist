use super::error::Error;
use crate::db::models::user::{InsertUser, QueryUser, UpdateUser};
use crate::db::pool::PgConnManager;
use crate::domain::user::{LoginUser, RegisterUser, User, UserRepo};
use crate::schema::users;
use argonautica::Hasher;
use diesel::{prelude::*, r2d2::PooledConnection};
use std::env;

pub struct UserPgRepo<'a> {
    connection: &'a PooledConnection<PgConnManager>,
}

impl<'a> UserPgRepo<'a> {
    pub fn new(connection: &'a PooledConnection<PgConnManager>) -> Self {
        Self { connection }
    }
}

impl<'a> UserRepo for UserPgRepo<'a> {
    fn register_user(&self, user: &RegisterUser) -> Result<usize, Error> {
        let password_hash = hash_password(user.password)?;

        let insert_user = InsertUser {
            username: user.username,
            password_hash: password_hash.as_str(),
        };

        let n = diesel::insert_into(users::table)
            .values(&insert_user)
            .execute(self.connection)?;

        Ok(n)
    }

    fn find_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let result = users::table
            .find(username)
            .first::<QueryUser>(self.connection);

        use diesel::result::Error as DieselError;
        let query_user = match result {
            Err(DieselError::NotFound) => return Ok(None),
            Err(e) => Err(Error::from(e)),
            Ok(user) => Ok(user),
        }?;

        let user = User {
            username: query_user.username,
        };

        Ok(Some(user))
    }

    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>, Error> {
        let password_hash = hash_password(credentials.password)?;

        let result = users::table
            .filter(users::username.eq(credentials.username))
            .filter(users::password_hash.eq(password_hash))
            .first::<QueryUser>(self.connection);

        use diesel::result::Error as DieselError;
        let query_user = match result {
            Err(DieselError::NotFound) => return Ok(None),
            Err(e) => Err(Error::from(e)),
            Ok(user) => Ok(user),
        }?;

        let user = User {
            username: query_user.username,
        };

        Ok(Some(user))
    }

    fn update_user(&self, username: &str, user: &crate::domain::user::UpdateUser) -> Result<usize, Error> {
        let password_hash = hash_password(user.password)?;

        let update_user = UpdateUser {
            password_hash: password_hash.as_str(),
        };

        let n = diesel::update(users::table.find(username))
            .set(&update_user)
            .execute(self.connection)?;

        Ok(n)
    }
}

fn hash_password(password: &str) -> Result<String, Error> {
    let secret_key = env::var("SECRET_KEY").expect("failed to read environment variable");

    Hasher::default()
        .with_password(password)
        .with_secret_key(secret_key)
        .hash()
        .map_err(anyhow::Error::msg)
        .map_err(anyhow::Error::into)
        .map_err(Error::DataPrepareError)
}
