mod contacts;
use contacts::endpoints as contact;

mod users;
use users::endpoints as user;

mod error;
pub use error::{Error, Result};

mod spa;

use crate::infrastructure::repository::postgres;
use rocket::config::{Config, Environment};
use rocket::Rocket;
use rocket_cors::CorsOptions;

pub fn rocket() -> Rocket {
    let db_pool = postgres::init_pool();

    let cors = CorsOptions::default()
        .to_cors()
        .expect("failed to create cors");

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(3721)
        .finalize()
        .expect("failed to create config");

    rocket::custom(config)
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount("/", spa::api())
        .mount("/user", user::api())
        .mount("/contact", contact::api())
        .manage(db_pool)
        .manage(cors.clone())
        .attach(cors)
}
