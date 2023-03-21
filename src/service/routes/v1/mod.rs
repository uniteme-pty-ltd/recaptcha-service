use super::*;

mod verify;

pub fn service() -> Scope {
    web::scope("/v1").service(verify::service())
}
