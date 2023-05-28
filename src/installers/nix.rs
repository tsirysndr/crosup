use std::{io::BufRead, process::Stdio};

use super::Installer;
use anyhow::Error;
use owo_colors::OwoColorize;

const INSTALL_SCRIPT: &str =
    "curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix";

pub struct NixInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Default for NixInstaller {
    fn default() -> Self {
        Self {
            name: "nix".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
        }
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

        println!("-> 🚚 Installing {}", self.name.bright_green());
        let curl = std::process::Command::new("sh")
            .arg("-c")
            .arg(INSTALL_SCRIPT)
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        let mut child = std::process::Command::new("sudo")
            .arg("sh")
            .arg("-s")
            .arg("--")
            .arg("install")
            .arg("--no-confirm")
            .stdin(Stdio::from(curl.stdout.unwrap()))
            .stdout(Stdio::piped())
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
        println!("-> Checking if {} is already installed", self.name);
        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg(". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix --version")
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
        "nix"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
}
