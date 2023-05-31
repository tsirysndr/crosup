use anyhow::Error;
use clap::{arg, Command};
use crosup::cmd::install::execute_install;

fn cli() -> Command<'static> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    Command::new("crosup")
        .version(VERSION)
        .about(
            r#"
             ______                __  __    
            / ____/________  _____/ / / /___ 
           / /   / ___/ __ \/ ___/ / / / __ \
          / /___/ /  / /_/ (__  ) /_/ / /_/ /
          \____/_/   \____/____/\____/ .___/ 
                                    /_/      

ChromeOS developer environment setup tool"#,
        )
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .subcommand(
            Command::new("install")
                .arg(arg!([tool] "Tool to install, e.g. docker, nix, devbox, homebrew, fish, vscode, ble.sh etc."))
                .about(
                    "Install developer tools, possible values are: docker, nix, devbox, homebrew, flox, fish, vscode, ble.sh, atuin, tig, fzf, httpie, kubectl, minikube, tilt, zellij, ripgrep, fd, exa, bat",
                ),
        )
}

fn main() -> Result<(), Error> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("install", args)) => {
            let tool = args.value_of("tool").map(|tool| tool.to_string());
            execute_install(tool)?;
        }
        _ => {
            cli().print_help().unwrap();
        }
    }

    Ok(())
}
