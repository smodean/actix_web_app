use actix_web::{Scope, web};

mod v1;

pub fn routes_scope() -> Scope {
    web::scope("/api")
        .service(v1::routes_scope())
}