use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files as fs;
use std::env;

mod api;

pub async fn start() -> std::io::Result<()> {

    let app_address = env::var("APP_ADDRESS").unwrap();
    let app_port = env::var("APP_PORT").unwrap();

    let full_app_address = format!("{}:{}", app_address, app_port);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes)
    })
        .bind(&full_app_address)?
        .run();


    println!("You can access the server at {}", full_app_address);

    server.await
}

fn get_root_static() -> fs::Files {
    fs::Files::new("/", "static/").index_file("index.html")
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(
            api::routes_scope()
        )
        .service(get_root_static());
}