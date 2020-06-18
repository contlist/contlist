pub mod contacts;
pub mod users;

use crate::db;
use rocket::Rocket;

pub fn rocket() -> Rocket {
    let db_pool = db::init_pool();

    Rocket::ignite().manage(db_pool)
}
