use crate::db;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use r2d2_sqlite::{self, SqliteConnectionManager};

pub async fn start() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("aws.db");
    let pool = db::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::new("%a %t %{Host}i \"%r\" %s %bB %Dms"))
            .configure(routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").to(index)).service(
        web::scope("/products")
            .route("", web::get().to(get_product_list))
            .route("/{id}", web::get().to(get_product)),
    );
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix-web-api!")
}

async fn get_product_list(pool: web::Data<db::Pool>) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let products = web::block(move || db::select_products(conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(products))
}

async fn get_product(id: web::Path<u32>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(format!("Product id: {}", id)))
}
