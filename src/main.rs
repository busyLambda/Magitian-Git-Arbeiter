mod extras;
mod services;

use std::io::Read;

use actix_web::{get, middleware::Logger, web::scope, App, HttpResponse, HttpServer, Responder};

use services::{
    collab::diff::show,
    object::{blob, tree},
    repository::new_repository,
};

#[macro_use]
extern crate log;
extern crate env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "actix_web=info");
    #[cfg(debug_assertions)]
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    #[cfg(not(debug_assertions))]
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Starting server on 0.0.0.0:8984");

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(index).service(
            scope("/api")
                .service(scope("repository").service(new_repository))
                .service(scope("collab").service(show))
                .service(scope("object").service(blob).service(tree)),
        )
    })
    .bind(("0.0.0.0", 8984))?
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
