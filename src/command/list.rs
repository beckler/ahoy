use log::*;
use octocrab::models::repos::Release;

use crate::{GITHUB_ORG, GITHUB_REPO};

use super::CommandError;

// map to custom error
pub async fn fetch_releases() -> Result<Vec<Release>, CommandError> {
    let fetch_all = async {
        info!("fetching releases from github...");
        // create crab instance
        let octocrab = octocrab::instance();
        // grab first page
        let page = octocrab
            .repos(GITHUB_ORG, GITHUB_REPO)
            .releases()
            .list()
            .per_page(50)
            .send()
            .await?;

        trace!("{:?}", page);

        // grab all pages. be warned there is no rate limiting here...
        octocrab.all_pages(page).await
    };

    match fetch_all.await {
        Ok(releases) => Ok(releases),
        Err(err) => Err(CommandError::RetievalError(err.to_string())),
    }
}
