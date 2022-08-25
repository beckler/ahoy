use log::info;
use serde::{Deserialize, Serialize};
use std::env::temp_dir;
use std::fs::File;
use std::io::{copy, Cursor};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{GITHUB_API_URL, GITHUB_ORG, GITHUB_REPO};

use super::CommandError;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    pub discussion_url: Option<String>,
    pub id: u64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: Option<String>,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Asset {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
struct Query {
    per_page: u32,
    page: u32,
}

/// retrieve all available github releases
pub async fn fetch_releases() -> Result<Vec<Release>, CommandError> {
    info!("fetching releases from github...");
    let request = surf::get(format!(
        "{}/repos/{}/{}/releases",
        GITHUB_API_URL, GITHUB_ORG, GITHUB_REPO
    ))
    .query(&Query {
        per_page: 50,
        page: 1,
    })
    .unwrap()
    .recv_json::<Vec<Release>>();

    match request.await {
        Ok(res) => Ok(res),
        Err(err) => Err(err.into()),
    }
}

pub async fn fetch_asset(asset: Asset) -> Result<PathBuf, CommandError> {
    // download the binary
    info!("fetching asset from github: {}", asset.browser_download_url);
    match surf::get(asset.browser_download_url)
        .middleware(surf::middleware::Redirect::default())
        .await
    {
        Ok(mut response) => match response.body_bytes().await {
            Ok(body) => {
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
                        let mut content = Cursor::new(body);
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
