use super::*;

pub mod post;

pub fn service() -> Scope {
    web::scope("/verify").service(post::route)
}
