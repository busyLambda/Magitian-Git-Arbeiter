use actix_web::{get, web::Path, HttpResponse, Responder};
use git2::{Oid, Repository};

use crate::extras::object::{TreeIterator, Component};

#[get("/tree/{path}")]
pub async fn tree() -> impl Responder {
    "tree"
}

// TODO: Add error handling and improve code
#[get("/{user_dir}/{repo_name}/blob/{path:.*}")]
pub async fn blob(path: Path<(String, String, String)>) -> impl Responder {
    let p = path.2.to_owned();
    /*let oid = find_blob_via_path(
        Repository::open(format!("git_test/{}/{}/", path.0, path.1)).unwrap(),
        &path.2,
        "master",
    )*/
    //.unwrap();
    //let contents = get_blob_contents(format!("git_test/{}/{}/", path.0, path.1), oid);
    let repo = Repository::open(format!("git_test/{}/{}/", path.0, path.1)).unwrap();
    let branch = repo.find_branch("master", git2::BranchType::Local).unwrap();
    let commit = repo.find_commit(branch.get().target().unwrap()).unwrap();
    let t = commit.tree().unwrap();
    let components = Component::from_string(p);

    let tri = TreeIterator::new(&repo, t, components);
    let contents = tri.filter_map(|r| r.ok()).flatten().last();

    HttpResponse::Ok().body(contents.unwrap())
}

fn get_blob_contents(repo_path: String, oid: Oid) -> String {
    let repo = Repository::open(repo_path).unwrap();
    let b = repo.find_blob(oid).unwrap();
    let contents = std::str::from_utf8(b.content()).unwrap();
    contents.to_string()
}

fn find_blob_via_path(repo: Repository, path: &String, branch_name: &str) -> Option<Oid> {
    let branch = repo
        .find_branch(branch_name, git2::BranchType::Local)
        .unwrap();
    let target = branch.get().target().unwrap();
    let commit = repo.find_commit(target).unwrap();
    let t = commit.tree().unwrap();

    for entry in t.iter() {
        println!("entry: {:?}", entry.name());
        if entry.name().unwrap() == path {
            return Some(entry.id());
        }
    }

    None
}
