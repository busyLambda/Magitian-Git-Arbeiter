mod services;
mod extras;

use std::io::Read;

use actix_web::{web::scope, App, HttpServer, middleware::Logger, Responder, get, HttpResponse};

use services::repository::new_repository;

#[macro_use]
extern crate log;
extern crate env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server on 0.0.0.0:8984");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(scope("/api")
                .service(scope("repository")
                    .service(new_repository)
            )
        )
        }).bind(("0.0.0.0", 8984))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    let mut file = std::fs::File::open("index.html").unwrap();
    let mut html = String::new();
    file.read_to_string(&mut html).unwrap();

    HttpResponse::Ok().body(html)
}