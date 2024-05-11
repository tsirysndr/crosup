use crate::cmd::{init::execute_init, install::execute_install};
use anyhow::Error;
use clap::{arg, Command};
use cmd::{add::execute_add, diff::execute_diff, history::execute_history, search::execute_search};
use crosup_types::configuration::ConfigFormat;
use types::{InstallArgs, SearchArgs};

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

Quickly install your development tools on your new Chromebook, MacOS or any Linux distribution"#,
        )
        .author("Tsiry Sandratraina <tsiry.sndr@fluentci.io>")
        .subcommand(
            Command::new("init")
            .arg(arg!(--toml "Generate a default configuration file in toml format"))
            .arg(arg!(-i --inventory "Generate a default inventory file"))
            .arg(arg!([packages]... "List of packages to install"))
            .about("Generate a default configuration file"),
        )
        .subcommand(
            Command::new("install")
                .arg(arg!(-a --ask "Ask for confirmation before installing tools"))
                .arg(arg!([tools]... "List of tools to install, e.g. docker, nix, devbox, homebrew, fish, vscode, ble.sh ..."))
                .arg(arg!(-r --remote [ip] "Install tools on a remote machine"))
                .arg(arg!(-p --port [port] "Port to use when connecting to the remote machine"))
                .arg(
                    arg!(-u --username [username] "Username to use when connecting to the remote machine"),
                )
                .arg(arg!(-i --inventory [inventory] "Path to the inventory file (list of remote machines) in HCL or TOML format"))
                .arg(arg!(-f --from [from] "A Github repository to install tools from, e.g. tsirysndr/crosup-example"))
                .about(
                    "Install developer tools, e.g. docker, nix, devbox, homebrew, fish, vscode, ble.sh ...",
                ),
        )
        .subcommand(
            Command::new("diff")
                .about("Show the difference between the current configuration and the previous one"),
        )
        .subcommand(
            Command::new("history")
                .about("Show the change history of the configuration file"),
        )
        .subcommand(
            Command::new("add")
                .arg(arg!(-a --ask "Ask for confirmation before adding a new tool"))
                .arg(arg!(<tools>... "Tools to add to the configuration file, e.g. gh, vim, tig ..."))
                .about("Add a new tool to the configuration file"),
        )
        .subcommand(
            Command::new("search")
                .arg(arg!(-c --channel [channel] "Channel to use when searching for a package"))
                .arg(arg!(-m --max [max_results] "Maximum number of results to return"))
                .arg(arg!(<package> "Package to search for"))
                .about("Search for a package in the nixpkgs repository"),
        )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("install", args)) => {
            let ask = args.is_present("ask");
            let tools = args.values_of("tools").map(|tools| {
                tools
                    .into_iter()
                    .map(|tool| tool.to_string())
                    .collect::<Vec<String>>()
            });
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
            let from = args.value_of("from").map(|from| from.to_string());

            execute_install(InstallArgs {
                ask,
                tools,
                remote_is_present,
                remote,
                username,
                inventory,
                port,
                from,
            })
            .await?;
        }
        Some(("init", args)) => {
            let toml = args.is_present("toml");
            let inventory = args.is_present("inventory");
            let packages = args.values_of("packages").map(|packages| {
                packages
                    .into_iter()
                    .map(|package| package.to_string())
                    .collect::<Vec<String>>()
            });
            match toml {
                true => execute_init(ConfigFormat::TOML, inventory, packages)?,
                false => execute_init(ConfigFormat::HCL, inventory, packages)?,
            }
        }
        Some(("diff", _)) => {
            execute_diff().await?;
        }
        Some(("history", _)) => {
            execute_history().await?;
        }
        Some(("add", args)) => {
            let tools = args
                .values_of("tools")
                .map(|tool| {
                    tool.into_iter()
                        .map(|tool| tool.to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap();
            let ask = args.is_present("ask");
            execute_add(tools, ask).await?;
        }
        Some(("search", args)) => {
            let package = args.value_of("package").unwrap();
            let channel = args.value_of("channel").unwrap_or("unstable");
            let max_results = args
                .value_of("max_results")
                .map(|max_results| max_results.parse::<u32>().unwrap())
                .unwrap_or(10);
            let args = SearchArgs {
                package: package.to_string(),
                channel: channel.to_string(),
                max_results,
            };
            execute_search(args).await?;
        }
        _ => {
            cli().print_help().unwrap();
        }
    }

    Ok(())
}
