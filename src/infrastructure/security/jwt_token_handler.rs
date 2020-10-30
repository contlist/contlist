use crate::domain::security::token_handler::TokenHandler;
use crate::domain::user::{auth::Claims, Error, Result};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use jsonwebtoken::{self as jwt, Algorithm};
use std::env;

pub struct JwtTokenHandler;

impl TokenHandler for JwtTokenHandler {
    type Claims = Claims;

    fn generate_token(claims: Self::Claims) -> Result<String> {
        use jwt::{EncodingKey, Header};
        let secret_key = env::var("JWT_SECRET_KEY").expect("failed to read environment variable"); // TODO: move to config

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(secret_key.as_bytes());
        jwt::encode(&header, &claims, &key).map_err(Error::from)
    }

    fn extract_claims(token: &str) -> Result<Self::Claims> {
        use jwt::{DecodingKey, Validation};
        let secret_key = env::var("JWT_SECRET_KEY").expect("failed to read environment variable"); // TODO: move to config

        let key = DecodingKey::from_secret(secret_key.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        jwt::decode(token, &key, &validation)
            .map_err(Error::from)
            .map(|token_data| token_data.claims)
    }
}

// postponed until refactoring
// impl From<JwtError> for Error {
//     fn from(src: JwtError) -> Self {
//         match src.kind() {
//             JwtErrorKind::ExpiredSignature => Error::ExpiredTokenError,
//             _ => Error::TokenError(Box::new(src).into()),
//         }
//     }
// }
