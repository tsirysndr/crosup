use std::process::Stdio;

use crate::macros::{exec_bash, exec_bash_with_output};
use anyhow::Error;
use owo_colors::OwoColorize;
use std::io::BufRead;

use super::Installer;

pub struct DevenvInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for DevenvInstaller {
    fn default() -> Self {
        Self {
            name: "devenv".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["nix".to_string()],
            default: false,
        }
    }
}

impl DevenvInstaller {
    pub fn setup_cachix(&self) -> Result<(), Error> {
        println!(
            "   Running {}",
            "nix profile install nixpkgs#cachix".bright_green()
        );
        exec_bash!(". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix profile install nixpkgs#cachix");

        println!("   Running {}", "echo \"trusted-users = root $USER\" | sudo tee -a /etc/nix/nix.conf && sudo pkill nix-daemon".bright_green());
        let mut echo = std::process::Command::new("bash")
            .arg("-c")
            .arg("echo \"trusted-users = root $USER\"")
            .stdout(Stdio::piped())
            .spawn()?;
        echo.wait()?;

        let mut child = std::process::Command::new("sudo")
            .arg("tee")
            .arg("-a")
            .arg("/etc/nix/nix.conf")
            .stdin(Stdio::from(echo.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        child.wait()?;

        let mut child = std::process::Command::new("sudo")
            .arg("pkill")
            .arg("nix-daemon")
            .stdout(Stdio::piped())
            .spawn()?;
        child.wait()?;

        println!("   Running {}", "cachix use devenv".bright_green());
        exec_bash!("cachix use devenv");

        Ok(())
    }
}

impl Installer for DevenvInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name.bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());

        self.setup_cachix()?;

        println!(
            "   Running {}",
            "nix profile install --accept-flake-config github:cachix/devenv/latest".bright_green()
        );
        exec_bash_with_output!(". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix profile install --accept-flake-config github:cachix/devenv/latest");

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!("-> Checking if {} is installed", self.name().bright_green());
        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg(". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && devenv version")
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
