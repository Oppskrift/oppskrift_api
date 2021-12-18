use actix_web::{web, Responder, Scope};

#[get("")]
async fn index() -> impl Responder {
    "Hello cookbook"
}

pub fn service() -> Scope {
    web::scope("/cookbooks").service(index)
}
