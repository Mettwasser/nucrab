use std::path::PathBuf;

use tokio::fs;

use crate::cli::UninstallCommand;

pub async fn uninstall(command: UninstallCommand, nupm_home: PathBuf) -> anyhow::Result<()> {
    let UninstallCommand { source } = command;

    println!("uninstalling {}...", source);
    fs::remove_dir_all(nupm_home.join(&source)).await?;
    println!("{} has been uninstalled", source);
    Ok(())
}
