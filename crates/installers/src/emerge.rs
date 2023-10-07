use anyhow::Error;
use crosup_macros::{check_version, emerge_install, exec_sh_with_output};
use crosup_types::emerge::Package;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::{any::Any, io::BufRead, process::Stdio};

use super::Installer;

#[derive(Default, Clone)]
pub struct EmergeInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub emerge_dependencies: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub ask: bool,
    pub verbose: bool,
    pub provider: String,
    pub session: Option<Session>,
}

impl From<Package> for EmergeInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            packages: pkg.packages,
            emerge_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "emerge".into(),
            version_check: pkg.version_check,
            ask: pkg.ask.unwrap_or(false),
            verbose: pkg.verbose.unwrap_or(false),
            ..Default::default()
        }
    }
}

impl EmergeInstaller {
    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.emerge_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let deps = self.emerge_dependencies.join(" ");
        emerge_install!(deps, "--ask --verbose", self.session.clone());
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

impl Installer for EmergeInstaller {
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
            let options = match self.ask {
                true => "--ask",
                false => "",
            };
            let options = match self.verbose {
                true => format!("{} --verbose", options),
                false => format!("{}", options),
            };
            let packages = packages.join(" ");
            let command = format!("sudo emerge install {} {}", options, packages);
            println!("-> Running {}", command.bright_green());
            emerge_install!(packages, options, self.session.clone());
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
