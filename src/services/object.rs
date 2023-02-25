use actix_web::{get, web::Path, HttpResponse, Responder};
use git2::Repository;

use crate::extras::object::{BoTo, Component, TreeIterator, TrObject};

// FIXME: It panics if the route end in '/'
#[get("/{user_dir}/{repo_name}/tree/{path:.*}")]
pub async fn tree(path: Path<(String, String, String)>) -> impl Responder {
    let p: String;
    let repo = Repository::open(format!("git_test/{}/{}/", path.0, path.1)).unwrap();
    let branch = repo.find_branch("master", git2::BranchType::Local).unwrap();
    let commit = repo.find_commit(branch.get().target().unwrap()).unwrap();
    let t = commit.tree().unwrap();
    let p  = path.2.to_owned();
    let components = Component::from_string(p);

    let tri = TreeIterator::new(&repo, t, components);
    let resp = match tri.filter_map(|r| r.ok()).flatten().last().unwrap() {
        BoTo::Tree(tree) => HttpResponse::Ok().json(tree),
        _ => HttpResponse::InternalServerError()
            .body("Found non tree item on the tree api endpoint, what?"),
    };
    resp
}

#[get("/{user_dir}/{repo_name}/tree")]
pub async fn root(path: Path<(String, String)>) -> impl Responder {
    let repo = Repository::open(format!("git_test/{}/{}/", path.0, path.1)).unwrap();
    let branch = repo.find_branch("master", git2::BranchType::Local).unwrap();
    let commit = repo.find_commit(branch.get().target().unwrap()).unwrap();
    let t = commit.tree().unwrap();

    let trobject = TrObject::from_tree(t, "/".to_owned()).unwrap();
    HttpResponse::Ok().json(trobject)
}

// TODO: move error handling into a different function.
#[get("/{user_dir}/{repo_name}/blob/")]
pub async fn blob(path: Path<(String, String, String)>) -> impl Responder {
    #[cfg(debug_assertions)]
    debug!("Getting blob at: {}", &path.2);

    let p = path.2.to_owned();
    let repo = match Repository::open(format!("git_test/{}/{}/", path.0, path.1)) {
        Ok(r) => r,
        Err(e) => return HttpResponse::NotFound().body(format!("Cannot find repository: {}", e)),
    };
    let branch = match repo.find_branch("master", git2::BranchType::Local) {
        Ok(b) => b,
        Err(e) => return HttpResponse::NotFound().body(format!("Cannot find branch: {}", e)),
    };
    let commit = match repo.find_commit(match branch.get().target() {
        Some(b) => b,
        None => return HttpResponse::NotFound().body(format!("Cannot get commit from branch")),
    }) {
        Ok(c) => c,
        Err(e) => return HttpResponse::NotFound().body(format!("Cannot find commit: {}", e)),
    };
    let t = match commit.tree() {
        Ok(t) => t,
        Err(te) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to get tree from commit: {}", te))
        }
    };
    let components = Component::from_string(p);

    let tri = TreeIterator::new(&repo, t, components);
    let resp = match match tri.filter_map(|r| r.ok()).flatten().last() {
        Some(resp) => resp,
        None => return HttpResponse::InternalServerError().body(format!("Failed to walk tree")),
    } {
        BoTo::Blob(blob) => HttpResponse::Ok().json(blob),
        _ => HttpResponse::InternalServerError()
            .body("Found non blob item on the blob api endpoint, what?"),
    };
    resp
}
