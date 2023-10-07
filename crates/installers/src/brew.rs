use anyhow::Error;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::{any::Any, io::BufRead, process::Stdio};

use crosup_macros::{brew_install, check_version, exec_bash_with_output};
use crosup_types::brew::{BrewConfiguration, Package};

use super::Installer;

#[derive(Default, Clone)]
pub struct BrewInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub brew_dependencies: Vec<String>,
    pub pkgs: Vec<String>,
    pub preinstall: Option<String>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub provider: String,
    pub session: Option<Session>,
    pub cask: bool,
}

impl From<BrewConfiguration> for BrewInstaller {
    fn from(config: BrewConfiguration) -> Self {
        Self {
            name: "brew".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["homebrew".into()],
            pkgs: config.pkgs.unwrap_or(vec![]),
            ..Default::default()
        }
    }
}

impl From<Package> for BrewInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            version: "latest".to_string(),
            dependencies: vec!["homebrew".into()],
            preinstall: pkg.preinstall,
            postinstall: pkg.postinstall,
            provider: "brew".into(),
            version_check: pkg.version_check,
            cask: pkg.cask.unwrap_or(false),
            ..Default::default()
        }
    }
}

impl BrewInstaller {
    fn preinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.preinstall.clone() {
            println!("-> Running preinstall command:\n{}", command.bright_green());
            for cmd in command.split("\n") {
                exec_bash_with_output!(cmd, self.session.clone());
            }
        }
        Ok(())
    }

    fn postinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.postinstall.clone() {
            println!(
                "-> Running postinstall command:\n{}",
                command.bright_green()
            );
            for cmd in command.split("\n") {
                exec_bash_with_output!(cmd, self.session.clone());
            }
        }
        Ok(())
    }
}

impl Installer for BrewInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        self.preinstall()?;
        brew_install!(self, &self.name, self.cask, self.session.clone());
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
