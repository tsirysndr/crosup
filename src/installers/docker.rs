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
        println!(
            "   Running {}",
            "sudo install -m 0755 -d /etc/apt/keyrings".bright_green()
        );
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

        println!("   Running {}", "curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg".bright_green());
        let mut curl = std::process::Command::new("curl")
            .arg("-fsSL")
            .arg("https://download.docker.com/linux/debian/gpg")
            .stdout(Stdio::piped())
            .spawn()?;

        curl.wait()?;

        let gpg = std::process::Command::new("sudo")
            .arg("gpg")
            .arg("--dearmor")
            .arg("-o")
            .arg("/etc/apt/keyrings/docker.gpg")
            .stdin(Stdio::from(curl.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;

        let output = gpg.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to install GPG key");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to install GPG key")));
        }

        println!(
            "   Running {}",
            "sudo chmod a+r /etc/apt/keyrings/docker.gpg".bright_green()
        );

        let mut chmod = std::process::Command::new("sudo")
            .arg("chmod")
            .arg("a+r")
            .arg("/etc/apt/keyrings/docker.gpg")
            .stdout(Stdio::piped())
            .spawn()?;

        chmod.wait()?;

        return Ok(());
    }

    pub fn setup_repository(&self) -> Result<(), Error> {
        println!("-> Setting up repository");
        println!(
            "   Running {}",
            "echo \\
  \"deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \\
  \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\" | \\
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null".bright_green()
        );

        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg("echo \"deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\"")
            .stdout(Stdio::piped())
            .spawn()?;

        child.wait()?;

        let tee = std::process::Command::new("sudo")
            .arg("tee")
            .arg("/etc/apt/sources.list.d/docker.list")
            .stdin(Stdio::from(child.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;

        let output = tee.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to setup repository");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to setup repository")));
        }

        Ok(())
    }

    pub fn install_docker(&self) -> Result<(), Error> {
        println!("-> Installing docker packages");
        self.apt_update()?;

        println!(
            "   Running {}",
            "sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin".bright_green()
        );

        let mut child = std::process::Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("docker-ce")
            .arg("docker-ce-cli")
            .arg("containerd.io")
            .arg("docker-buildx-plugin")
            .arg("docker-compose-plugin")
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

    pub fn post_install(&self) -> Result<(), Error> {
        println!("-> Post install");
        println!(
            "   Running {}",
            "sudo usermod -aG docker $USER".bright_green()
        );

        let mut child = std::process::Command::new("sudo")
            .arg("usermod")
            .arg("-aG")
            .arg("docker")
            .arg("$USER")
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }

        child.wait()?;

        println!(" You need to logout and login again for the changes to take effect");
        println!(
            " You can also run {} to apply the changes",
            "newgrp docker".bright_green()
        );

        Ok(())
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

        self.apt_update()?;
        self.install_dependencies()?;
        self.install_gpg_key()?;
        self.setup_repository()?;
        self.install_docker()?;
        self.post_install()?;

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
            .spawn()?;
        let output = child.wait_with_output()?;
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
