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

pub(crate) use pipe_brew_curl;
pub(crate) use pipe_curl;
