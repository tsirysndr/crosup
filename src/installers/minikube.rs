use std::{io::BufRead, process::Stdio};

use super::Installer;
use crate::macros::{brew_install, check_version};
use anyhow::Error;
use owo_colors::OwoColorize;

pub struct MinikubeInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for MinikubeInstaller {
    fn default() -> Self {
        Self {
            name: "minikube".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["homebrew".to_string(), "kubectl".to_string()],
            default: true,
        }
    }
}

impl MinikubeInstaller {
    pub fn install_dependencies(&self) -> Result<(), Error> {
        println!(
            "-> 🚚 Installing dependencies for {}",
            self.name().bright_green()
        );
        println!(
            "->   Running {}",
            "sudo apt install -y qemu-system libvirt-clients libvirt-daemon-system".bright_green()
        );
        let mut child = std::process::Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg("qemu-system")
            .arg("libvirt-clients")
            .arg("libvirt-daemon-system")
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }

        child.wait()?;

        Ok(())
    }
}

impl Installer for MinikubeInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        self.install_dependencies()?;
        brew_install!(self, "minikube");
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        check_version!(self, "minikube", "version");
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
