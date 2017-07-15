#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;

#[macro_use] extern crate diesel;

extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;

mod config;
mod model;

use model::post::*;

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use uuid::Uuid;

fn main() {
    use model::schema::posts::dsl::*;

    let app_config = config::get_config();

    let r2d2_config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(app_config.database_url);
    let pool = r2d2::Pool::new(r2d2_config, manager).expect("failed to create database pool");

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
}
