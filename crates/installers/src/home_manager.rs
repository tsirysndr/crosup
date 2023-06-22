use anyhow::Error;
use crosup_macros::{check_version, exec_bash_with_output, home_manager_init};
use crosup_types::home_manager::Package;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::{any::Any, fs, io::BufRead, path::Path, process::Stdio};

use crate::home_manager;

use super::Installer;

#[derive(Default, Clone)]
pub struct HomeManagerInstaller {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub hm_dependencies: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub postinstall: Option<String>,
    pub version_check: Option<String>,
    pub provider: String,
    pub session: Option<Session>,
    pub apply: bool,
}

impl From<Package> for HomeManagerInstaller {
    fn from(pkg: Package) -> Self {
        Self {
            name: pkg.name,
            packages: pkg.packages,
            dependencies: vec!["nix".into()],
            hm_dependencies: pkg.depends_on.unwrap_or(vec![]),
            provider: "home-manager".into(),
            version_check: pkg.version_check,
            apply: pkg.apply.unwrap_or(true),
            ..Default::default()
        }
    }
}

impl HomeManagerInstaller {
    pub fn home_manager_init(&self) -> Result<(), Error> {
        println!(
            "-> Checking if {} exists",
            "~/.config/home-manager/home.nix".bright_green()
        );

        let home = std::env::var("HOME").unwrap();
        let home_manager_path = format!("{}/.config/home-manager/home.nix", home);

        if Path::new(&home_manager_path).exists() {
            println!(
                "-> {} exists",
                "~/.config/home-manager/home.nix".bright_green()
            );
            return Ok(());
        }

        println!(
            "-> Running {}",
            "nix run home-manager/master -- init".bright_green()
        );
        home_manager_init!(self.session.clone());
        Ok(())
    }

    pub fn install_dependencies(&self) -> Result<(), Error> {
        if self.hm_dependencies.is_empty() {
            return Ok(());
        }

        println!(
            "-> Installing dependencies for {}",
            self.name.bright_green()
        );
        let home = std::env::var("HOME").unwrap();
        let home_nix = format!("{}/.config/home-manager/home.nix", home);
        let home_nix_content = fs::read_to_string("tests/home.nix").unwrap();
        let deps = self.hm_dependencies.clone();
        let updated_nix_configs = crosup_nix::add_packages(&home_nix_content, deps)?;
        fs::write(home_nix, updated_nix_configs)?;

        let nix_env = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh";
        let command = format!("{} && home-manager switch", nix_env);
        exec_bash_with_output!(command, self.session.clone());

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
}

impl Installer for HomeManagerInstaller {
    fn install(&self) -> Result<(), Error> {
        self.home_manager_init()?;
        self.install_dependencies()?;

        if let Some(packages) = self.packages.clone() {
            let home = std::env::var("HOME").unwrap();
            let home_nix = format!("{}/.config/home-manager/home.nix", home);

            println!(
                "-> Adding {} to {}",
                packages.join(", ").bright_green(),
                home_nix.bright_green()
            );

            let home_nix_content = fs::read_to_string(&home_nix).unwrap();
            let updated_nix_configs = crosup_nix::add_packages(&home_nix_content, packages)?;
            fs::write(home_nix, updated_nix_configs)?;

            println!("-> Running {}", "home-manager switch".bright_green());
            let nix_env = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh";
            let command = format!("{} && home-manager switch", nix_env);
            exec_bash_with_output!(command, self.session.clone());
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
            check_version!(self, command, self.session.clone());
            return Ok(true);
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
