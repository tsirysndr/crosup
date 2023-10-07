use anyhow::Error;
use crosup_macros::{check_version, exec_sh_with_output, fleek_install};
use crosup_types::fleek::Package;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::{any::Any, io::BufRead, path::Path, process::Stdio};

use super::Installer;

#[derive(Default, Clone)]
pub struct FleekInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub fleek_dependencies: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub provider: String,
    pub session: Option<Session>,
    pub apply: bool,
}

impl From<Package> for FleekInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            packages: pkg.packages,
            dependencies: vec!["nix".into()],
            fleek_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "fleek".into(),
            version_check: pkg.version_check,
            apply: pkg.apply.unwrap_or(true),
            ..Default::default()
        }
    }
}

impl FleekInstaller {
    pub fn fleek_init(&self) -> Result<(), Error> {
        println!("-> Checking if {} exists", "~/.fleek.yml".bright_green());

        let home = std::env::var("HOME").unwrap();
        let fleek_path = format!("{}/.fleek.yml", home);

        if Path::new(&fleek_path).exists() {
            println!("-> {} exists", "~/.fleek.yml".bright_green());
            return Ok(());
        }

        println!(
            "-> {} does not exist, please run {} first",
            "~/.fleek.yml".bright_green(),
            "nix run github:ublue-os/fleek -- init".bright_green()
        );
        return Err(anyhow::anyhow!("Fleek not initialized"));
    }

    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.fleek_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let deps = self.fleek_dependencies.join(" ");
        fleek_install!(deps, "--apply", self.session.clone());
        Ok(())
    }

    fn postinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.postinstall.clone() {
            println!(
                "-> Running postinstall command:\n{}",
                command.bright_green()
            );
            for cmd in command.split("\n") {
                exec_sh_with_output!(cmd, self.session.clone());
            }
        }
        Ok(())
    }
}

impl Installer for FleekInstaller {
    fn install(&self) -> Result<(), Error> {
        self.fleek_init()?;
        self.install_dependencies()?;

        if let Some(packages) = self.packages.clone() {
            let options = if self.apply { "--apply" } else { "" };
            let packages = packages.join(" ");
            let command = match self.apply {
                true => format!("nix run github:ublue-os/fleek add {} {}", options, packages),
                false => format!("nix run github:ublue-os/fleek add {}", packages),
            };
            println!("-> Running {}", command.bright_green());
            fleek_install!(packages, options, self.session.clone());
        }

        self.postinstall()?;
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        if let Some(command) = self.version_check.clone() {
            check_version!(self, command, self.session.clone());
            return Ok(false);
        }
        let command = self.name.clone();
        check_version!(self, command, self.session.clone());
        Ok(false)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn is_default(&self) -> bool {
        true
    }

    fn provider(&self) -> &str {
        &self.provider
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
