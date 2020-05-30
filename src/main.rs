#[macro_use]
extern crate diesel;
use diesel::prelude::*;

mod schema;
mod db;
// mod domain;
mod web;

fn main() {
    dotenv::dotenv()
        .expect("failed to set environment variables");
}
