extern crate iron;

use model::post::*;
use model::schema::posts::dsl::*;
use middleware::pool::ConnectionPool;

use chrono::prelude::*;
use diesel;
use diesel::prelude::*;
use iron::prelude::{IronResult, Request, Response, Plugin};
use params::{Params, Value};
use uuid::Uuid;

use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

const MAX_BODY_SIZE: usize = 512;
const MAX_AUTHOR_SIZE: usize = 36;
const MAX_ATTACH_SIZE: usize = 256;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let params = req.get::<Params>().unwrap();

    // TODO: please tell me there is an iron Params-based form
    // validation library out there

    info!("Validating input for new post");

    let _body = match params.find(&["body"]) {
        Some(&Value::String(ref value)) => value,
        _ => {
            return Ok(Response::with((iron::status::BadRequest, "no body param")));
        }
    };

    if _body.len() > MAX_BODY_SIZE {
        return Ok(Response::with((iron::status::BadRequest, "body too big")));
    }

    let _author = match params.find(&["author"]) {
        Some(&Value::String(ref value)) => value,
        _ => "Anonymous"
    };

    if _author.len() > MAX_AUTHOR_SIZE {
        return Ok(Response::with((iron::status::BadRequest, "author too big")));
    }

    /*let _attachment : Option<String> = match params.find(&["attachment"]) {
        Some(&Value::String(ref value)) => Some(value),
        _ => None
    };*/

    let _uuid = Uuid::new_v4().hyphenated().to_string();
    let _source_addr = req.remote_addr.ip().to_string();

    let new_post = NewPost {
        uuid: &_uuid,
        reply_id: None,
        create_date: Utc::now().naive_utc(),
        body: &_body,
        author: &_author,
        attachment: None,
        source_addr: &_source_addr,
    };

    info!("Creating new post: {:?}", new_post);

    {
        let conn = req.extensions.get::<ConnectionPool>().unwrap();
        let sql_conn : &SqliteConnection = &*conn;

        let _ = sql_conn.transaction::<_, Error, _>(||{
            diesel::insert(&new_post).into(posts).execute(sql_conn).unwrap();
            return Ok(());
        });
    }

    Ok(Response::with((iron::status::Ok, "")))
}
