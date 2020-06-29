mod contacts;
use contacts::endpoints as contact;

mod users;
use users::endpoints as user;

mod error;

use crate::db;
use rocket::Rocket;

pub fn rocket() -> Rocket {
    let db_pool = db::init_pool();

    Rocket::ignite()
        .mount("/user", user::api())
        .mount("/contact", contact::api())
        .manage(db_pool)
}
