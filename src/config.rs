use anyhow::Error;
use std::path::Path;

use crate::types::{configuration::Configuration, CROSFILE_HCL, CROSFILE_TOML};

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
