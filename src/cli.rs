use std::str::FromStr;

use clap::{
    Args,
    Parser,
};

#[derive(Debug, Clone)]
pub struct GithubPackageSource {
    pub repo: String,
    pub owner: String,
}

impl FromStr for GithubPackageSource {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_string = s.split('/').collect::<Vec<_>>();

        let owner = (*split_string
            .first()
            .ok_or("no owner provided in the target string")?)
        .to_owned();

        let repo = (*split_string
            .get(1)
            .ok_or("No name for the repository in the target string")?)
        .to_owned();

        Ok(Self { owner, repo })
    }
}

#[derive(Debug, Parser)]
#[clap(version, about)]
pub enum CLI {
    /// Install a package from a github repo
    Install(InstallCommand),

    /// Uninstall a package
    Uninstall(UninstallCommand),

    /// List all installed packages
    List,
}

#[derive(Debug, Args)]
pub struct InstallCommand {
    /// The package source on github
    #[clap(help = "format is `owner/repo`")]
    pub source: GithubPackageSource,

    /// Replace EVERYTHING, in case the package is already installed
    #[clap(long, short)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct UninstallCommand {
    /// The name of the package to uninstall
    pub source: String,
}
