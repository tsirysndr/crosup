use std::fs;

use anyhow::Error;
use crosup_core::{config::verify_if_config_file_is_present, graph::build_installer_graph};
use crosup_types::configuration::Configuration;

use crate::{cmd::print_diff, macros::install, types::InstallArgs};

pub async fn execute_add(tools: String, _apply: bool) -> Result<(), Error> {
    let (mut current_config, filename, content, is_present) = verify_if_config_file_is_present()?;

    let tools = match tools.contains(",") {
        true => tools.split(",").map(|x| x.trim().to_string()).collect(),
        false => vec![tools],
    };

    current_config.packages = match current_config.packages {
        Some(ref mut packages) => {
            tools.iter().for_each(|x| {
                if !packages.contains(x) {
                    packages.push(x.clone())
                }
            });
            Some(packages.clone())
        }
        None => Some(tools.clone()),
    };
    let new_content = match filename.ends_with(".hcl") {
        true => hcl::to_string(&current_config)?,
        false => toml::to_string(&current_config)?,
    };

    let mut config = Configuration {
        packages: current_config.packages,
        ..Default::default()
    };

    print_diff(&content, &new_content);

    let args = InstallArgs {
        ..Default::default()
    };
    install!(args, config, None);

    if is_present {
        fs::write(filename, new_content)?;
    }

    Ok(())
}
