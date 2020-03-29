use actix_web::Responder;

pub async fn post() -> impl Responder {
    String::from("user/login::post")
}