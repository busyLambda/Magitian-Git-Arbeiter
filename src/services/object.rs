use std::fmt::format;

use actix_web::{get, web::Path, HttpResponse, Responder};
use git2::{Oid, Repository};

#[get("/tree/{path}")]
pub async fn tree() -> impl Responder {
    "tree"
}

// TODO: Add error handling and improve code
#[get("/{user_dir}/{repo_name}/blob/{path}")]
pub async fn blob(path: Path<(String, String, String)>) -> impl Responder {
    let oid = Oid::from_str("2ba21893ef48c7a3a95798df8ba3d7d9e8fe9b27").unwrap();
    let contents = get_blob_contents(format!("git_test/{}/{}/", path.0, path.1), oid);

    HttpResponse::Ok().body(contents)
}

fn get_blob_contents(repo_path: String, oid: Oid) -> String {
    let repo = Repository::open(repo_path).unwrap();
    let b = repo.find_blob(oid).unwrap();
    let contents = std::str::from_utf8(b.content()).unwrap();
    contents.to_string()
}
