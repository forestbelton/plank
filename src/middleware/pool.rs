extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;

use diesel::sqlite::SqliteConnection;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;
use self::r2d2::PooledConnection;
use self::r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub struct ConnectionPool {
    pool: Pool<ConnectionManager<SqliteConnection>>
}

impl ConnectionPool {
    pub fn new(manager: ConnectionManager<SqliteConnection>) -> ConnectionPool {
        let config = r2d2::Config::default();
        let pool = Pool::new(config, manager).expect("failed to create database pool");

        return ConnectionPool {
            pool: pool
        };
    }
}

impl typemap::Key for ConnectionPool {
    type Value = PooledConnection<ConnectionManager<SqliteConnection>>;
}

impl BeforeMiddleware for ConnectionPool {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let conn = self.pool.get().unwrap();

        req.extensions.insert::<ConnectionPool>(conn);
        Ok(())
    }
}

impl AfterMiddleware for ConnectionPool {
    fn after(&self, req: &mut Request, resp: Response) -> IronResult<Response> {
        req.extensions.remove::<ConnectionPool>();
        Ok(resp)
    }
}
