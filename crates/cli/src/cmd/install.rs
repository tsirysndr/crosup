use anyhow::Error;
use crosup_core::{
    config::{verify_if_config_file_is_present, verify_if_inventory_config_file_is_present},
    graph::build_installer_graph,
};
use crosup_ssh::setup_ssh_connection;
use crosup_types::configuration::Configuration;
use owo_colors::OwoColorize;
use ssh2::Session;

use crate::types::InstallArgs;

pub fn execute_install(args: InstallArgs) -> Result<(), Error> {
    let mut config = verify_if_config_file_is_present()?;

    ask_confirmation(args.ask, &mut config);

    if args.remote_is_present {
        parse_args(&args)?;
    }

    match args.tool {
        Some(tool) => {
            let tool = tool.replace(" ", "");
            let tool = tool.replace("ble.sh", "blesh");
            let tools = match tool.contains(",") {
                true => tool.split(",").collect(),
                false => vec![tool.as_str()],
            };
            for tool in tools {
                let (graph, installers) = build_installer_graph(&mut config);
                let tool = installers
                    .into_iter()
                    .find(|installer| installer.name() == tool)
                    .ok_or_else(|| Error::msg(format!("{} is not available yet", tool)))?;
                let mut visited = vec![false; graph.size()];
                graph.install(tool, &mut visited)?;
            }
        }
        None => {
            let (graph, _) = build_installer_graph(&mut config);
            graph.install_all()?;
        }
    }

    Ok(())
}

fn ask_confirmation(ask: bool, config: &mut Configuration) {
    if ask {
        let (_, installers) = build_installer_graph(config);
        println!("-> The following tools will be installed:");

        for installer in installers.iter() {
            println!("  - {}", installer.name().bright_green());
        }

        println!(
            "-> Are you sure you want to install these {} tools? [y/N]",
            installers.len().bold().cyan()
        );
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "y" | "Y" => {}
            _ => std::process::exit(0),
        }
    }
}

fn parse_args(args: &InstallArgs) -> Result<Vec<Session>, Error> {
    let remote = args.remote.as_ref();

    match remote {
        Some(remote) => {
            if args.username.is_none() {
                return Err(Error::msg(
                    "username is required, please use -u or --username",
                ));
            }
            let port = args.port.unwrap_or(22);
            let username = args.username.as_ref().unwrap();
            let addr = format!("{}:{}", *remote, port);
            let session = setup_ssh_connection(&addr, username)?;
            Ok(vec![session])
        }
        None => {
            let config = verify_if_inventory_config_file_is_present()?;
            let mut sessions = Vec::new();
            for (_, server) in config.server.iter() {
                let port = server.port.unwrap_or(22);
                let addr = format!("{}:{}", server.host, port);
                let session = setup_ssh_connection(&addr, &server.username).unwrap();
                sessions.push(session);
            }
            return Ok(sessions);
        }
    }
}
