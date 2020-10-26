use super::model::{InsertContact, QueryContact, UpdateContact};
use crate::domain::contact::{Contact, ContactRepo, CreateContact};
use crate::domain::contact::{Error, Result};
use crate::domain::phone_number::PhoneNumber;
use crate::infrastructure::repository::postgres::pool::PooledConnection;
use crate::schema::contacts;
use diesel::{prelude::*, result::Error as DieselError};

pub struct ContactPgRepo {
    connection: PooledConnection,
}

impl ContactPgRepo {
    pub fn new(connection: PooledConnection) -> Self {
        Self { connection }
    }
}

impl ContactRepo for ContactPgRepo {
    fn save_new_contact(&self, username: &str, contact: CreateContact<'_>) -> Result<i64> {
        let insert_contact = InsertContact {
            username,
            contact_name: contact.contact_name,
            phone_number: contact.phone_number,
        };

        diesel::insert_into(contacts::table)
            .values(&insert_contact)
            .returning(contacts::id)
            .get_result(&self.connection)
            .map_err(Error::from)
    }

    fn find_contact(&self, id: i64) -> Result<Option<Contact>> {
        let contact = contacts::table
            .find(id)
            .first::<QueryContact>(&self.connection)
            .map(Some)
            .or_else(|e| match e {
                DieselError::NotFound => Ok(None),
                e => Err(Error::from(e)),
            })?
            .map(Contact::from);

        Ok(contact)
    }

    fn find_contact_by_username(&self, username: &str) -> Result<Vec<Contact>> {
        let contacts = contacts::table
            .filter(contacts::username.eq(username))
            .load::<QueryContact>(&self.connection)
            .map_err(Error::from)?
            .iter()
            .cloned()
            .map(Contact::from)
            .collect();

        Ok(contacts)
    }

    fn find_contacts_by_name(&self, username: &str, name: &str) -> Result<Vec<Contact>> {
        let contacts = contacts::table
            .filter(contacts::username.eq(username))
            .filter(contacts::contact_name.eq(name))
            .load::<QueryContact>(&self.connection)
            .map_err(Error::from)?
            .iter()
            .cloned()
            .map(Contact::from)
            .collect();

        Ok(contacts)
    }

    fn find_contacts_by_number(
        &self,
        username: &str,
        number: PhoneNumber<&'_ str>,
    ) -> Result<Vec<Contact>> {
        let contacts = contacts::table
            .filter(contacts::username.eq(username))
            .filter(contacts::phone_number.eq(number))
            .load::<QueryContact>(&self.connection)
            .map_err(Error::from)?
            .iter()
            .cloned()
            .map(Contact::from)
            .collect();

        Ok(contacts)
    }

    fn update_contact(
        &self,
        id: i64,
        contact: crate::domain::contact::UpdateContact<'_>,
    ) -> Result<usize> {
        let update_contact = UpdateContact::from(contact);

        diesel::update(contacts::table.find(id))
            .set(&update_contact)
            .execute(&self.connection)
            .map_err(Error::from)
    }

    fn update_contact_with_username(
        &self,
        username: &str,
        id: i64,
        contact: crate::domain::contact::UpdateContact<'_>,
    ) -> Result<usize> {
        let update_contact = UpdateContact::from(contact);

        diesel::update(
            contacts::table
                .filter(contacts::username.eq(username))
                .filter(contacts::id.eq(id)),
        )
        .set(&update_contact)
        .execute(&self.connection)
        .map_err(Error::from)
    }

    fn delete_contact(&self, id: i64) -> Result<usize> {
        diesel::delete(contacts::table.find(id))
            .execute(&self.connection)
            .map_err(Error::from)
    }

    fn delete_contact_with_username(&self, username: &str, id: i64) -> Result<usize> {
        diesel::delete(
            contacts::table
                .filter(contacts::username.eq(username))
                .filter(contacts::id.eq(id)),
        )
        .execute(&self.connection)
        .map_err(Error::from)
    }
}

impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        let error = anyhow::Error::new(value);
        Error::RepoError(error.into())
    }
}

impl From<QueryContact> for Contact {
    fn from(value: QueryContact) -> Self {
        Self {
            id: value.id,
            contact_name: value.contact_name,
            phone_number: value.phone_number,
        }
    }
}

impl<'a> From<crate::domain::contact::UpdateContact<'a>> for UpdateContact<'a> {
    fn from(value: crate::domain::contact::UpdateContact<'a>) -> Self {
        Self {
            contact_name: value.contact_name,
            phone_number: value.phone_number,
        }
    }
}