extern crate iron;

use iron::prelude::{IronResult, Request, Response};

pub fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}
