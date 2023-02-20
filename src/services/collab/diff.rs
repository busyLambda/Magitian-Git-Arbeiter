use actix_web::{get, web::Query, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
enum Show {
    BranchToBranch,
    CommitToCommit,
}

#[get("/show")]
async fn show(opts: Query<Show>) -> impl Responder {
    "shown!"
}
