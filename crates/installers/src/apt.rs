use anyhow::Error;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::{any::Any, io::BufRead, process::Stdio};

use crosup_macros::{apt_install, check_version, exec_bash, exec_bash_with_output, exec_sudo};
use crosup_types::apt::Package;

use super::Installer;

#[derive(Default, Clone)]
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
    pub session: Option<Session>,
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
        apt_install!(deps, self.session.clone());
        Ok(())
    }

    fn postinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.postinstall.clone() {
            println!(
                "-> Running postinstall command:\n{}",
                command.bright_green()
            );
            for cmd in command.split("\n") {
                exec_bash_with_output!(cmd, self.session.clone());
            }
        }
        Ok(())
    }

    pub fn apt_update(&self) -> Result<(), Error> {
        if !self.apt_update.unwrap_or(false) {
            return Ok(());
        }

        println!("-> Running {}", "apt-get update".bright_green());

        if let Some(session) = &self.session {
            crosup_ssh::exec(session.clone(), "sudo apt-get update")?;
            return Ok(());
        }

        let mut child = std::process::Command::new("sudo")
            .arg("apt-get")
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

        let command = format!("wget -c {} -O {}", url, package_name);
        println!("   Running {}", command.bright_green());

        if let Some(session) = &self.session {
            crosup_ssh::exec(session.clone(), &command)?;
            return Ok(());
        }

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
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            return Err(Error::msg(format!("Failed to install {}", self.name())));
        }

        let command = format!("sudo apt-get install -y ./{}", package_name);
        println!("   Running {}", command.bright_green());

        if let Some(session) = &self.session {
            crosup_ssh::exec(session.clone(), &command)?;
            return Ok(());
        }

        let mut child = std::process::Command::new("sudo")
            .arg("apt-get")
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

        if let Some(session) = &self.session {
            crosup_ssh::exec(session.clone(), &command)?;
            return Ok(());
        }

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
                exec_sudo!("install -m 0755 -d /etc/apt/keyrings", self.session.clone());
                exec_bash!(
                    format!(
                        "curl -fsSL {} | sudo gpg --dearmor -o {}",
                        gpg_key, gpg_path
                    ),
                    self.session.clone()
                );
                exec_sudo!(format!("chmod a+r {}", gpg_path), self.session.clone());
            }
        }

        if let Some(setup_repository) = self.setup_repository.clone() {
            println!("-> Adding repository {}", setup_repository.bright_green());
            exec_bash!(setup_repository, self.session.clone());
        }

        self.apt_update()?;
        self.install_dependencies()?;

        if let Some(_) = self.url.clone() {
            self.install_from_url()?;
            self.postinstall()?;
            return Ok(());
        }

        if let Some(packages) = self.packages.clone() {
            let packages = packages.join(" ");
            let command = format!("sudo apt-get install -y {}", packages);
            println!("-> Running {}", command.bright_green());
            apt_install!(packages, self.session.clone());
        }

        self.postinstall()?;
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        println!(
            "-> Checking if {} is already installed",
            self.name.bright_green()
        );
        if let Some(command) = self.version_check.clone() {
            check_version!(self, command, self.session.clone());
            return Ok(false);
        }
        let command = self.name.clone();
        check_version!(self, command, self.session.clone());
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
