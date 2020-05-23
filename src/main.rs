#[macro_use]
extern crate diesel;
use diesel::prelude::*;

fn main() {
    dotenv::dotenv()
        .expect("failed to set environment variables");
}
