use anyhow::Error;
use owo_colors::OwoColorize;
use std::{any::Any, io::BufRead, process::Stdio};

use crate::{
    macros::{apt_install, check_version, exec_bash, exec_bash_with_output, exec_sudo},
    types::apt::Package,
};

use super::Installer;

#[derive(Default, Clone, Debug)]
pub struct AptInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub apt_dependencies: Vec<String>,
    pub url: Option<String>,
    pub gpg_key: Option<String>,
    pub gpg_path: Option<String>,
    pub setup_repository: Option<String>,
    pub apt_update: Option<bool>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub provider: String,
}

impl From<Package> for AptInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            url: pkg.url,
            gpg_key: pkg.gpg_key,
            gpg_path: pkg.gpg_path,
            setup_repository: pkg.setup_repository,
            apt_update: pkg.apt_update,
            packages: pkg.packages,
            apt_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "apt".into(),
            version_check: pkg.version_check,
            ..Default::default()
        }
    }
}

impl AptInstaller {
    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.apt_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let deps = self.apt_dependencies.join(" ");
        apt_install!(deps);
        Ok(())
    }

    fn postinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.postinstall.clone() {
            println!(
                "-> Running postinstall command:\n{}",
                command.bright_green()
            );
            for cmd in command.split("\n") {
                exec_bash_with_output!(cmd);
            }
        }
        Ok(())
    }

    pub fn apt_update(&self) -> Result<(), Error> {
        if !self.apt_update.unwrap_or(false) {
            return Ok(());
        }

        println!("-> Running {}", "apt update".bright_green());
        let mut child = std::process::Command::new("sudo")
            .arg("apt")
            .arg("update")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }

        child.wait()?;

        Ok(())
    }

    pub fn install_from_url(&self) -> Result<(), Error> {
        let url = self.url.clone().unwrap();
        let package_name = format!("{}.deb", self.name.clone());

        let command = format!("wget -c {} -O {}.deb", url, package_name);
        println!("   Running {}", command.bright_green());
        let mut child = std::process::Command::new("wget")
            .arg("-c")
            .arg(url)
            .arg("-O")
            .arg(package_name.clone())
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

        let command = format!("sudo apt install -y ./{}", package_name);
        println!("   Running {}", command.bright_green());
        let mut child = std::process::Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg(format!("./{}", package_name))
            .stdout(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().unwrap();
        let stdout = std::io::BufReader::new(stdout);
        for line in stdout.lines() {
            println!("   {}", line.unwrap());
        }

        let command = format!("rm {}", package_name);
        println!("   Running {}", command.bright_green());
        let mut child = std::process::Command::new("rm")
            .arg(package_name)
            .stdout(Stdio::piped())
            .spawn()?;

        child.wait()?;

        println!(" Done! 🚀");
        Ok(())
    }
}

impl Installer for AptInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        let from_url = match self.url.clone() {
            Some(url) => format!("from {}", url),
            None => "".into(),
        };
        println!(
            "-> 🚚 Installing {} {}",
            self.name().bright_green(),
            from_url
        );

        if let Some(gpg_key) = self.gpg_key.clone() {
            if let Some(gpg_path) = self.gpg_path.clone() {
                println!(
                    "-> Adding GPG key {}",
                    "sudo install -m 0755 -d /etc/apt/keyrings".bright_green()
                );
                exec_sudo!("install -m 0755 -d /etc/apt/keyrings");
                exec_bash!(format!(
                    "curl -fsSL {} | sudo gpg --dearmor -o {}",
                    gpg_key, gpg_path
                ));
                exec_sudo!(format!("chmod a+r {}", gpg_path));
            }
        }

        if let Some(setup_repository) = self.setup_repository.clone() {
            println!("-> Adding repository {}", setup_repository.bright_green());
            exec_bash!(setup_repository);
        }

        self.apt_update()?;
        self.install_dependencies()?;

        if let Some(_) = self.url.clone() {
            self.install_from_url()?;
            self.postinstall()?;
            return Ok(());
        }

        apt_install!(self.name);
        self.postinstall()?;
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        if let Some(command) = self.version_check.clone() {
            println!(
                "-> Checking if {} is already installed",
                self.name.bright_green()
            );
            check_version!(self, command);
        }
        Ok(false)
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
        true
    }

    fn provider(&self) -> &str {
        &self.provider
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
