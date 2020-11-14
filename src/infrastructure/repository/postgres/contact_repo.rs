use crate::domain_logic::repository::{ContactRepo, InsertContact, QueryContact, UpdateContact};
use crate::domain_model::entities::contact::{Contact, Error, Result};
use crate::domain_model::phone_number::PhoneNumber;
use crate::infrastructure::repository::connection::R2D2Connection;
use crate::schema::contacts;
use diesel::{pg::PgConnection, prelude::*, result::Error as DieselError};
use shaku::Provider;

#[derive(Provider)]
#[shaku(interface = ContactRepo)]
pub struct ContactPgRepo {
    #[shaku(provide)]
    conn: Box<R2D2Connection<PgConnection>>,
}

impl ContactRepo for ContactPgRepo {
    fn save_new_contact(
        &self,
        username: &str,
        contact_name: &str,
        phone_number: PhoneNumber<&'_ str>,
    ) -> Result<i64> {
        let insert_contact = InsertContact {
            username,
            contact_name,
            phone_number,
        };

        diesel::insert_into(contacts::table)
            .values(&insert_contact)
            .returning(contacts::id)
            .get_result(&**self.conn)
            .map_err(Error::from)
    }

    fn find_contact(&self, id: i64) -> Result<Option<Contact>> {
        let contact = contacts::table
            .find(id)
            .first::<QueryContact>(&**self.conn)
            .map(Some)
            .or_else(|e| match e {
                DieselError::NotFound => Ok(None),
                e => Err(Error::from(e)),
            })?
            .map(Contact::from);

        Ok(contact)
    }

    fn find_contacts_by_username(&self, username: &str) -> Result<Vec<Contact>> {
        let contacts = contacts::table
            .filter(contacts::username.eq(username))
            .load::<QueryContact>(&**self.conn)
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
            .load::<QueryContact>(&**self.conn)
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
            .load::<QueryContact>(&**self.conn)
            .map_err(Error::from)?
            .iter()
            .cloned()
            .map(Contact::from)
            .collect();

        Ok(contacts)
    }

    fn update_contact_with_username(
        &self,
        username: &str,
        id: i64,
        contact_name: &str,
        phone_number: PhoneNumber<&'_ str>,
    ) -> Result<usize> {
        let update_contact = UpdateContact {
            contact_name,
            phone_number,
        };

        diesel::update(
            contacts::table
                .filter(contacts::username.eq(username))
                .filter(contacts::id.eq(id)),
        )
        .set(&update_contact)
        .execute(&**self.conn)
        .map_err(Error::from)
    }

    fn delete_contact_with_username(&self, username: &str, id: i64) -> Result<usize> {
        diesel::delete(
            contacts::table
                .filter(contacts::username.eq(username))
                .filter(contacts::id.eq(id)),
        )
        .execute(&**self.conn)
        .map_err(Error::from)
    }
}

impl From<QueryContact> for Contact {
    fn from(query_contact: QueryContact) -> Self {
        Self::new(
            query_contact.id,
            query_contact.contact_name,
            query_contact.phone_number,
        )
    }
}
