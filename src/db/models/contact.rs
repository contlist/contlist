use crate::domain::phone_number::PhoneNumber;
use crate::schema::contacts;
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable, Clone, Debug)]
#[primary_key(id)]
#[table_name = "contacts"]
pub struct QueryContact {
    pub id: i64,
    pub username: String,
    pub contact_name: String,
    pub phone_number: PhoneNumber<String>,
    pub create_timestamp: DateTime<Utc>,
    pub change_timestamp: Option<DateTime<Utc>>,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "contacts"]
pub struct InsertContact<'a> {
    pub username: &'a str,
    pub contact_name: &'a str,
    pub phone_number: PhoneNumber<&'a str>,
}

#[derive(AsChangeset, Clone, Debug)]
#[table_name = "contacts"]
pub struct UpdateContact<'a> {
    pub contact_name: &'a str,
    pub phone_number: PhoneNumber<&'a str>,
}
