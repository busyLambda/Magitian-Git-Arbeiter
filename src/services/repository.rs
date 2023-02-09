use actix_web::{get, post, Responder, web::Query, delete};
use super::int_repr::queries::NewRepoQ;

#[get("/new")]
pub async fn new_repository(repo_form: Query<NewRepoQ>) -> impl Responder {
    let repo_form = repo_form.into_inner();

    format!("User dir: {}, Repo name: {}", repo_form.user_dir, repo_form.repo_name)
}
#[delete("/new")]
pub async fn delete_repository(repo_form: Query<NewRepoQ>) -> impl Responder {
    let repo_form = repo_form.into_inner();

    format!("User dir: {}, Repo name: {}", repo_form.user_dir, repo_form.repo_name)
}