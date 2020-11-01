use chrono::{Duration, Utc};
use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Getters, MutGetters, Debug)]
pub struct Claims {
    username: String,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, duration: Duration) -> Self {
        let exp = (Utc::now() + duration).timestamp();
        Self { username, exp }
    }
}
