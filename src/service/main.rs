use actix_web::{App, HttpServer};
use routes::*;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || App::new().service(v1::service()).service(get::route))
        .workers(num_cpus::get())
        .bind(("0.0.0.0", std::env::var("PORT").expect("PORT not set").parse().expect("PORT was not u16")))?
        .run()
        .await
}
