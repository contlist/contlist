use crate::domain_logic::repository::{InsertUser, QueryUser, UpdateUser, UserRepo};
use crate::domain_model::entities::user::{Error, Result, User};
use crate::infrastructure::repository::connection::R2D2Connection;
use crate::schema::users;
use diesel::prelude::*;
use shaku::Provider;

#[derive(Provider)]
#[shaku(interface = UserRepo)]
pub struct UserPgRepo {
    #[shaku(provide)]
    conn: Box<R2D2Connection<PgConnection>>,
}

impl UserRepo for UserPgRepo {
    fn save_new_user(
        &self,
        username: &str,
        password_hash: &str,
        password_salt: &str,
    ) -> Result<usize> {
        let insert_user = InsertUser {
            username,
            password_hash,
            password_salt,
        };

        diesel::insert_into(users::table)
            .values(&insert_user)
            .execute(&**self.conn)
            .map_err(Error::from)
    }

    fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        use diesel::result::Error as DieselError;

        users::table
            .find(username)
            .first::<QueryUser>(&**self.conn)
            .map(User::from)
            .map(Some)
            .or_else(|e| match e {
                DieselError::NotFound => Ok(None),
                e => Err(Error::from(e)),
            })
    }

    fn list_users(&self) -> Result<Vec<User>> {
        let users = users::table
            .load::<QueryUser>(&**self.conn)?
            .iter()
            .cloned()
            .map(User::from)
            .collect();

        Ok(users)
    }

    fn update_user(
        &self,
        username: &str,
        password_hash: &str,
        password_salt: &str,
    ) -> Result<usize> {
        let update_user = UpdateUser {
            password_hash,
            password_salt,
        };

        diesel::update(users::table.find(username))
            .set(&update_user)
            .execute(&**self.conn)
            .map_err(Error::from)
    }
}

impl From<QueryUser> for User {
    fn from(quser: QueryUser) -> Self {
        Self::new(quser.username, quser.password_hash, quser.password_salt)
    }
}
