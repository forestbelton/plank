extern crate iron;

use model::post::*;
use model::schema::posts::dsl::*;
use pool::ConnectionPool;

use chrono::prelude::*;
use diesel;
use diesel::prelude::*;
use iron::prelude::{IronResult, Request, Response};
use uuid::Uuid;

use diesel::sqlite::SqliteConnection;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let conn = req.extensions.get::<ConnectionPool>().unwrap();
    let sql_conn : &SqliteConnection = &*conn;

    let new_uuid = Uuid::new_v4().hyphenated().to_string();
    let new_post = NewPost {
        uuid: &new_uuid,
        reply_uuid: None,
        create_date: Utc::now().naive_utc(),
        body: "Hello, world!",
        author: "Anonymous",
        attachment: None,
    };

    diesel::insert(&new_post).into(posts).execute(sql_conn).unwrap();
    Ok(Response::with((iron::status::Ok, "")))
}
