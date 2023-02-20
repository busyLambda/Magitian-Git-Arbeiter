use actix_web::{get, Responder};

/// Merge a branch into another branch in the same or fork repositories!
#[get("/merge")]
async fn merge() -> impl Responder {
    "Merged!"
}

/// If the merge wouldn't raise any conflicts we put a seal of approval on the merge request.
/// Otherwise we raise the appropriate error and highlight the issues with the merge request
/// that require manual fixes.
#[get("/merge_vanguard")]
async fn merge_vanguard() -> impl Responder {
    "Looks mergeable!"
}
