use std::{io::BufRead, process::Stdio};

use anyhow::Error;
use owo_colors::OwoColorize;

use super::Installer;

const INSTALL_SCRIPT: &str =
    "curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish";

pub struct FisherInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Default for FisherInstaller {
    fn default() -> Self {
        Self {
            name: "fisher".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["fish".to_string()],
        }
    }
}

impl Installer for FisherInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }

        println!("-> 🚚 Installing {}", self.name().bright_green());
        let curl = std::process::Command::new("sh")
            .arg("-c")
            .arg(INSTALL_SCRIPT)
            .stdout(Stdio::piped())
            .spawn()?;

        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg("source")
            .stdin(Stdio::from(curl.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to install {}", self.name.bright_green());
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Ok(());
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));

        let mut child = std::process::Command::new("fisher")
            .arg("install")
            .arg("jorgebucaran/fisher")
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stdout = std::io::BufReader::new(stdout);
        for line in stdout.lines() {
            println!("   {}", line.unwrap());
        }

        let stderr = child.stderr.take().unwrap();
        let stderr = std::io::BufReader::new(stderr);
        for line in stderr.lines() {
            println!("   {}", line.unwrap());
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to install {}", self.name.bright_green());
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Ok(());
        }

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        let child = std::process::Command::new("fish")
            .arg("-c")
            .arg("fisher --version")
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
        self.name.as_str()
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
}
