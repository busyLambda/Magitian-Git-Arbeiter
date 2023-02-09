mod services;

use actix_web::{web::scope, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(scope("/api")))
        .bind(("0.0.0.0", 8984))?
        .run()
        .await
}
