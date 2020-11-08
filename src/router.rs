use actix_web::{Scope, web};
use crate::api;

pub fn config() -> Scope {
    web::scope("/api")
        .service(api::ping_controller::ping)
        .service(recipes_scope())
        .service(users_scope())

}

fn recipes_scope() -> Scope {
    web::scope("/recipes")
        .service(api::recipes_controller::list)
        .service(api::recipes_controller::get)
        .service(api::recipes_controller::create)
}

fn users_scope() -> Scope {
    web::scope("/users")
        .service(api::users_controller::list)
        .service(api::users_controller::get)
        .service(api::users_controller::create)
}
