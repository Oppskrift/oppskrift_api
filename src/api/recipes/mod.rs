use actix_web::{web, Responder, Scope};

#[get("")]
async fn index() -> impl Responder {
    "Hello recipes"
}

pub fn service() -> Scope {
    web::scope("/recipes").service(index)
}
