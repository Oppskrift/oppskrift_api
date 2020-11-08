pub mod recipes_controller;
pub mod users_controller;

pub mod ping_controller {
    use actix_web::HttpResponse;

    #[get("/ping")]
    pub fn ping() -> HttpResponse {
        HttpResponse::Ok()
            .body("pong")
    }
}
