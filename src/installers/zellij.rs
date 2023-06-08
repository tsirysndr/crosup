use std::{any::Any, io::BufRead, process::Stdio};

use anyhow::Error;
use owo_colors::OwoColorize;

use crate::macros::{brew_install, check_version};

use super::Installer;

pub struct ZellijInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for ZellijInstaller {
    fn default() -> Self {
        Self {
            name: "zellij".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["homebrew".to_string()],
            default: true,
        }
    }
}

impl Installer for ZellijInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        brew_install!(self, "zellij");
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        check_version!(self, "zellij --version");
        Ok(true)
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
        self.default
    }

    fn provider(&self) -> &str {
        ""
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
