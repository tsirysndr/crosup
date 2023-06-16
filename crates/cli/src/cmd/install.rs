use anyhow::Error;
use crosup_core::{
    config::{verify_if_config_file_is_present, verify_if_inventory_config_file_is_present},
    graph::build_installer_graph,
};
use crosup_repo::{file::FileRepo, modification::ModificationRepo};
use crosup_ssh::setup_ssh_connection;
use crosup_types::configuration::Configuration;
use owo_colors::OwoColorize;
use sea_orm::{Database, DatabaseConnection};
use ssh2::Session;

use crate::{macros::install, types::InstallArgs};

pub async fn execute_install(args: InstallArgs) -> Result<(), Error> {
    let (mut config, filename, content, _) = verify_if_config_file_is_present()?;

    ask_confirmation(args.ask, &mut config);

    let mut sessions = Vec::new();

    if args.remote_is_present {
        sessions = parse_args(&args)?;
    }

    match sessions.len() {
        0 => {
            install!(args, config, None);
        }
        _ => {
            println!(
                "-> Installing tools on {} machine{}",
                sessions.len().bold().cyan(),
                if sessions.len() > 1 { "s" } else { "" }
            );
            let mut children = Vec::new();
            for session in sessions {
                let args = args.clone();
                let mut config = config.clone();

                let child = std::thread::spawn(move || {
                    install!(args, config, Some(session.clone()));
                    Ok::<(), Error>(())
                });
                children.push(child);
            }

            for child in children {
                child.join().unwrap()?;
            }
        }
    }

    let home = std::env::var("HOME").unwrap();
    let crosup_dir = format!("{}/.config/crosup", home);

    let database_url = format!("sqlite:{}/modifications.sqlite3?mode=rwc", crosup_dir);

    let db: DatabaseConnection = Database::connect(&database_url).await?;

    let current_dir = std::env::current_dir()?;
    let path = format!("{}/{}", current_dir.display(), filename);

    let file = FileRepo::new(&db).create(&filename, &path).await?;

    let hash = sha256::digest(content.clone());
    ModificationRepo::new(&db)
        .create(file.id, &hash, &content)
        .await?;

    Ok(())
}

fn ask_confirmation(ask: bool, config: &mut Configuration) {
    if ask {
        let (_, installers) = build_installer_graph(config, None);
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
