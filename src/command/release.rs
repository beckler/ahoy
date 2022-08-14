use std::env::temp_dir;
use std::fs::File;
use std::io::{copy, Cursor};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use log::{info, trace};
use octocrab::models::repos::{Asset, Release};

use crate::{GITHUB_ORG, GITHUB_REPO};

use super::CommandError;

/// retrieve all available github releases
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
        Err(err) => Err(CommandError::Retieval(err.to_string())),
    }
}

pub async fn fetch_asset(asset: Asset) -> Result<PathBuf, CommandError> {
    // download the binary
    info!("fetching asset from github...");
    match reqwest::get(asset.browser_download_url).await {
        Ok(response) => match response.bytes().await {
            Ok(content) => {
                // create timestamp
                let time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                // create temp file
                let temp_file_path = temp_dir().join(format!("{time}-{}", asset.name));
                info!("downloading file to: {}", temp_file_path.display());
                // create temp file
                match File::create(&temp_file_path) {
                    Ok(mut file) => {
                        let mut content = Cursor::new(content);
                        match copy(&mut content, &mut file) {
                            Ok(written) => {
                                info!("successfully downloaded - total bytes written: {}", written);
                                Ok(temp_file_path)
                            }
                            Err(err) => Err(CommandError::IO(err.to_string())),
                        }
                    }
                    Err(err) => Err(CommandError::IO(err.to_string())),
                }
            }
            Err(err) => Err(CommandError::Retieval(err.to_string())),
        },
        Err(err) => Err(CommandError::Retieval(err.to_string())),
    }
}
