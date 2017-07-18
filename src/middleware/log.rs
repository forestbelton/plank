extern crate chrono;
extern crate iron;
extern crate log;

use chrono::{DateTime, Utc};
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;

#[derive(Clone, Copy)]
pub struct RequestLogger;

impl typemap::Key for RequestLogger {
    type Value = DateTime<Utc>;
}

impl BeforeMiddleware for RequestLogger {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let now = Utc::now();

        req.extensions.insert::<RequestLogger>(now);
        Ok(())
    }
}

impl AfterMiddleware for RequestLogger {
    fn after(&self, req: &mut Request, resp: Response) -> IronResult<Response> {
        let end = Utc::now();
        let start = req.extensions.get::<RequestLogger>().unwrap();
        let elapsed = end.signed_duration_since(*start).num_milliseconds();

        info!("{} {} {} {} {}ms", req.remote_addr, req.method, req.url, resp.status.unwrap(), elapsed);
        Ok(resp)
    }
}