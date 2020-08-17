mod app;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info"); //my_errors=debug,
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    app::start().await
}
