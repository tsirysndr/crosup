use anyhow::Error;
use indexmap::IndexMap;
use owo_colors::OwoColorize;
use std::{any::Any, io::BufRead, process::Stdio};

use crosup_macros::{check_version, exec_bash_with_output};
use crosup_types::curl::Script;

use super::Installer;

#[derive(Default, Clone, Debug)]
pub struct CurlInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub url: String,
    pub enable_sudo: Option<bool>,
    pub version_check: Option<String>,
    pub postinstall: Option<String>,
    pub args: Option<String>,
    pub env: Option<IndexMap<String, String>>,
    pub shell: String,
    pub provider: String,
}

impl From<Script> for CurlInstaller {
    fn from(config: Script) -> Self {
        Self {
            name: config.name,
            version: "latest".to_string(),
            dependencies: config.depends_on.unwrap_or(vec![]),
            url: config.url,
            enable_sudo: config.enable_sudo,
            version_check: config.version_check,
            postinstall: config.postinstall,
            args: config.args,
            env: config.env,
            shell: config.shell.unwrap_or("sh".into()),
            provider: "curl".into(),
        }
    }
}

impl CurlInstaller {
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

impl Installer for CurlInstaller {
    fn install(&self) -> Result<(), Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name().bright_green()
            );
            return Ok(());
        }

        println!("-> 🚚 Installing {}", self.name().bright_green());

        let sudo = match self.enable_sudo {
            Some(true) => "sudo ",
            _ => "",
        };

        let script = match self.args {
            Some(ref args) => format!(
                "curl --proto '=https' --tlsv1.2 -sSf -L {} | {}{} -s -- {}",
                self.url, sudo, self.shell, args
            ),
            None => format!(
                "curl --proto '=https' --tlsv1.2 -sSf -L {} | {}{}",
                self.url, sudo, self.shell
            ),
        };

        println!("   Running {}", script.bright_green());

        let mut child = std::process::Command::new(&self.shell)
            .arg("-c")
            .arg(script)
            .envs(self.env.clone().unwrap_or_default())
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }
        let status = child.wait()?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Failed to install {}",
                self.name.bright_green()
            ));
        }

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
