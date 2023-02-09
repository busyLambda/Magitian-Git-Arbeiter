use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewRepoQ {
    pub user_dir: String,
    pub repo_name: String,
}