use std::{any::Any, io::BufRead, process::Stdio};

use super::Installer;
use crate::macros::{brew_install, check_version};
use anyhow::Error;
use owo_colors::OwoColorize;

pub struct HttpieInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for HttpieInstaller {
    fn default() -> Self {
        Self {
            name: "httpie".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["homebrew".to_string()],
            default: true,
        }
    }
}

impl Installer for HttpieInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        brew_install!(self, "httpie");
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        check_version!(self, "http --version");
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
