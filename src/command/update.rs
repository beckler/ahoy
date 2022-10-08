use log::info;
use self_update::{backends::github::Update, cargo_crate_version};

use super::CommandError;

pub async fn update_available() -> Result<bool, CommandError> {
    let updater = Update::configure()
        .repo_owner("beckler")
        .repo_name("ahoy")
        .bin_name("ahoy")
        .current_version(cargo_crate_version!())
        .build()
        .map_err(|e| CommandError::Retieval(format!("error creating for update builder: {}", e)))?;

    let latest = updater
        .get_latest_release()
        .map_err(|e| CommandError::Retieval(format!("could not check for updates: {}", e)))?;

    info!("latest release available: {}", latest.version);
    let is_greater = self_update::version::bump_is_greater(cargo_crate_version!(), &latest.version)
        .map_err(|e| CommandError::Retieval(format!("issue compairing versions: {}", e)))?;
    let is_compatible =
        self_update::version::bump_is_compatible(cargo_crate_version!(), &latest.version).map_err(
            |e| CommandError::Retieval(format!("issue checking compatable versions: {}", e)),
        )?;

    Ok(is_greater && is_compatible)
}

pub fn update_self(interactive: bool) -> Result<(), Box<dyn ::std::error::Error>> {
    let status = Update::configure()
        .repo_owner("beckler")
        .repo_name("ahoy")
        .bin_name("ahoy")
        .no_confirm(!interactive)
        .show_download_progress(interactive)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    info!("update status: `{}`!", status.version());
    Ok(())
}
