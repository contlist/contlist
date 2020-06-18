mod contacts;

mod users;
use users::endpoints as user;

use crate::db;
use rocket::Rocket;

pub fn rocket() -> Rocket {
    let db_pool = db::init_pool();

    Rocket::ignite().mount("/user", user::api()).manage(db_pool)
}
