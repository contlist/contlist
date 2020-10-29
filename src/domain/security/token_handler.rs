use crate::domain::user::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait TokenHandler {
    type Claims: Serialize + DeserializeOwned;

    /// Encode `Claims` structore to token
    fn generate_token(claims: Self::Claims) -> Result<String>;

    /// Decode token into `CurrentUser` struct
    fn extract_claims(token: &str) -> Result<Self::Claims>;
}