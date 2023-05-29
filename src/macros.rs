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

pub(crate) use append_to_nix_conf;
pub(crate) use pipe_brew_curl;
pub(crate) use pipe_curl;
