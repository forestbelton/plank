#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate fern;
extern crate iron;
extern crate mount;
extern crate params;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate router;
extern crate staticfile;
extern crate uuid;

mod config;
mod middleware;
mod model;
mod route;

use middleware::log::RequestLogger;
use middleware::pool::ConnectionPool;

use mount::Mount;
use staticfile::Static;
use std::env;
use std::path::Path;
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

    let logger = RequestLogger;
    let pool = ConnectionPool::new(manager);

    // TODO: Move static path into configuration
    let mut mount = Mount::new();

    mount
        .mount("/api", route::router())
        .mount("/", Static::new(Path::new("static/")));

    let mut chain = Chain::new(mount);

    chain.link_before(logger);
    chain.link_before(pool);

    chain.link_after(logger);

    let cwd = env::current_dir().unwrap();
    info!("Current directory: {}", cwd.display());

    let url = format!("0.0.0.0:{}", app_config.app_port);
    info!("Starting server at {}", url);

    Iron::new(chain).http(url).unwrap();
}
