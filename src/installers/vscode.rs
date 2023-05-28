use std::{io::BufRead, process::Stdio};

use super::Installer;
use anyhow::Error;
use owo_colors::OwoColorize;

pub struct VSCodeInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Default for VSCodeInstaller {
    fn default() -> Self {
        Self {
            name: "vscode".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
        }
    }
}

impl Installer for VSCodeInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }

        println!("-> 🚚 Installing {}", self.name().bright_green());
        println!("   Running {}", "wget -c https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64 -O vscode-installer.deb".bright_green());
        let mut child = std::process::Command::new("wget")
            .arg("-c")
            .arg("https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64")
            .arg("-O")
            .arg("vscode-installer.deb")
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stdout = std::io::BufReader::new(stdout);
        for line in stdout.lines() {
            println!("   {}", line.unwrap());
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to download {}", self.name().bright_green());
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to install {}", self.name())));
        }

        println!(
            "   Running {}",
            "sudo apt install ./vscode-installer.deb".bright_green()
        );
        let mut child = std::process::Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("./vscode-installer.deb")
            .stdout(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().unwrap();
        let stdout = std::io::BufReader::new(stdout);
        for line in stdout.lines() {
            println!("   {}", line.unwrap());
        }

        println!("   Running {}", "rm vscode-installer.deb".bright_green());
        let mut child = std::process::Command::new("rm")
            .arg("vscode-installer.deb")
            .stdout(Stdio::piped())
            .spawn()?;

        child.wait()?;

        println!(" Done! 🚀");
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        let child = std::process::Command::new("code")
            .arg("--version")
            .stdout(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to check {} version", self.name.bright_green());
            return Err(Error::msg(format!("Failed to check {} version", self.name)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            println!("   {}", line.cyan());
        }

        Ok(true)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
}
