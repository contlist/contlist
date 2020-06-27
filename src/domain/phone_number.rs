use super::utils::OptionExt;
use diesel::deserialize::{self, FromSql};
use diesel::{backend::Backend, sql_types::Text};
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};
use std::{convert::TryFrom, ops::Deref};
use thiserror::Error;

/// A string that matches (+)?[0-9]+
#[derive(
    Serialize, AsExpression, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
#[sql_type = "Text"]
pub struct PhoneNumber<T>(T);

impl<T> PhoneNumber<T> {
    /// Creates a phone number without checking the value.
    ///
    /// # Safety
    ///
    /// The values must match phone pattern.
    pub const unsafe fn new_unchecked(value: T) -> Self {
        PhoneNumber(value)
    }

    /// Unwraps the `PhoneNumber`, returning the underlying `String`.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: AsRef<str>> PhoneNumber<T> {
    /// Creates a phone number if the given string matches phone pattern.
    pub fn new(value: T) -> Result<Self> {
        value
            .as_ref()
            .chars()
            .enumerate()
            .skip_while(|&(i, c)| i == 0 && c == '+')
            .find(|t| !t.1.is_digit(10))
            .err_and(PhoneNumber(value))
            .map_err(|(p, c)| Error::InvalidCharacterError(c, p))
    }

    /// Returns a `str` to underlying `String`
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Returns a mutable `str` to underlying `String`.
    ///
    /// This function is unsafe as the string could no longer
    /// be valid phone number after mutation.
    pub unsafe fn as_mut_str(&mut self) -> &mut str
    where
        T: AsMut<str>,
    {
        self.0.as_mut()
    }
}

impl TryFrom<String> for PhoneNumber<String> {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a> TryFrom<&'a str> for PhoneNumber<&'a str> {
    type Error = Error;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<T: AsRef<str>> Deref for PhoneNumber<T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<'de, T: Deserialize<'de> + AsRef<str>> Deserialize<'de> for PhoneNumber<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        T::deserialize(deserializer)
            .and_then(|value| PhoneNumber::new(value).map_err(Error::to_serde))
    }
}

impl<DB, T> FromSql<Text, DB> for PhoneNumber<T>
where
    DB: Backend,
    T: FromSql<Text, DB> + AsRef<str>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        T::from_sql(bytes).and_then(|v| Self::new(v).map_err(|e| Box::new(e).into()))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid character: {0} is not avaliable at position {1}")]
    InvalidCharacterError(char, usize),
}

impl Error {
    // Can't implement From<Error> for DeError
    fn to_serde<E: DeError>(self) -> E {
        use serde::de::Unexpected;
        match self {
            Error::InvalidCharacterError(c, _) => E::invalid_value(Unexpected::Char(c), &"digit"),
        }
    }
}

// // Conflict implementations, see Error::to_serde
// // impl<E: DeError> From<Error> for E {
// //     fn from(src: Error) -> Self {
// //         todo!()
// //     }
// // }
