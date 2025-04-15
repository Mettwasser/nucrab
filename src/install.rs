use std::path::PathBuf;

use anyhow::{
    Context,
    anyhow,
};
use indicatif::ProgressStyle;
use tokio::{
    fs,
    fs::File,
    io::AsyncWriteExt,
};

use crate::{
    cli::InstallCommand,
    models::{
        FromNuonValue,
        package_root::PackageRoot,
    },
};

pub async fn install(command: InstallCommand, nupm_home: PathBuf) -> anyhow::Result<()> {
    let InstallCommand { source, force } = command;

    let octocrab = octocrab::instance();
    let reqwest_client = reqwest::Client::new();

    let pb = indicatif::ProgressBar::new(2).with_style(ProgressStyle::with_template(
        "{msg} ({wide_bar}) {percent}%  ",
    )?);
    pb.set_message("Analyzing nupm.nuon...");

    let nuon = octocrab
        .repos(&source.owner, &source.repo)
        .get_content()
        .path("nupm.nuon")
        .send()
        .await?
        .items
        .first()
        .context("Couldn't find nupm.nuon in the package's root")?
        .decoded_content()
        .context("nupm.nuon couldn't be decoded")?;

    pb.inc(1);

    pb.set_message("Preparing package installation...");
    let value = nuon::from_nuon(&nuon, None)?;

    let pkg_root = PackageRoot::from_nuon_value(value)?;

    let pkg_module_path = nupm_home.join(&pkg_root.name);
    if pkg_module_path.exists() {
        if force {
            fs::remove_dir_all(&pkg_module_path).await?;
        } else {
            return Err(anyhow!(
                "package already installed. Use the --force flag to replace existing content."
            ));
        }
    }

    fs::create_dir_all(&pkg_module_path).await?;
    pb.inc(1);

    let mut path_stack = Vec::from([pkg_root.name.clone()]);
    while let Some(path) = path_stack.pop() {
        pb.set_message(format!("Downloading package content at {path}"));
        let repo_dir = octocrab
            .repos(&source.owner, &source.repo)
            .get_content()
            .path(path)
            .send()
            .await?;
        pb.set_length(repo_dir.items.len() as u64);
        pb.reset();

        for content in repo_dir.items {
            match content.r#type.as_str() {
                "dir" => {
                    let path = nupm_home.join(&content.path);
                    if !path.exists() {
                        fs::create_dir_all(path).await?;
                    }
                    path_stack.push(content.path);
                }

                "file" => {
                    pb.set_message(format!("Downloading file at {}", content.path));
                    let file_content = reqwest_client
                        .get(content.download_url.context("missing download_url")?)
                        .send()
                        .await?
                        .text()
                        .await?;

                    let path = nupm_home.join(&content.path);

                    let mut file = File::create(path).await?;

                    file.write_all(file_content.as_bytes()).await?;
                }

                _ => (),
            }

            pb.inc(1);
        }
    }

    pb.finish_with_message(format!("{} has been installed", pkg_root.name));

    Ok(())
}
