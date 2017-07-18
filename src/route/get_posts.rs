extern crate iron;
extern crate serde;
extern crate serde_json;

use model::post::Post;
use model::schema::posts;
use middleware::pool::ConnectionPool;

use diesel::prelude::*;
use diesel::expression::dsl::sql;
use diesel::sqlite::SqliteConnection;
use iron::prelude::{IronResult, Request, Response};

const POST_LIMIT : i64 = 20;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let conn = req.extensions.get::<ConnectionPool>().unwrap();
    let sql_conn : &SqliteConnection = &*conn;

    info!("Fetching latest post IDs");

    let latest_post_ids = posts::table
        .select(posts::id)
        .filter(posts::reply_id.is_null())
        .order(posts::id.desc())
        .limit(POST_LIMIT)
        .load::<i32>(&*sql_conn);

    if latest_post_ids.is_err() {
        error!("Failed to fetch post IDs: {}", latest_post_ids.unwrap_err());
        return Ok(Response::with((iron::status::InternalServerError, "failed to fetch posts")));
    }

    let post_ids = latest_post_ids.unwrap();
    let mut formatted_ids = "".to_string();

    for post_id in post_ids {
        formatted_ids = if formatted_ids == "" {
            post_id.to_string()
        } else {
            format!("{}, {}", formatted_ids, post_id.to_string())
        };
    }

    info!("Fetching posts and replies for IDs = [{}]", formatted_ids);
    let id_clause = format!("id IN ({}) OR reply_id IN ({})", formatted_ids, formatted_ids);

    let posts_and_replies = posts::table
        .filter(sql(&id_clause))
        .load::<Post>(&*sql_conn);

    let resp = match posts_and_replies {
        Ok(posts) => {
            let post_json = serde_json::to_string(&posts).unwrap();
            Response::with((iron::status::Ok, post_json))
        },
        Err(err) => {
            error!("Failed to fetch posts: {}", err);
            Response::with((iron::status::InternalServerError, "failed to fetch posts"))
        }
    };

    Ok(resp)
}