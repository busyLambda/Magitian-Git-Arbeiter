use actix_web::{get, Responder};

#[get("/tree/{path}")]
pub async fn tree() -> impl Responder {
    "tree"
}

#[get("/blob/{path}")]
pub async fn blob() -> impl Responder {
    "blob"
}