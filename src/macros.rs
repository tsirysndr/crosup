macro_rules! pipe_curl {
    ($curl:ident) => {
        let mut child = std::process::Command::new("bash")
            .stdin(Stdio::from($curl.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }
        child.wait()?;
    };
}

macro_rules! pipe_brew_curl {
    ($curl:ident) => {
        let mut child = std::process::Command::new("bash")
            .env("NONINTERACTIVE", "true")
            .stdin(Stdio::from($curl.stdout.unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stdout = std::io::BufReader::new(stdout);
        for line in stdout.lines() {
            println!("   {}", line.unwrap());
        }

        child.wait()?;
    };
}

macro_rules! append_to_nix_conf {
    ($echo:ident) => {
        let mut tee = std::process::Command::new("bash")
            .arg("-c")
            .arg("sudo tee -a /etc/nix/nix.conf")
            .stdin(Stdio::from($echo.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        tee.wait()?;
    };
}

macro_rules! brew_install {
    ($self:ident, $package:expr) => {
        let mut child = std::process::Command::new("brew")
            .arg("install")
            .arg($package)
            .env(
                "PATH",
                "/home/linuxbrew/.linuxbrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
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
            println!("-> Failed to install {}", $self.name().bright_green());
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!("Failed to install {}", $self.name())));
        }
    };
}

macro_rules! check_version {
    ($self:ident, $command:expr) => {
        let child = std::process::Command::new("bash")
            .arg("-c")
            .arg($command)
            .env(
                "PATH",
                "/home/linuxbrew/.linuxbrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
            )
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.wait_with_output()?;

        if !output.status.success() {
            println!("-> Failed to check {} version", $self.name.bright_green());
            println!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::msg(format!(
                "Failed to check {} version",
                $self.name
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            println!("   {}", line.cyan());
        }
    };
}

macro_rules! exec_bash {
    ($command:expr) => {
        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg($command)
            .stdout(Stdio::piped())
            .spawn()?;
        child.wait()?;
    };
}

macro_rules! exec_bash_with_output {
    ($command:expr) => {
        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg($command)
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }
        child.wait()?;
    };
}

macro_rules! apt_install {
    ($package:expr) => {
        let mut child = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("sudo apt-get install -y {}", $package))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let output = child.stdout.take().unwrap();
        let output = std::io::BufReader::new(output);

        for line in output.lines() {
            println!("{}", line?);
        }
        child.wait()?;
    };
}

macro_rules! exec_sudo {
    ($command:expr) => {
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("sudo {}", $command))
            .stdout(Stdio::piped())
            .spawn()?;
        child.wait()?;
    };
}

macro_rules! exec_piped_sudo {
    ($command:expr, $stdin:ident) => {
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("sudo {}", $command))
            .stdin(Stdio::from($stdin.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        child.wait()?;
    };
}

macro_rules! add_vertex {
    ($graph:ident, $installer:ident, $config:ident, $pkg_manager:ident, $pkg:ident) => {
        if let Some(pkg_manager) = &$config.$pkg_manager {
            if let Some(installer) = pkg_manager.get("install") {
                installer.$pkg.iter().for_each(|(name, x)| {
                    $graph.add_vertex(Vertex::from(Box::new($installer {
                        name: name.clone(),
                        ..$installer::from(x.clone())
                    }) as Box<dyn Installer>));
                });
            }
        }
    };
}

macro_rules! add_vertex_with_condition {
    ($graph:ident, $installer:ident, $config:ident, $pkg_manager:ident, $pkg:ident) => {
        if let Some(pkg_manager) = &$config.$pkg_manager {
            if let Some(installer) = pkg_manager.get("install") {
                match installer.$pkg.clone() {
                    Some(pkg) => {
                        pkg.iter().for_each(|(name, x)| {
                            $graph.add_vertex(Vertex::from(Box::new($installer {
                                name: name.clone(),
                                ..$installer::from(x.clone())
                            })
                                as Box<dyn Installer>));
                        });
                    }
                    None => {}
                }
            }
        }
    };
}

pub(crate) use add_vertex;
pub(crate) use add_vertex_with_condition;
pub(crate) use append_to_nix_conf;
pub(crate) use apt_install;
pub(crate) use brew_install;
pub(crate) use check_version;
pub(crate) use exec_bash;
pub(crate) use exec_bash_with_output;
pub(crate) use exec_piped_sudo;
pub(crate) use exec_sudo;
pub(crate) use pipe_brew_curl;
pub(crate) use pipe_curl;
