#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;
extern crate csv;
extern crate ctrlc;
#[macro_use]
extern crate log;
extern crate env_logger;
mod index;
mod register;
mod server;
mod utils;

use server::Server;

fn main() {
    //env_logger::init();
    Server::new().launch();
}
