use anyhow::Error;
use crosup_macros::{check_version, exec_sh_with_output, pacman_install};
use crosup_types::pacman::Package;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::{any::Any, io::BufRead, process::Stdio};

use super::Installer;

#[derive(Default, Clone)]
pub struct PacmanInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub pacman_dependencies: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub non_interactive: bool,
    pub provider: String,
    pub session: Option<Session>,
}

impl From<Package> for PacmanInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            packages: pkg.packages,
            pacman_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "pacman".into(),
            version_check: pkg.version_check,
            ..Default::default()
        }
    }
}

impl PacmanInstaller {
    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.pacman_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let deps = self.pacman_dependencies.join(" ");
        pacman_install!(deps, self.session.clone());
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

impl Installer for PacmanInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            if self.is_installed().unwrap() {
                println!(
                    "-> {} is already installed, skipping",
                    self.name().bright_green()
                );
                return Ok(());
            }
        }

        self.install_dependencies()?;

        if let Some(packages) = self.packages.clone() {
            let packages = packages.join(" ");
            let command = format!("sudo pacman -S {}", packages);
            println!("-> Running {}", command.bright_green());
            pacman_install!(packages, self.session.clone());
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
