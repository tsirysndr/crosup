use anyhow::Error;
use owo_colors::OwoColorize;

use crate::{
    config::verify_if_config_file_is_present, graph::build_installer_graph,
    types::configuration::Configuration,
};

pub fn execute_install(tool: Option<String>, ask: bool) -> Result<(), Error> {
    let mut config = verify_if_config_file_is_present()?;

    ask_confirmation(ask, &mut config);

    match tool {
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
