use super::{Error, Result};
use crate::db::models::user::{InsertUser, QueryUser, UpdateUser};
use crate::db::pool::PooledConnection;
use crate::domain::user::{LoginUser, RegisterUser, User, UserRepo};
use crate::schema::users;
use boolinator::Boolinator;
use diesel::prelude::*;
use rand::{CryptoRng, Rng};

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
    fn save_new_user(&self, user: RegisterUser) -> Result<usize> {
        let (password_hash, password_salt) = hash_password(user.password)?;

        let insert_user = InsertUser {
            username: user.username,
            password_hash: password_hash.as_str(),
            password_salt: password_salt.as_str(),
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

    fn find_user_by_credentials(&self, credentials: LoginUser) -> Result<Option<User>> {
        let quser = if let Some(quser) = self.find_query_user_by_username(credentials.username)? {
            quser
        } else {
            return Ok(None);
        };

        let hash = base64::decode(quser.password_hash)?;
        let salt = base64::decode(quser.password_salt)?;

        let config = argon2::Config::default();
        let user = argon2::verify_raw(
            credentials.password.as_bytes(),
            salt.as_slice(),
            hash.as_slice(),
            &config,
        )?
        .as_some(User {
            username: quser.username,
        });

        Ok(user)
    }

    fn update_user(&self, username: &str, user: crate::domain::user::UpdateUser) -> Result<usize> {
        let (password_hash, password_salt) = hash_password(user.password)?;

        let update_user = UpdateUser {
            password_hash: password_hash.as_str(),
            password_salt: password_salt.as_str(),
        };

        diesel::update(users::table.find(username))
            .set(&update_user)
            .execute(&self.connection)
            .map_err(Error::from)
    }
}

// TODO: move hashing to service
fn hash_password(password: &str) -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let salt = gen_salt(&mut rng);
    let config = argon2::Config::default();
    argon2::hash_raw(password.as_bytes(), &salt[..], &config)
        .map(|raw| (base64::encode(&raw), base64::encode(&salt[..])))
        .map_err(Error::from)
}

const SALT_LENGTH: usize = 32; //TODO: move to config
type Salt = [u8; SALT_LENGTH];

fn gen_salt<R: Rng + CryptoRng>(rng: &mut R) -> Salt {
    let mut salt = [0u8; SALT_LENGTH];
    rng.fill(&mut salt);
    salt
}
