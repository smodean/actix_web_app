use dotenv::dotenv;

mod app;

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // let sys = actix::System::new("conduit");

    app::start().await.ok();

    // sys.run();
}
