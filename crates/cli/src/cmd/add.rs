use std::fs;

use anyhow::Error;
use crosup_core::{config::verify_if_config_file_is_present, graph::build_installer_graph};
use owo_colors::OwoColorize;

use crate::{cmd::print_diff, macros::install, types::InstallArgs};

pub async fn execute_add(tools: Vec<String>, ask: bool) -> Result<(), Error> {
    let (mut current_config, filename, content, is_present) = verify_if_config_file_is_present()?;

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

    print_diff(&content, &new_content);

    if ask {
        ask_confirmation(ask, tools.clone());
    }

    let args = InstallArgs {
        ..Default::default()
    };

    install!(args, current_config, None);

    if is_present {
        fs::write(filename, new_content)?;
    }

    Ok(())
}

fn ask_confirmation(ask: bool, tools: Vec<String>) {
    if ask {
        println!("\n-> The following packages will be added to your configuration:");

        for tool in tools.iter() {
            println!("  - {}", tool.bright_green());
        }

        match tools.len() {
            1 => println!("-> Are you sure you want to install this package? [y/N]"),
            _ => println!(
                "-> Are you sure you want to install these {} packages? [y/N]",
                tools.len().bold().cyan()
            ),
        };
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
