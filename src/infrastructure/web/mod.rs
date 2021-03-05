pub mod current_user;
pub mod error;

mod endpoints;
pub use endpoints::contact_endpoints as contact;
pub use endpoints::swagger;
pub use endpoints::user_endpoints as user;

use crate::module::MainModule;
use rocket::config::{Config, Environment};
use rocket::Rocket;
use rocket_cors::CorsOptions;

pub fn rocket() -> Rocket {
    let module = MainModule::builder().build();

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
        .mount("/user", user::api())
        .mount("/contact", contact::api())
        .mount("/swagger", swagger::api())
        .manage(cors.clone())
        .manage(box module)
        .attach(cors)
}
