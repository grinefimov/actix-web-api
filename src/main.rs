use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder, Result};

async fn index() -> Result<impl Responder> {
    //println!("GET: /");
    Ok(HttpResponse::Ok().body("Hello world!"))
}

async fn again() -> Result<impl Responder> {
    //println!("GET: /again");
    Ok(HttpResponse::Ok().body("Hello world again!"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/again").to(again))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
