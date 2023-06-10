use std::io::Write;

use anyhow::Error;
use inquire::Confirm;
use owo_colors::OwoColorize;

use crosup_types::{
    configuration::{ConfigFormat, Configuration},
    inventory::Inventory,
};

pub fn execute_init(cfg_format: ConfigFormat, inventory: bool) -> Result<(), Error> {
    let ext = match cfg_format {
        ConfigFormat::HCL => "hcl",
        ConfigFormat::TOML => "toml",
    };

    let filename = match inventory {
        true => format!("Inventory.{}", ext),
        false => format!("Crosfile.{}", ext),
    };

    if std::path::Path::new(&filename).exists() {
        let answer = Confirm::new(
            format!(
                "A {} file already exists in this directory, do you want to overwrite it?",
                filename.bright_green()
            )
            .as_str(),
        )
        .with_default(false)
        .with_help_message("Press y to overwrite the file or n to exit")
        .prompt();
        if answer.is_err() || !answer.unwrap() {
            println!("Exiting...");
            return Ok(());
        }
    }

    if inventory {
        let inventory = Inventory::default();
        let serialized = match cfg_format {
            ConfigFormat::HCL => hcl::to_string(&inventory).unwrap(),
            ConfigFormat::TOML => toml::to_string_pretty(&inventory).unwrap(),
        };

        let mut file = std::fs::File::create(&filename).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
        println!("Created {} ✨", filename.bright_green());
        return Ok(());
    }

    let config = Configuration::default();
    let serialized = match cfg_format {
        ConfigFormat::HCL => hcl::to_string(&config).unwrap(),
        ConfigFormat::TOML => toml::to_string_pretty(&config).unwrap(),
    };

    let mut file = std::fs::File::create(&filename).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    println!("Created {} ✨", filename.bright_green());

    Ok(())
}
