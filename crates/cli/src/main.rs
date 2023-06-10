use crate::cmd::{init::execute_init, install::execute_install};
use anyhow::Error;
use clap::{arg, Command};
use crosup_types::configuration::ConfigFormat;
use types::InstallArgs;

pub mod cmd;
pub mod macros;
pub mod types;

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
            .arg(arg!(--inventory -i "Generate a default inventory file"))
            .about("Generate a default configuration file"),
        )
        .subcommand(
            Command::new("install")
                .arg(arg!(--ask -a "Ask for confirmation before installing tools"))
                .arg(arg!([tool] "Tool to install, e.g. docker, nix, devbox, homebrew, fish, vscode, ble.sh etc."))
                .arg(arg!(--remote -r [ip] "Install tools on a remote machine"))
                .arg(arg!(--port -p [port] "Port to use when connecting to the remote machine"))
                .arg(
                    arg!(--username -u [username] "Username to use when connecting to the remote machine"),
                )
                .arg(arg!(--invetory -i [inventory] "Path to the inventory file (list of remote machines) in HCL or TOML format"))
                .about(
                    "Install developer tools, possible values are: docker, nix, devbox, homebrew, flox, fish, vscode, ble.sh, atuin, tig, fzf, httpie, kubectl, minikube, tilt, zellij, ripgrep, fd, exa, bat, glow, devenv",
                ),
        )
}

fn main() -> Result<(), Error> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("install", args)) => {
            let ask = args.is_present("ask");
            let tool = args.value_of("tool").map(|tool| tool.to_string());
            let remote_is_present = args.is_present("remote");
            let remote = args.value_of("remote").map(|remote| remote.to_string());
            let port = args
                .value_of("port")
                .map(|port| port.parse::<u16>().unwrap());
            let username = args
                .value_of("username")
                .map(|username| username.to_string());
            let inventory = args
                .value_of("inventory")
                .map(|inventory| inventory.to_string());

            execute_install(InstallArgs {
                ask,
                tool,
                remote_is_present,
                remote,
                username,
                inventory,
                port,
            })?;
        }
        Some(("init", args)) => {
            let toml = args.is_present("toml");
            let inventory = args.is_present("inventory");
            match toml {
                true => execute_init(ConfigFormat::TOML, inventory)?,
                false => execute_init(ConfigFormat::HCL, inventory)?,
            }
        }
        _ => {
            cli().print_help().unwrap();
        }
    }

    Ok(())
}
