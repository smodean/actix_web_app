use actix_web::{Scope, web};

mod login;
mod registration;

pub fn routes_scope() -> Scope {
    web::scope("/user")
        .service(
            web::resource("/login")
                .route(web::post().to(login::post))
        )
        .service(
            web::resource("/registration")
                .route(web::post().to(registration::post))
        )
}