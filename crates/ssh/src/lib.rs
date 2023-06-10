use std::{
    collections::HashMap,
    io::{self, Read},
    process::Command,
};

use anyhow::Error;
use ssh2::Session;

pub fn exec(sess: Session, command: &str) -> Result<(), Error> {
    let mut channel = sess.channel_session()?;
    channel.exec(command)?;

    let mut output = String::new();
    let mut buffer = [0; 1024];
    loop {
        match channel.read(&mut buffer) {
            Ok(n) => {
                if n > 0 {
                    let chunk = std::str::from_utf8(&buffer[..n])?;
                    output.push_str(chunk);
                    println!("{}", chunk);
                } else {
                    break;
                }
            }
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => continue,
            Err(err) => return Err(err.into()),
        }
    }

    channel.close()?;
    channel.wait_close()?;

    if channel.exit_status()? != 0 {
        return Err(Error::msg("exit status is not 0"));
    }

    Ok(())
}

pub fn setup_ssh_agent_var() -> Result<(), Error> {
    let child = Command::new("ssh-agent").arg("-s").output()?;
    let output = String::from_utf8(child.stdout)?;

    let mut envs = HashMap::new();

    for line in output.lines() {
        let env = line.split(";").next();
        if let Some(env) = env {
            let mut env = env.split("=");
            let key = env.next();
            let value = env.next();
            if let (Some(key), Some(value)) = (key, value) {
                std::env::set_var(key, value);
                envs.insert(key, value);
            }
        }
    }

    let mut child = Command::new("sh")
        .arg("-c")
        .arg("ssh-add ~/.ssh/id_rsa")
        .envs(envs)
        .spawn()?;

    child.wait()?;

    Ok(())
}

pub fn setup_ssh_connection(addr: &str, username: &str) -> Result<Session, Error> {
    setup_ssh_agent_var()?;
    let tcp = std::net::TcpStream::connect(addr)?;
    let mut sess = Session::new()?;
    let mut agent = sess.agent()?;
    agent.connect()?;

    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_agent(username)?;

    if !sess.authenticated() {
        return Err(Error::msg("authentication failed"));
    }

    Ok(sess)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let session = setup_ssh_connection("192.168.8.101:22", "tsirysandratraina").unwrap();
        exec(session, "ls -l").unwrap();
        assert!(true);
    }
}
