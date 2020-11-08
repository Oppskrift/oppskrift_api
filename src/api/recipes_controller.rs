use actix_web::{HttpResponse, web};

#[get("")]
pub fn list() -> HttpResponse {
    HttpResponse::Ok().body("list recipes")
}

#[post("")]
pub fn create() -> HttpResponse {
    HttpResponse::Created().finish()
}

#[get("/{id}")]
pub fn get(web::Path(id): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("get recipe #{}", id))
}
