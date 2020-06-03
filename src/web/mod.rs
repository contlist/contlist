pub mod contacts;
pub mod error;
pub mod users;

use crate::db::pool;
use rocket::Rocket;

pub fn rocket() -> Rocket {
    let db = pool::init_db();

    Rocket::ignite().manage(db)
}
