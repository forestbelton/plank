extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;

use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;
use r2d2_diesel::ConnectionManager;

pub struct ConnectionPool<T: 'static + diesel::Connection> {
    pub pool: r2d2::Pool<ConnectionManager<T>>,
}

impl<T: diesel::Connection> ConnectionPool<T> {
    pub fn new(manager: ConnectionManager<T>) -> ConnectionPool<T> {
        let r2d2_config = r2d2::Config::default();
        let pool = r2d2::Pool::new(r2d2_config, manager).expect("failed to create database pool");

        return ConnectionPool {
            pool: pool
        };
    }
}

impl<T: diesel::Connection> typemap::Key for ConnectionPool<T> {
    type Value = T;
}

impl<T: diesel::Connection> BeforeMiddleware for ConnectionPool<T> {
    fn before(&self, _: &mut Request) -> IronResult<()> {
        return Ok(());
    }
}

impl<T: diesel::Connection> AfterMiddleware for ConnectionPool<T> {
    fn after(&self, _: &mut Request, resp: Response) -> IronResult<Response> {
        return Ok(resp);
    }
}
