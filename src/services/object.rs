use actix_web::{get, web::Path, HttpResponse, Responder};
use git2::Repository;

use crate::extras::object::{Component, TreeIterator, BoTo};

#[get("/tree/{path}")]
pub async fn tree() -> impl Responder {
    "tree"
}

// TODO: Add error handling and improve code
#[get("/{user_dir}/{repo_name}/blob/{path:.*}")]
pub async fn blob(path: Path<(String, String, String)>) -> impl Responder {
    #[cfg(debug_assertions)]
    debug!("Getting blob at: {}", &path.2);

    let p = path.2.to_owned();
    let repo = Repository::open(format!("git_test/{}/{}/", path.0, path.1)).unwrap();
    let branch = repo.find_branch("master", git2::BranchType::Local).unwrap();
    let commit = repo.find_commit(branch.get().target().unwrap()).unwrap();
    let t = commit.tree().unwrap();
    let components = Component::from_string(p);

    let tri = TreeIterator::new(&repo, t, components);
    let resp = match tri.filter_map(|r| r.ok()).flatten().last().unwrap() {
        BoTo::Blob(blob) => HttpResponse::Ok().json(blob),
        _ => {
            HttpResponse::InternalServerError()
                .body("Found non blob item on the blob api endpoint, what?")
        }
    };
    resp
}
