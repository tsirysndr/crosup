use std::{io::BufRead, process::Stdio};

use super::Installer;
use anyhow::Error;
use owo_colors::OwoColorize;

const INSTALL_SCRIPT: &str =
    "curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";

pub struct HomebrewInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Default for HomebrewInstaller {
    fn default() -> Self {
        Self {
            name: "homebrew".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
        }
    }
}

impl HomebrewInstaller {
    pub fn setup_bashrc(&self) -> Result<(), Error> {
        println!("-> Setting up bashrc");
        println!(
            "-> Running {}",
            "echo 'export PATH=/home/linuxbrew/.linuxbrew/bin:$PATH' >> ~/.bashrc".bright_green()
        );
        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg("echo 'export PATH=/home/linuxbrew/.linuxbrew/bin:$PATH' >> ~/.bashrc")
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;
        if !output.status.success() {
            println!("-> Failed to setup bashrc");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg("Failed to setup bashrc"));
        }
        Ok(())
    }
}

impl Installer for HomebrewInstaller {
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

        let mut child = std::process::Command::new("bash")
            .env("NONINTERACTIVE", "true")
            .stdin(Stdio::from(curl.stdout.unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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

        child.wait()?;
        self.setup_bashrc()?;

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        let child = std::process::Command::new("brew")
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
        self.name.as_str()
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
}
