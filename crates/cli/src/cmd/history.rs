use std::collections::HashMap;

use super::get_database_connection;
use anyhow::Error;
use crosup_core::config::verify_if_config_file_is_present;
use crosup_repo::{file::FileRepo, modification::ModificationRepo};
use crosup_tui::{history::display_history, App};
use migration::MigratorTrait;
use owo_colors::OwoColorize;
use sea_orm::DatabaseConnection;

pub async fn execute_history() -> Result<(), Error> {
    let (_, filename, _, _) = verify_if_config_file_is_present()?;

    let db: DatabaseConnection = get_database_connection().await?;

    migration::Migrator::up(&db, None).await?;

    let current_dir = std::env::current_dir()?;
    let path = format!("{}/{}", current_dir.display(), filename);

    let result = FileRepo::new(&db).find_by_path(&path).await?;

    if let Some(file) = result {
        let result = ModificationRepo::new(&db).find_by_file_id(file.id).await?;
        let mut content = HashMap::new();
        let mut dates = HashMap::new();
        let mut hashes = HashMap::new();
        for (index, m) in result.iter().enumerate() {
            content.insert(index, m.content.clone());
            dates.insert(index, m.timestamp);
            hashes.insert(index, m.hash.clone());
        }

        let app = App {
            items: result
                .into_iter()
                .map(|m| format!("{} {} {}", m.timestamp.to_string(), filename, m.hash))
                .collect(),
            content,
            selected_index: 0,
            title: filename,
        };
        display_history(app)?;
        return Ok(());
    }

    println!(
        "{} has not been modified, no history available",
        filename.bold().cyan()
    );

    Ok(())
}
