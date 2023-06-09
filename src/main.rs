use anyhow::Error;
use clap::{arg, Command};
use crosup::{
    cmd::{init::execute_init, install::execute_install},
    types::configuration::ConfigFormat,
};

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

Quickly install your development tools on your new Chromebook or any Linux distribution"#,
        )
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .subcommand(
            Command::new("init")
            .arg(arg!(--toml "Generate a default configuration file in toml format")) 
            .about("Generate a default configuration file"),
        )
        .subcommand(
            Command::new("install")
                .arg(arg!([tool] "Tool to install, e.g. docker, nix, devbox, homebrew, fish, vscode, ble.sh etc."))
                .about(
                    "Install developer tools, possible values are: docker, nix, devbox, homebrew, flox, fish, vscode, ble.sh, atuin, tig, fzf, httpie, kubectl, minikube, tilt, zellij, ripgrep, fd, exa, bat, glow, devenv",
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
        Some(("init", args)) => {
            let toml = args.is_present("toml");
            match toml {
                true => execute_init(ConfigFormat::TOML)?,
                false => execute_init(ConfigFormat::HCL)?,
            }
        }
        _ => {
            cli().print_help().unwrap();
        }
    }

    Ok(())
}
