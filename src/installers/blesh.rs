use std::io::BufRead;

use anyhow::Error;
use owo_colors::OwoColorize;

use super::Installer;

pub struct BleshInstaller {
    name: String,
    version: String,
    dependencies: Vec<String>,
    default: bool,
}

impl Default for BleshInstaller {
    fn default() -> Self {
        Self {
            name: "ble.sh".to_string(),
            version: "latest".to_string(),
            dependencies: vec![],
            default: true,
        }
    }
}

impl BleshInstaller {
    fn apt_update(&self) -> Result<(), Error> {
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

    fn install_dependencies(&self) -> Result<(), Error> {
        println!("-> 🚚 Installing dependencies");
        println!(
            "   Running {}",
            "sudo apt install -y gawk build-essential".bright_green()
        );
        let mut child = std::process::Command::new("sudo")
            .arg("apt install -y gawk build-essential")
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
}

impl Installer for BleshInstaller {
    fn install(&self) -> Result<(), anyhow::Error> {
        if self.is_installed().is_ok() {
            println!(
                "-> {} is already installed, skipping",
                self.name.bright_green()
            );
            return Ok(());
        }
        println!("-> 🚚 Installing {}", "ble.sh".bright_green());
        self.apt_update()?;
        self.install_dependencies()?;

        println!("-> Running {}", "git clone --recursive --depth 1 --shallow-submodules https://github.com/akinomyoga/ble.sh.git".bright_green());
        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg("git clone --recursive --depth 1 --shallow-submodules https://github.com/akinomyoga/ble.sh.git")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);
        for line in output.lines() {
            println!("{}", line?);
        }
        child.wait()?;

        println!(
            "-> Running {}",
            "make -C ble.sh install PREFIX=~/.local".bright_green()
        );
        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg("make -C ble.sh install PREFIX=~/.local")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);
        for line in output.lines() {
            println!("{}", line?);
        }
        child.wait()?;

        println!(
            "-> Running {}",
            "echo 'source ~/.local/share/blesh/ble.sh' >> ~/.bashrc".bright_green()
        );
        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg("echo 'source ~/.local/share/blesh/ble.sh' >> ~/.bashrc")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        child.wait()?;

        println!(
            "   Done! 🎉 Open a new terminal to start using ble.sh or run {}",
            "source ~/.bashrc".bright_green()
        );

        Ok(())
    }

    fn is_installed(&self) -> Result<bool, anyhow::Error> {
        println!("-> Checking if {} is installed", self.name.bright_green());
        let home = std::env::var("HOME")?;
        // verify if ~/.local/share/blesh/ble.sh exists
        if std::path::Path::new(&format!("{}/.local/share/blesh/ble.sh", home)).exists() {
            return Ok(true);
        }
        Err(anyhow::anyhow!("ble.sh is not installed"))
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn version(&self) -> &str {
        self.version.as_str()
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn is_default(&self) -> bool {
        self.default
    }
}
