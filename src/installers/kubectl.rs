use std::{io::BufRead, process::Stdio};

use super::Installer;
use crate::macros::brew_install;
use anyhow::Error;
use owo_colors::OwoColorize;

pub struct KubectlInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for KubectlInstaller {
    fn default() -> Self {
        Self {
            name: "kubectl".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["homebrew".to_string()],
            default: true,
        }
    }
}

impl Installer for KubectlInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        brew_install!(self, "kubernetes-cli");
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        let child = std::process::Command::new("http")
            .arg("--version")
            .env(
                "PATH",
                "/home/linuxbrew/.linuxbrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
            )
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to check {} version", self.name.bright_green());
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to check {} version", self.name)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            println!("   {}", line.cyan());
        }

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
