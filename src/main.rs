#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel_codegen;

#[macro_use]
extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

mod config;
mod model;

use model::schema::*;
use model::post::*;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

fn main() {
    use model::schema::posts::dsl::*;

    let app_config = config::get_config();

    let r2d2_config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(app_config.database_url);
    let pool = r2d2::Pool::new(r2d2_config, manager).expect("failed to create database pool");

    let conn = pool.get();
    //let ps = posts.find(1).load::<Post>(&conn).expect("can't fetch posts");
}
