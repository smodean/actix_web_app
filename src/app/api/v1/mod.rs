use actix_web::{web, Scope};

mod user;

pub fn routes_scope() -> Scope {
    web::scope("/v1")
        .service(user::routes_scope())
}