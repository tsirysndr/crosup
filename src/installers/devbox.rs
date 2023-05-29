use std::{io::BufRead, process::Stdio};

use anyhow::Error;
use owo_colors::OwoColorize;

use crate::macros::pipe_curl;

use super::Installer;

pub struct DevboxInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for DevboxInstaller {
    fn default() -> Self {
        Self {
            name: "devbox".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["nix".to_string()],
            default: true,
        }
    }
}

impl Installer for DevboxInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name.bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        println!(
            "   Running {}",
            "curl -fsSL https://get.jetpack.io/devbox | bash".bright_green()
        );
        let curl = std::process::Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://get.jetpack.io/devbox")
            .stdout(Stdio::piped())
            .spawn()?;

        pipe_curl!(curl);

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!("-> Checking if {} is installed", self.name().bright_green());
        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg("devbox version")
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;
        if !output.status.success() {
            println!("-> Failed to check devbox version");
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
