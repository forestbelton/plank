extern crate chrono;
extern crate diesel;
extern crate diesel_codegen;
extern crate serde;

use self::chrono::naive::NaiveDateTime;
use std::option::Option;

use model::schema::posts;

#[derive(Serialize, Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub uuid: String,
    pub author: String,
    pub create_date: NaiveDateTime,
    pub body: String,
    pub attachment: Option<String>,
    pub reply_id: Option<i32>,
    pub source_addr: String
}

#[derive(Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub uuid: &'a str,
    pub reply_id: Option<i32>,
    pub author: &'a str,
    pub create_date: NaiveDateTime,
    pub body: &'a str,
    pub attachment: Option<&'a str>,
    pub source_addr: &'a str
}
