use actix_web::{get, Responder, web::Query, delete, HttpResponse};
use magitian_models::git::RawRepository;
use crate::extras::repository::create_repo;

use super::int_repr::queries::NewRepoQ;

#[get("/new")]
pub async fn new_repository(repo_form: Query<NewRepoQ>) -> impl Responder {
    let rf = repo_form.into_inner();

    match create_repo(&rf.user_dir, &rf.repo_name) {
        Ok(_) => HttpResponse::Ok().body(format!("User dir: {}, Repo name: {}", rf.user_dir, rf.repo_name)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Cannot create repository: {}", e)),
    }
}

#[delete("/delete")]
pub async fn delete_repository(repo_form: Query<NewRepoQ>) -> impl Responder {
    let repo_form = repo_form.into_inner();

    format!("User dir: {}, Repo name: {}", repo_form.user_dir, repo_form.repo_name)
}