use std::{io::BufRead, process::Stdio};

use super::Installer;
use anyhow::Error;
use owo_colors::OwoColorize;

#[derive(Clone)]
pub struct DockerInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Default for DockerInstaller {
    fn default() -> Self {
        Self {
            name: "docker".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
        }
    }
}

impl DockerInstaller {
    pub fn apt_update(&self) -> Result<(), Error> {
        println!("-> Running {}", "apt update".bright_green());
        let child = std::process::Command::new("sudo")
            .arg("apt")
            .arg("update")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to update apt");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to update apt")));
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));

        Ok(())
    }

    pub fn install_dependencies(&self) -> Result<(), Error> {
        println!("-> 🚚 Installing dependencies");
        let child = std::process::Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg("ca-certificates")
            .arg("curl")
            .arg("gnupg")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to install dependencies");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to install dependencies")));
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));

        Ok(())
    }

    pub fn install_gpg_key(&self) -> Result<(), Error> {
        println!("-> Installing GPG key");
        let child = std::process::Command::new("sudo")
            .arg("install")
            .arg("-m")
            .arg("0755")
            .arg("-d")
            .arg("/etc/apt/keyrings")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to install GPG key");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to install GPG key")));
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));

        let curl = std::process::Command::new("curl")
            .arg("curl")
            .arg("-fsSL")
            .arg("https://download.docker.com/linux/debian/gpg")
            .stdout(Stdio::piped())
            .spawn()?;

        let gpg = std::process::Command::new("gpg")
            .arg("--dearmor")
            .arg("-o")
            .arg("/etc/apt/keyrings/docker.gpg")
            .stdin(Stdio::from(curl.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;

        let output = gpg.wait_with_output().expect("failed to wait on child");

        if !output.status.success() {
            println!("-> Failed to install GPG key");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to install GPG key")));
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        return Ok(());
    }
}

impl Installer for DockerInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name.bright_green()
            );
            return Ok(());
        }

        println!("-> Installing {}", self.name.bright_green());
        let mut child = std::process::Command::new("brew")
            .arg("cask")
            .arg("install")
            .arg("docker")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
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
            return Err(Error::msg(format!("Failed to install {}", self.name)));
        }

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        let child = std::process::Command::new("docker")
            .arg("--version")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("failed to execute process");
        let output = child.wait_with_output().expect("failed to wait on child");
        if !output.status.success() {
            println!("-> Failed to check docker version");
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
        "docker"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
}
