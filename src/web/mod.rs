pub mod contacts;
pub mod users;

use crate::db;
use rocket::Rocket;

pub fn rocket() -> Rocket {
    let db = db::init_db();

    Rocket::ignite().manage(db)
}
