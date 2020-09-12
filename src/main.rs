#![feature(proc_macro_hygiene, decl_macro, or_patterns, box_syntax)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod db;
mod domain;
mod log;
mod schema;
mod utils;
mod web;

fn main() {
    dotenv::dotenv().expect("failed to set environment variables");
    log::LoggerConfig::default()
        .init()
        .expect("failed to init logger");

    web::rocket().launch();
}
