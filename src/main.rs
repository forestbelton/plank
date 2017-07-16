#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;

#[macro_use] extern crate diesel;

extern crate chrono;
extern crate iron;
extern crate r2d2_diesel;
extern crate uuid;
extern crate router;

mod config;
mod model;
mod pool;
mod route;

use pool::ConnectionPool;
use model::post::*;

//use chrono::prelude::*;
//use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use iron::{Iron, Chain};

fn main() {
    let app_config = config::get_config();
    let manager = ConnectionManager::<SqliteConnection>::new(app_config.database_url);
    let pool = ConnectionPool::new(manager);

    let mut chain = Chain::new(route::router());
    chain.link_before(pool);

    Iron::new(chain).http(app_config.app_url).unwrap();

    //let pool = ConnectionPool::<SqliteConnection>new(manager);
/*
    use model::schema::posts::dsl::*;
    let conn = pool.get().unwrap();

    let ps = posts
        .load::<Post>(&*conn)
        .expect("error loading posts");

    println!("{:?}", ps);

    let new_uuid = Uuid::new_v4().hyphenated().to_string();
    let new_post = NewPost {
        uuid: &new_uuid,
        create_date: Utc::now().naive_utc(),
        body: "Hello, world!",
        author: "Anonymous",
        attachment: None,
    };

    diesel::insert(&new_post).into(posts).execute(&*conn).unwrap();
*/
}
