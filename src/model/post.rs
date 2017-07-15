extern crate diesel;

use std::option::Option;
use std::time::SystemTime;

use model::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub uuid: String,
    pub reply_uuid: Option<String>,
    pub create_date: SystemTime,
    pub author: String,
    pub body: String,
    pub attachment: Option<String>
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub author: &'a str,
    pub body: &'a str,
    pub attachment: Option<&'a str>
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewReply<'a> {
    pub author: &'a str,
    pub body: &'a str,
    pub attachment: Option<&'a str>,
    pub reply_uuid: &'a str
}
