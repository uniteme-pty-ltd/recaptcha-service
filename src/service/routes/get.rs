use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn route() -> impl Responder {
    HttpResponse::Ok()
}
