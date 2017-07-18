#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate fern;
extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;
extern crate router;

mod config;
mod middleware;
mod model;
mod route;

use middleware::pool::ConnectionPool;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use iron::{Iron, Chain};

fn main() {
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("{} :: {} [{}] {}",
                chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message))
        })
        .level(log::LogLevelFilter::Info)
        .chain(std::io::stdout())
        .apply();

    let app_config = config::get_config();
    let manager = ConnectionManager::<SqliteConnection>::new(app_config.database_url.clone());

    let pool = ConnectionPool::new(manager);

    let mut chain = Chain::new(route::router());
    chain.link_before(pool);

    info!("Starting server at {}", app_config.app_url);
    Iron::new(chain).http(app_config.app_url).unwrap();

/*
    let ps = posts
        .load::<Post>(&*conn)
        .expect("error loading posts");
*/
}
