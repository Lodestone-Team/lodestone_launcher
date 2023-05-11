use color_eyre::eyre::Result;
use semver::Version;
use std::path::PathBuf;

use crate::util;

fn get_release_url(version: &Version) -> Result<String> {
    // Get the target architecture and operating system
    let github_repo_url = "https://github.com/Lodestone-Team/lodestone_core/";
    let executable_name = util::get_executable_name(version)?;

    Ok(format!(
        "{}releases/download/v{}/{}",
        github_repo_url, version, executable_name
    ))
}

pub async fn download_release(version: &Version) -> Result<(PathBuf, String)> {
    // we try to backup the current core before downloading the new one
    // let lodestone_path = util::get_lodestone_path();
    // TODO: implement backup
    // let dest_dir = lodestone_path.join(PathBuf::from(".core_backup"));
    // copy_dir(&lodestone_path, &dest_dir)?;

    let executable_name = util::get_executable_name(version)?;
    let release_url = get_release_url(version)?;
    let executable_path = util::download_file(&release_url, &executable_name).await?;

    Ok((executable_path, executable_name))
}
