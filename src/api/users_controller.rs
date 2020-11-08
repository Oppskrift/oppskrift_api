use actix_web::{HttpResponse, web};

#[get("")]
pub fn list() -> HttpResponse {
    HttpResponse::Ok().body("List users")
}

#[post("")]
pub fn create() -> HttpResponse {
    HttpResponse::Created().finish()
}

#[get("/{id}")]
pub fn get(web::Path(id): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Get user #{}", id))
}
