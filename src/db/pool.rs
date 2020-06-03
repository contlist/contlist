use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use std::env;

pub type PgConnManager = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<PgConnManager>;

pub fn init_db() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let bd_manager = PgConnManager::new(database_url);
    Pool::new(bd_manager).expect("failed to create database pool")
}
