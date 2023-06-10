use anyhow::Error;
use crosup_macros::{check_version, exec_sh_with_output, yum_install};
use crosup_types::yum::Package;
use owo_colors::OwoColorize;
use std::{any::Any, io::BufRead, process::Stdio};

use super::Installer;

#[derive(Default, Clone, Debug)]
pub struct YumInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub yum_dependencies: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub provider: String,
}

impl From<Package> for YumInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            packages: pkg.packages,
            yum_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "yum".into(),
            version_check: pkg.version_check,
            ..Default::default()
        }
    }
}

impl YumInstaller {
    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.yum_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let deps = self.yum_dependencies.join(" ");
        yum_install!(deps);
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

impl Installer for YumInstaller {
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
            let command = format!("sudo yum install -y {}", packages);
            println!("-> Running {}", command.bright_green());
            yum_install!(packages);
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
