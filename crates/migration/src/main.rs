use std::fs;

use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Statement},
};

#[async_std::main]
async fn main() {
    let home = std::env::var("HOME").unwrap();
    let crosup_dir = format!("{}/.config/crosup", home);
    fs::create_dir_all(&crosup_dir).unwrap();

    let database_url = format!("sqlite:{}/modifications.sqlite3?mode=rwc", crosup_dir);

    std::env::set_var("DATABASE_URL", database_url);

    cli::run_cli(migration::Migrator).await;
}
