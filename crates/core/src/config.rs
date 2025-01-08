use anyhow::Error;
use owo_colors::OwoColorize;
use reqwest::Client;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use zip::ZipArchive;

use crosup_types::{
    configuration::Configuration, inventory::Inventory, CROSFILE_HCL, CROSFILE_TOML, INVENTORY_HCL,
    INVENTORY_TOML,
};

pub async fn verify_if_config_file_is_present(
    github_repo: Option<String>,
) -> Result<(Configuration, String, String, bool), Error> {
    let current_dir = match github_repo {
        Some(repo) => download_github_repo(&repo).await?,
        None => std::env::current_dir()?,
    };

    if !Path::new(CROSFILE_HCL).exists() && !Path::new(CROSFILE_TOML).exists() {
        let config = Configuration::default();
        return Ok((
            config.clone(),
            CROSFILE_HCL.into(),
            hcl::to_string(&config)?,
            false,
        ));
    }

    if Path::new(CROSFILE_HCL).exists() {
        let config = std::fs::read_to_string(current_dir.join(CROSFILE_HCL))?;
        let content = config.clone();
        let config = hcl::from_str(&config)?;
        return Ok((config, CROSFILE_HCL.into(), content, true));
    }

    let config = std::fs::read_to_string(current_dir.join(CROSFILE_TOML))?;
    let content = config.clone();
    let config = toml::from_str(&config)?;
    return Ok((config, CROSFILE_TOML.into(), content, true));
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

pub async fn download_github_repo(github_repo: &str) -> Result<PathBuf, Error> {
    let home = std::env::var("HOME").unwrap();
    let cache_dir = format!("{}/.crosup/cache", home);

    create_dir_all(&cache_dir)?;

    println!(
        "{} {} ...",
        "Downloading".bold().bright_green(),
        github_repo
    );

    // download the repo as a zip file using reqwest
    // and extract it to the cache_dir
    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.github.com/repos/{}/zipball",
            github_repo
        ))
        .header("User-Agent", "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await?;
    let mut file = File::create(format!(
        "{}/{}.zip",
        cache_dir,
        github_repo.replace("/", "_")
    ))?;
    file.write_all(&response.bytes().await?)?;

    let file = File::open(format!(
        "{}/{}.zip",
        cache_dir,
        github_repo.replace("/", "_")
    ))?;

    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file.mangled_name();

        println!(
            "{} {} ...",
            "Extracting".bold().bright_green(),
            outpath.display()
        );

        if (&*file.name()).ends_with('/') {
            create_dir_all(format!("{}/{}", cache_dir, outpath.display()))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(format!("{}/{}", cache_dir, p.display()))?;
                }
            }
            let mut outfile = File::create(format!("{}/{}", cache_dir, outpath.display()))?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    std::env::current_dir().map_err(|e| Error::msg(e.to_string()))
}
