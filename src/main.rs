use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::Context;
use clap::Parser;
use nucrab::{
    cli::CLI,
    install,
    uninstall,
};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    let nupm_home_env = env::var("NUPM_HOME").context("NUPM_HOME env var not found or faulty")?;
    let nupm_home = Path::new(&nupm_home_env).join("modules");

    match cli {
        CLI::Install(cmd) => install::install(cmd, nupm_home).await,
        CLI::Uninstall(cmd) => uninstall::uninstall(cmd, nupm_home).await,
        CLI::List => list(nupm_home).await,
    }
}

pub async fn list(nupm_home: PathBuf) -> anyhow::Result<()> {
    let mut entries = fs::read_dir(nupm_home).await?;

    let mut install_count = 0;

    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry.path().is_dir() {
            println!("- {}", entry.file_name().to_string_lossy());
            install_count += 1;
        }
    }

    println!();

    if install_count == 0 {
        println!("no packages installed");
    } else {
        println!("{} packages installed", install_count);
    }

    Ok(())
}
