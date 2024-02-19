use anyhow::Error;
use crosup_core::config::verify_if_config_file_is_present;
use crosup_repo::{file::FileRepo, modification::ModificationRepo};
use migration::MigratorTrait;
use owo_colors::OwoColorize;
use sea_orm::DatabaseConnection;

use crate::cmd::print_diff;

use super::get_database_connection;

pub async fn execute_diff() -> Result<(), Error> {
    let (_, filename, content, _) = verify_if_config_file_is_present(None).await?;

    let db: DatabaseConnection = get_database_connection().await?;
    migration::Migrator::up(&db, None).await?;
    let current_dir = std::env::current_dir()?;
    let path = format!("{}/{}", current_dir.display(), filename);

    let result = FileRepo::new(&db).find_by_path(&path).await?;
    if let Some(file) = result {
        let result = ModificationRepo::new(&db)
            .find_last_by_file_id(file.id)
            .await?;
        if let Some(last_modif) = result {
            let hash = sha256::digest(content.clone());
            if hash != last_modif.hash {
                println!("   📝 {} has been modified", filename.bold().cyan());
                print_diff(&last_modif.content, &content);
                return Ok(());
            }
            println!("{} has not been modified", filename.bold().cyan());
        }
    }
    Ok(())
}
