use std::io::BufRead;

use anyhow::Error;
use owo_colors::OwoColorize;

use super::Installer;

pub struct AtuinInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Default for AtuinInstaller {
    fn default() -> Self {
        Self {
            name: "atuin".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
        }
    }
}

impl Installer for AtuinInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name.bright_green()
            );
            return Ok(());
        }
        println!("-> Installing {}", "atuin".bright_green());
        println!(
            "   Running {}",
            "bash <(curl https://raw.githubusercontent.com/ellie/atuin/main/install.sh)"
                .bright_green()
        );
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg("bash <(curl https://raw.githubusercontent.com/ellie/atuin/main/install.sh)")
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!("-> Checking if {} is installed", self.name().bright_green());
        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg("atuin --version")
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;
        if !output.status.success() {
            println!("-> Failed to check atuin version");
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
