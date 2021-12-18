use crate::api;
use crate::services::env::Env;
use crate::services::logger;

use crate::services::database;
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn launch_server() {
    logger::init();

    let host = Env::get_var(&Env::Host);
    let port = Env::get_var(&Env::Port);

    let address = format!("{}:{}", host, port);
    let database_pool = database::get_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(database_pool.clone())
            .service(api::service())
    })
    .bind(address)
    .expect("Cannot bind to provided host")
    .run()
    .await
    .expect("Error while running server");
}
