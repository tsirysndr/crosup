use std::{io::BufRead, process::Stdio};

use anyhow::Error;
use owo_colors::OwoColorize;

use super::Installer;

pub struct TigInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Default for TigInstaller {
    fn default() -> Self {
        Self {
            name: "tig".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["homebrew".to_string()],
        }
    }
}

impl Installer for TigInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        let mut child = std::process::Command::new("brew")
            .arg("install")
            .arg("tig")
            .env(
                "PATH",
                "/home/linuxbrew/.linuxbrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
            )
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stdout = std::io::BufReader::new(stdout);
        for line in stdout.lines() {
            println!("   {}", line.unwrap());
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to install {}", self.name().bright_green());
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to install {}", self.name())));
        }

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        let child = std::process::Command::new("tig")
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
}
