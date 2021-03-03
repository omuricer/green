use actix_web::{web, App, HttpServer};
use controller;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controller::hello)
            .service(controller::echo)
            .route("/hey", web::get().to(controller::manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
