mod api;
mod entities;
mod services;

#[macro_use]
extern crate actix_web;
extern crate diesel;
extern crate dotenv;

use services::server;

fn main() {
    server::launch_server();
}
