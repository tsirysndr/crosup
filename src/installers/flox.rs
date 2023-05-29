use std::{io::BufRead, process::Stdio};

use anyhow::Error;
use owo_colors::OwoColorize;

use crate::macros::append_to_nix_conf;

use super::Installer;

pub struct FloxInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for FloxInstaller {
    fn default() -> Self {
        Self {
            name: "flox".to_string(),
            version: "latest".to_string(),
            dependencies: vec!["nix".to_string()],
            default: false,
        }
    }
}

impl FloxInstaller {
    fn configure_substituers(&self) -> Result<(), Error> {
        println!("-> 🛠 Configuring substituters");
        println!(
            "   Running {}",
            r#"echo 'extra-trusted-substituters = https://cache.floxdev.com' | sudo tee -a /etc/nix/nix.conf
            echo 'extra-trusted-public-keys = flox-store-public-0:8c/B+kjIaQ+BloCmNkRUKwaVPFWkriSAd0JJvuDu4F0=' | sudo tee -a /etc/nix/nix.conf"#.bright_green()
        );
        let echo = std::process::Command::new("bash")
            .arg("-c")
            .arg("echo 'extra-trusted-substituters = https://cache.floxdev.com'")
            .stdout(Stdio::piped())
            .spawn()?;
        append_to_nix_conf!(echo);

        let echo = std::process::Command::new("bash")
            .arg("-c")
            .arg("echo 'extra-trusted-public-keys = flox-store-public-0:8c/B+kjIaQ+BloCmNkRUKwaVPFWkriSAd0JJvuDu4F0='")
            .stdout(Stdio::piped())
            .spawn()?;
        append_to_nix_conf!(echo);
        Ok(())
    }
}

impl Installer for FloxInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name.bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        self.configure_substituers()?;
        println!(
            "   Running {}",
            r#"sudo -H nix profile install --impure \
        --profile /nix/var/nix/profiles/default \
        --experimental-features "nix-command flakes" \
        --accept-flake-config \
        'github:flox/floxpkgs#flox.fromCatalog'"#
                .bright_green()
        );
        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg(
                r#". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && \
                sudo -H nix profile install --impure \
            --profile /nix/var/nix/profiles/default \
            --experimental-features "nix-command flakes" \
            --accept-flake-config \
            'github:flox/floxpkgs#flox.fromCatalog'"#,
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
        println!("-> Checking if {} is installed", self.name().bright_green());
        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg(". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && flox --version")
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;
        if !output.status.success() {
            println!("-> Failed to check {} version", self.name());
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
