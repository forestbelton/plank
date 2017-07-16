#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;

#[macro_use] extern crate diesel;

extern crate chrono;
extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;
extern crate router;

mod config;
mod model;
mod pool;
mod route;

use pool::ConnectionPool;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use iron::{Iron, Chain};

fn main() {
    let app_config = config::get_config();
    let manager = ConnectionManager::<SqliteConnection>::new(app_config.database_url.clone());

    let pool = ConnectionPool::new(manager);

    let mut chain = Chain::new(route::router());
    chain.link_before(pool);

    Iron::new(chain).http(app_config.app_url).unwrap();

/*
    let ps = posts
        .load::<Post>(&*conn)
        .expect("error loading posts");
*/
}
