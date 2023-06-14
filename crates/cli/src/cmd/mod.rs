use anyhow::Error;
use sea_orm::{Database, DatabaseConnection};

pub mod diff;
pub mod history;
pub mod init;
pub mod install;

pub async fn get_database_connection() -> Result<DatabaseConnection, Error> {
    let home = std::env::var("HOME").unwrap();
    let crosup_dir = format!("{}/.config/crosup", home);

    let database_url = format!("sqlite:{}/modifications.sqlite3?mode=rwc", crosup_dir);

    let db: DatabaseConnection = Database::connect(&database_url).await?;
    Ok(db)
}
