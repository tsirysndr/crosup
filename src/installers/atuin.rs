use std::{io::BufRead, process::Stdio};

use anyhow::Error;
use owo_colors::OwoColorize;

use crate::macros::{check_version, pipe_curl};

use super::Installer;

pub struct AtuinInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for AtuinInstaller {
    fn default() -> Self {
        Self {
            name: "atuin".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
            default: true,
        }
    }
}

impl Installer for AtuinInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name.bright_green()
            );
            return Ok(());
        }
        println!("-> Installing {}", "atuin".bright_green());
        println!(
            "   Running {}",
            "bash <(curl https://raw.githubusercontent.com/ellie/atuin/main/install.sh)"
                .bright_green()
        );
        let curl = std::process::Command::new("sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf -L https://raw.githubusercontent.com/ellie/atuin/main/install.sh")
            .stdout(Stdio::piped())
            .spawn()?;

        pipe_curl!(curl);

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!("-> Checking if {} is installed", self.name().bright_green());
        check_version!(self, "atuin", "--version");
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
}
