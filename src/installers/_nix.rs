use anyhow::Error;
use owo_colors::OwoColorize;
use std::{any::Any, io::BufRead, process::Stdio, vec};

use crate::{
    macros::{check_version, exec_bash_with_output},
    types::nix::Package,
};

use super::Installer;

#[derive(Default, Clone, Debug)]
pub struct NixInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub impure: Option<bool>,
    pub experimental_features: Option<String>,
    pub accept_flake_config: Option<bool>,
    pub preinstall: Option<String>,
    pub flake: String,
    pub version_check: Option<String>,
    pub provider: String,
}

impl From<Package> for NixInstaller {
    fn from(pkg: Package) -> Self {
        let mut dependencies = vec!["nix".into()];
        dependencies.extend(pkg.depends_on.unwrap_or(vec![]));
        Self {
            name: pkg.name,
            impure: pkg.impure,
            experimental_features: pkg.experimental_features,
            accept_flake_config: pkg.accept_flake_config,
            preinstall: pkg.preinstall,
            flake: pkg.flake,
            dependencies,
            version_check: pkg.version_check,
            provider: "nix".into(),
            ..Default::default()
        }
    }
}

impl NixInstaller {
    fn preinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.preinstall.clone() {
            println!("-> Running preinstall command:\n{}", command.bright_green());
            for cmd in command.split("\n") {
                exec_bash_with_output!(cmd);
            }
        }
        Ok(())
    }
}

impl Installer for NixInstaller {
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

        let impure = match self.impure {
            Some(impure) => match impure {
                true => "--impure",
                false => "",
            },
            None => "",
        };

        let experimental_features = match self.experimental_features.clone() {
            Some(features) => format!("--experimental-features \"{}\"", features),
            None => "".to_string(),
        };

        let accept_flake_config = match self.accept_flake_config {
            Some(accept) => match accept {
                true => "--accept-flake-config",
                false => "",
            },
            None => "",
        };

        let command = format!(
            r#". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && \
        nix profile install {} {} {} \
'{}'"#,
            impure, experimental_features, accept_flake_config, self.flake
        );
        exec_bash_with_output!(command);

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        if let Some(command) = self.version_check.clone() {
            println!(
                "-> Checking if {} is already installed",
                self.name.bright_green()
            );
            check_version!(self, command);
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
