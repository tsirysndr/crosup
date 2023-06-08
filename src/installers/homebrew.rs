use std::{any::Any, io::BufRead, process::Stdio};

use crate::macros::{check_version, pipe_brew_curl};

use super::Installer;
use anyhow::Error;
use owo_colors::OwoColorize;

const INSTALL_SCRIPT: &str =
    "curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";

pub struct HomebrewInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for HomebrewInstaller {
    fn default() -> Self {
        Self {
            name: "homebrew".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
            default: true,
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

        pipe_brew_curl!(curl);

        self.setup_bashrc()?;

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        check_version!(self, "brew --version");

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

    fn provider(&self) -> &str {
        ""
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
