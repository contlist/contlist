#![feature(proc_macro_hygiene, decl_macro, or_patterns, box_syntax, trait_alias)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod application_logic;
mod domain;
mod domain_logic;
mod domain_model;
mod infrastructure;
mod log;
mod module;
mod schema;
mod utils;

use infrastructure::web;

fn main() {
    dotenv::dotenv().expect("failed to set environment variables");
    log::LoggerConfig::default()
        .init()
        .expect("failed to init logger");

    web::rocket().launch();
}
