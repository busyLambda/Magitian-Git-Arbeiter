use magitian_models::git::RawRepository;

pub fn create_repo(user_dir: &String, repo_name: &String) -> Result<(), git2::Error> {
    match std::fs::create_dir(format!("git_test/{}/{}", user_dir, repo_name)) {
        Ok(d) => {d},
        _ => return Err(git2::Error::from_str("Cannot create directory.")),
    };
    let repo = RawRepository::init(format!("git_test/{}/{}", user_dir, repo_name))?;

    assert!(repo.is_empty()?);

    Ok(())
}
