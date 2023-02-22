use actix_web::{
    get,
    web::{Path, Query},
    HttpResponse, Responder,
};
use git2::{DiffOptions, Oid, Repository};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
struct Qy {
    opts: Show,
}

#[derive(Deserialize, Serialize, PartialEq)]
enum Show {
    #[serde(rename = "branch2branch")]
    BranchToBranch(FT),
    #[serde(rename = "commit2commit")]
    CommitToCommit(FT),
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct FT {
    from: String,
    to: String,
}

#[get("/diff/show/{user_dir}/{repo_dir}")]
pub async fn show(path: Path<(String, String)>, opts: Query<FT>) -> impl Responder {
    info!("asd");
    let repo = Repository::open(format!("git_test/{}/{}", path.0, path.1)).unwrap();
    let opts = opts.into_inner();

    let old_commit = repo
        .find_commit(Oid::from_str(&opts.from).unwrap())
        .unwrap();
    let new_commit = repo.find_commit(Oid::from_str(&opts.to).unwrap()).unwrap();

    let diff = repo
        .diff_tree_to_tree(
            Some(&old_commit.tree().unwrap()),
            Some(&new_commit.tree().unwrap()),
            None,
        )
        .unwrap();

    let mut diff_opts = DiffOptions::new();
    diff_opts.ignore_whitespace(false);

    let mut buffer = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        buffer.extend_from_slice(line.content());
        true
    })
    .unwrap();

    HttpResponse::Ok().body(buffer)
}

#[get("/test")]
pub async fn asd() -> impl Responder {
    "asd"
}
