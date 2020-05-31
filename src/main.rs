#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;

mod db;
mod schema;
// mod domain;
mod web;

fn main() {
    dotenv::dotenv().expect("failed to set environment variables");

    web::rocket().launch();
}
