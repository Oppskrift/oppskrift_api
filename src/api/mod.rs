use actix_web::{web, Responder, Scope};

mod cookbooks;
mod recipes;

#[get("")]
async fn index() -> impl Responder {
    "Hello API"
}

pub fn service() -> Scope {
    web::scope("/api")
        .service(index)
        .service(recipes::service())
        .service(cookbooks::service())
}
