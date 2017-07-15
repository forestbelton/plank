#![feature(proc_macro)]
#[macro_use]
extern crate serde_derive;

mod config;

fn main() {
    let config = config::get_config();
    println!("{:?}", config);
}
