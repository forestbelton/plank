extern crate chrono;
extern crate diesel;
extern crate diesel_codegen;

use self::chrono::naive::NaiveDateTime;
use std::option::Option;

use model::schema::posts;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub uuid: String,
    pub author: String,
    pub create_date: NaiveDateTime,
    pub body: String,
    pub attachment: Option<String>,
    pub reply_uuid: Option<String>,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub uuid: &'a str,
    pub author: &'a str,
    pub create_date: NaiveDateTime,
    pub body: &'a str,
    pub attachment: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewReply<'a> {
    pub uuid: &'a str,
    pub reply_uuid: &'a str,
    pub author: &'a str,
    pub create_date: NaiveDateTime,
    pub body: &'a str,
    pub attachment: Option<&'a str>
}