use actix_web::{
    middleware, web, /*web::block, */ App, HttpResponse, HttpServer,
    /*Responder, */ Result,
};

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
            // .route("", web::get().to_async(get_product_list))
            .route("/{product_id}", web::get().to_async(get_product)),
    );
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix-web-app!")
}

// async fn get_product_list() -> Result<HttpResponse> {
//     //authenticate(&state, &req)
//     let result = block(move || get_all(&pool)).await?;
//     match result {
//         Ok(products) => Ok(HttpResponse::Ok().json(products)),
//         Err(e) => Ok(e.error_response()),
//     }
// }

async fn get_product(id: web::Path<u32>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(format!("Product id: {}", id)))
}
