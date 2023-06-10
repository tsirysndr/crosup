use anyhow::Error;
use owo_colors::OwoColorize;
use std::{any::Any, io::BufRead, path::Path, process::Stdio};

use crosup_macros::exec_bash_with_output;
use crosup_types::git::Repository;

use super::Installer;

#[derive(Default, Clone, Debug)]
pub struct GitInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub url: String,
    pub install: String,
    pub preinstall: Option<String>,
    pub postinstall: Option<String>,
    pub install_check: Option<String>,
    pub recursive: Option<bool>,
    pub depth: Option<u32>,
    pub shallow_submodules: Option<bool>,
    pub provider: String,
}

impl From<Repository> for GitInstaller {
    fn from(config: Repository) -> Self {
        Self {
            name: config.name,
            version: "latest".to_string(),
            dependencies: vec![],
            url: config.url,
            install: config.install,
            preinstall: config.preinstall,
            postinstall: config.postinstall,
            install_check: config.install_check,
            recursive: config.recursive,
            depth: config.depth,
            shallow_submodules: config.shallow_submodules,
            provider: "git".into(),
        }
    }
}

impl GitInstaller {
    fn preinstall(&self) -> Result<(), Error> {
        if let Some(command) = self.preinstall.clone() {
            println!("-> Running preinstall command:\n{}", command.bright_green());
            for cmd in command.split("\n") {
                exec_bash_with_output!(cmd);
            }
        }
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
}

impl Installer for GitInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", self.name().bright_green());
        self.preinstall()?;

        let repo_dir = self.url.split("/").last().unwrap();
        let repo_dir = repo_dir.replace(".git", "");

        if !Path::exists(Path::new(&repo_dir)) {
            println!(
                "-> Cloning {} into {}",
                self.url.bright_green(),
                repo_dir.bright_green()
            );
            let recursive = match self.recursive {
                Some(recursive) => match recursive {
                    true => "--recursive",
                    false => "",
                },
                None => "",
            };

            let depth = match self.depth {
                Some(depth) => format!("--depth {}", depth),
                None => "".to_string(),
            };

            let shallow_submodules = match self.shallow_submodules {
                Some(shallow_submodules) => match shallow_submodules {
                    true => "--shallow-submodules",
                    false => "",
                },
                None => "",
            };

            let git_clone = format!(
                "git clone {} {} {} {}",
                recursive, depth, shallow_submodules, self.url
            );

            let mut child = std::process::Command::new("sh")
                .arg("-c")
                .arg(git_clone)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;
            let output = child.stdout.take().unwrap();
            let output = std::io::BufReader::new(output);
            for line in output.lines() {
                println!("{}", line?);
            }
            let status = child.wait()?;

            if !status.success() {
                return Err(anyhow::anyhow!("Failed to install {}", self.name()));
            }
        } else {
            println!("-> {} already exists, skipping clone", repo_dir);
        }

        println!(
            "-> Running install command: {}",
            self.install.bright_green()
        );

        exec_bash_with_output!(self.install.clone());

        self.postinstall()?;
        Ok(())
    }

    fn is_installed(&self) -> Result<bool, Error> {
        if let Some(install) = self.install_check.clone() {
            println!(
                "-> Checking if {} is already installed",
                self.name.bright_green()
            );
            let home = std::env::var("HOME")?;
            let install = install.replace("~", &home);
            if Path::new(&install).exists() {
                return Ok(true);
            }
            return Err(anyhow::anyhow!("ble.sh is not installed"));
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
