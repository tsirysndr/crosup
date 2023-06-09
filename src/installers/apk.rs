use crate::{
    macros::{apk_add, check_version, exec_sh_with_output},
    types::apk::Package,
};
use anyhow::Error;
use owo_colors::OwoColorize;
use std::{any::Any, io::BufRead, process::Stdio};

use super::Installer;

#[derive(Default, Clone, Debug)]
pub struct ApkInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub apk_dependencies: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub interactive: bool,
    pub provider: String,
}

impl From<Package> for ApkInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            packages: pkg.packages,
            apk_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "apk".into(),
            version_check: pkg.version_check,
            interactive: pkg.interactive.unwrap_or(false),
            ..Default::default()
        }
    }
}

impl ApkInstaller {
    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.apk_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let deps = self.apk_dependencies.join(" ");
        apk_add!(deps, "");
        Ok(())
    }

    fn postinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.postinstall.clone() {
            println!(
                "-> Running postinstall command:\n{}",
                command.bright_green()
            );
            for cmd in command.split("\n") {
                exec_sh_with_output!(cmd);
            }
        }
        Ok(())
    }
}

impl Installer for ApkInstaller {
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
            let options = match self.interactive {
                true => "--interactive",
                false => "",
            };
            let packages = packages.join(" ");
            let command = format!("sudo apk add {} {}", options, packages);
            println!("-> Running {}", command.bright_green());
            apk_add!(packages, options);
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
            check_version!(self, command);
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
