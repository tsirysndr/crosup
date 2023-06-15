#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! brew_install {
    ($self:ident, $package:expr, $session:expr) => {
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
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
            return Err(Error::msg(format!("Failed to install {}", $self.name())));
        }
    };
}

#[macro_export]
macro_rules! check_version {
    ($self:ident, $command:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sh -c 'PATH=/home/linuxbrew/.linuxbrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin {}'", $command);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
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
                    if !output.stderr.is_empty() {
                        println!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                    return Err(Error::msg(format!(
                        "Failed to check {} version",
                        $self.name
                    )));
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    println!("   {}", line.cyan());
                }
            }
        };
    };
}

#[macro_export]
macro_rules! exec_bash {
    ($command:expr,$session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("bash -c '{}'", $command);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("bash")
                    .arg("-c")
                    .arg($command)
                    .stdout(Stdio::piped())
                    .spawn()?;
                child.wait()?;
            }
        };
    };
}

#[macro_export]
macro_rules! exec_sh {
    ($command:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sh -c '{}'", $command);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg($command)
                    .stdout(Stdio::piped())
                    .spawn()?;
                child.wait()?;
            }
        };
    };
}

#[macro_export]
macro_rules! exec_bash_with_output {
    ($command:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("bash -c '{}'", $command);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
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
            }
        };
    };
}

#[macro_export]
macro_rules! exec_sh_with_output {
    ($command:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sh -c '{}'", $command);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg($command)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                child.wait()?;
            }
        };
    };
}

#[macro_export]
macro_rules! apt_install {
    ($package:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo apt-get install -y {}", $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
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
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! yum_install {
    ($package:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo yum install -y {}", $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo yum install -y {}", $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! dnf_install {
    ($package:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo dnf install -y {}", $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo dnf install -y {}", $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! zypper_install {
    ($package:expr, $options:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo zypper install {} {}", $options, $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo zypper install {} {}", $options, $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! apk_add {
    ($package:expr, $options:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo apk add {} {}", $options, $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo apk add {} {}", $options, $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! pacman_install {
    ($package:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo pacman -S {}", $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo pacman -S {}", $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! emerge_install {
    ($package:expr, $options:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo emerge {} {}", $options, $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo emerge {} {}", $options, $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! slackpkg_install {
    ($package:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo slackpkg install {}", $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo slackpkg {}", $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! fleek_install {
    ($package:expr, $options:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("bash -c '. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix run github:ublue-os/fleek -- add {} {}'", $options, $package);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(format!(". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix run github:ublue-os/fleek -- add {} {}", $options, $package))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to install {}", $package)));
                }
            }
        };
    };
}

#[macro_export]
macro_rules! exec_sudo {
    ($command:expr, $session:expr) => {
        match $session {
            Some(session) => {
                let command = format!("sudo {}", $command);
                crosup_ssh::exec(session.clone(), &command)?;
            }
            None => {
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("sudo {}", $command))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
                let output = child.stdout.take().unwrap();
                let output = std::io::BufReader::new(output);

                for line in output.lines() {
                    println!("{}", line?);
                }
                let status = child.wait()?;
                if !status.success() {
                    return Err(Error::msg(format!("Failed to execute {}", $command)));
                }
            }
        }
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! add_vertex {
    ($graph:ident, $installer:ident, $config:ident, $pkg_manager:ident, $pkg:ident, $session:ident) => {
        if let Some(pkg_manager) = &$config.$pkg_manager {
            if let Some(installer) = pkg_manager.get("install") {
                installer.$pkg.iter().for_each(|(name, x)| {
                    $graph.add_vertex(Vertex::from(Box::new($installer {
                        name: name.clone(),
                        session: $session.clone(),
                        ..$installer::from(x.clone())
                    }) as Box<dyn Installer>));
                });
            }
        }
    };
}

#[macro_export]
macro_rules! add_vertex_with_condition {
    ($graph:ident, $installer:ident, $config:ident, $pkg_manager:ident, $pkg:ident, $session:ident) => {
        if let Some(pkg_manager) = &$config.$pkg_manager {
            if let Some(installer) = pkg_manager.get("install") {
                match installer.$pkg.clone() {
                    Some(pkg) => {
                        pkg.iter().for_each(|(name, x)| {
                            $graph.add_vertex(Vertex::from(Box::new($installer {
                                name: name.clone(),
                                session: $session.clone(),
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

#[macro_export]
macro_rules! downcast_installer {
    ($label: expr,$installer: ident, $installer_type: ident) => {
        match $installer.provider() {
            $label => Some(
                $installer
                    .as_any()
                    .downcast_ref::<$installer_type>()
                    .map(|x| x.clone())
                    .unwrap(),
            ),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! convert_generic_installer {
    ($config: ident, $generic_install: ident, $installer: ident) => {
        $config.$installer = Some(
            [("install".into(), $generic_install.clone().into())]
                .into_iter()
                .chain($config.clone().$installer.unwrap_or_default().into_iter())
                .collect(),
        );
    };
}
