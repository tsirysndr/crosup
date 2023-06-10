use anyhow::Error;
use owo_colors::OwoColorize;
use std::path::Path;

use crosup_types::{
    configuration::Configuration, inventory::Inventory, CROSFILE_HCL, CROSFILE_TOML, INVENTORY_HCL,
    INVENTORY_TOML,
};

pub fn verify_if_config_file_is_present() -> Result<Configuration, Error> {
    if !Path::new(CROSFILE_HCL).exists() && !Path::new(CROSFILE_TOML).exists() {
        let config = Configuration::default();
        return Ok(config);
    }

    let current_dir = std::env::current_dir()?;

    if Path::new(CROSFILE_HCL).exists() {
        let config = std::fs::read_to_string(current_dir.join(CROSFILE_HCL))?;
        let config = hcl::from_str(&config)?;
        return Ok(config);
    }

    let config = std::fs::read_to_string(current_dir.join(CROSFILE_TOML))?;
    let config = toml::from_str(&config)?;
    return Ok(config);
}

pub fn verify_if_inventory_config_file_is_present() -> Result<Inventory, Error> {
    if !Path::new(INVENTORY_HCL).exists() && !Path::new(INVENTORY_TOML).exists() {
        return Err(anyhow::anyhow!(format!(
            "Inventory file not found, please create one using {}",
            "crosup init --inventory".bright_green()
        )));
    }

    let current_dir = std::env::current_dir()?;

    if Path::new(INVENTORY_HCL).exists() {
        let config = std::fs::read_to_string(current_dir.join(INVENTORY_HCL))?;
        let config = hcl::from_str(&config)?;
        return Ok(config);
    }

    let config = std::fs::read_to_string(current_dir.join(INVENTORY_TOML))?;
    let config = toml::from_str(&config)?;
    return Ok(config);
}
