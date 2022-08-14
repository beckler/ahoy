use self_update::{backends::github::Update, cargo_crate_version};

// pub fn update_available() -> bool {
//     let status = self_update::version::bump_is_greater(current, other)
// }

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
    println!("update status: `{}`!", status.version());
    Ok(())
}
