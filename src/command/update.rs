use log::{debug, info, trace};
use self_update::{backends::github::Update, cargo_crate_version};

use super::CommandError;

pub async fn update_available() -> Result<Option<String>, CommandError> {
    let updater = Update::configure()
        .repo_owner("beckler")
        .repo_name("ahoy")
        .bin_name("ahoy")
        .current_version(cargo_crate_version!())
        .build()
        .map_err(|e| CommandError::Update(format!("error creating for update builder: {}", e)))?;

    let latest = updater
        .get_latest_release()
        .map_err(|e| CommandError::Update(format!("could not check for updates: {}", e)))?;

    trace!("release response: {:?}", latest);
    info!("current version: {}", cargo_crate_version!());
    info!("latest release available: {}", latest.version);
    let is_greater = self_update::version::bump_is_greater(cargo_crate_version!(), &latest.version)
        .map_err(|e| CommandError::Update(format!("issue compairing versions: {}", e)))?;

    debug!("is new release greater? - {}", is_greater);

    if is_greater {
        return Ok(Some(latest.version));
    }
    Ok(None)
}

pub async fn update_self(interactive: bool) -> Result<(), CommandError> {
    let status = Update::configure()
        .repo_owner("beckler")
        .repo_name("ahoy")
        .bin_name("ahoy")
        .no_confirm(!interactive)
        .show_output(interactive)
        .show_download_progress(interactive)
        .current_version(cargo_crate_version!())
        .build()
        .map_err(|e| CommandError::Update(format!("unable to build updater: {}", e)))?
        .update()
        .map_err(|e| CommandError::Update(format!("unable to update: {}", e)))?;
    info!("update status: `{}`!", status.version());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::command::update::update_available;

    #[async_std::test]
    async fn test_update_status() -> std::io::Result<()> {
        match update_available().await {
            Ok(status) => print!("status: {:?}", status),
            Err(err) => print!("error: {}", err),
        }
        Ok(())
    }
}
