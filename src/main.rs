#![feature(proc_macro)]
#[macro_use]
extern crate serde_derive;

extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod config;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

fn main() {
    let app_config = config::get_config();

    let r2d2_config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(app_config.database);
    let pool = r2d2::Pool::new(r2d2_config, manager).expect("failed to create database pool");
}
