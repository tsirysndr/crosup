use anyhow::Error;
use crosup_macros::{check_version, exec_sh_with_output, zypper_install};
use crosup_types::zypper::Package;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::{any::Any, io::BufRead, process::Stdio};

use super::Installer;

#[derive(Default, Clone)]
pub struct ZypperInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub zypper_dependencies: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub non_interactive: bool,
    pub provider: String,
    pub session: Option<Session>,
}

impl From<Package> for ZypperInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            packages: pkg.packages,
            zypper_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "zypper".into(),
            version_check: pkg.version_check,
            non_interactive: pkg.non_interactive.unwrap_or(true),
            ..Default::default()
        }
    }
}

impl ZypperInstaller {
    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.zypper_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let deps = self.zypper_dependencies.join(" ");
        zypper_install!(deps, "--non-interactive", self.session.clone());
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

impl Installer for ZypperInstaller {
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
            let options = match self.non_interactive {
                true => "--non-interactive",
                false => "",
            };
            let packages = packages.join(" ");
            let command = format!("sudo zypper install {} {}", options, packages);
            println!("-> Running {}", command.bright_green());
            zypper_install!(packages, options, self.session.clone());
        }

        self.postinstall()?;
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        if let Some(command) = self.version_check.clone() {
            println!(
                "-> Checking if {} is already installed",
                self.name.bright_green()
            );
            check_version!(self, command, self.session.clone());
            return Ok(true);
        }
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
