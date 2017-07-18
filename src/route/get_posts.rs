extern crate iron;
extern crate serde;
extern crate serde_json;

use model::post::Post;
use model::schema::posts;
use middleware::pool::ConnectionPool;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use iron::prelude::{IronResult, Request, Response};

const POST_LIMIT : i64 = 20;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let conn = req.extensions.get::<ConnectionPool>().unwrap();
    let sql_conn : &SqliteConnection = &*conn;

    let post_result = posts::table
        .order(posts::id.desc())
        .limit(POST_LIMIT)
        .load::<Post>(&*sql_conn);

    let resp = match post_result {
        Ok(posts) => {
            let post_json = serde_json::to_string(&posts).unwrap();
            Response::with((iron::status::Ok, post_json))
        },
        Err(err) => {
            error!("failed to fetch posts: {}", err);
            Response::with((iron::status::InternalServerError, "failed to fetch posts"))
        }
    };

    Ok(resp)
}