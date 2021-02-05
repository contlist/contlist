use crate::domain_model::entities::user::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait TokenHandler: Debug {
    type Claims: Serialize + DeserializeOwned;

    /// Encode `Claims` structore to token
    fn generate_token(&self, claims: Self::Claims) -> Result<String>;

    /// Decode token into `CurrentUser` struct
    fn extract_claims(&self, token: &str) -> Result<Self::Claims>;
}
