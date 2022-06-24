use log::*;
use octocrab::{models::repos::Release, Page};

use crate::{GITHUB_ORG, GITHUB_REPO};

use super::CommandError;

pub async fn retrieve_releases() -> Result<Page<Release>, CommandError> {
    info!("fetching releases from github...");
    let results = octocrab::instance()
        .repos(GITHUB_ORG, GITHUB_REPO)
        .releases()
        .list()
        .send()
        .await;

    trace!("{:?}", results);
    match results {
        Ok(result) => Ok(result),
        Err(err) => Err(CommandError::RetievalError(err.to_string())),
    }
}
