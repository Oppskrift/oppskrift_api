#[macro_use]
extern crate actix_web;

mod api;
mod entities;
mod middlewares;
mod router;
mod services;
mod tools;
mod lib;

use actix_web::{App, HttpServer};
use dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to parse .env file");

    let host = dotenv::var("HOST").expect("HOST not found");
    let port = dotenv::var("PORT").expect("PORT not found");
    let url = format!("{}:{}", host, port);

    HttpServer::new(||
        App::new()
            .service(router::config())
    )
        .bind(&url)?
        .run()
        .await
}
