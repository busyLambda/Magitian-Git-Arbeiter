use crate::extras::repository::create_repo;
use actix_web::{delete, get, web::Query, HttpResponse, Responder};

use super::int_repr::queries::NewRepoQ;

#[get("/new")]
pub async fn new_repository(repo_form: Query<NewRepoQ>) -> impl Responder {
    let rf = repo_form.into_inner();

    match create_repo(&rf.user_dir, &rf.repo_name) {
        Ok(_) => HttpResponse::Ok().body(format!(
            "User dir: {}, Repo name: {}",
            rf.user_dir, rf.repo_name
        )),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Cannot create repository: {}", e))
        }
    }
}

#[delete("/delete")]
pub async fn delete_repository(repo_form: Query<NewRepoQ>) -> impl Responder {
    let repo_form = repo_form.into_inner();

    format!(
        "User dir: {}, Repo name: {}",
        repo_form.user_dir, repo_form.repo_name
    )
}

#[get("/fork")]
pub async fn fork() -> impl Responder {
    "fork"
}

#[get("/diffs")]
pub async fn get_diffs() -> impl Responder {
    "diffs"
}
