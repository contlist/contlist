use super::error::{Error, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{self as jwt, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub username: String,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, duration: Duration) -> Self {
        let exp = (Utc::now() + duration).timestamp();
        Self { username, exp }
    }

    /// Encode `Claims` structore to token
    pub fn as_token(&self) -> Result<String> {
        use jwt::{EncodingKey, Header};
        let secret_key = env::var("JWT_SECRET_KEY").expect("failed to read environment variable"); // TODO: move to config

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(secret_key.as_bytes());
        jwt::encode(&header, &self, &key).map_err(Error::from)
    }

    /// Decode token into `CurrentUser` struct
    pub fn from_token(token: &str) -> Result<Self> {
        use jwt::{DecodingKey, Validation};
        let secret_key = env::var("JWT_SECRET_KEY").expect("failed to read environment variable"); // TODO: move to config

        let key = DecodingKey::from_secret(secret_key.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        jwt::decode(token, &key, &validation)
            .map_err(Error::from)
            .map(|token_data| token_data.claims)
    }
}
