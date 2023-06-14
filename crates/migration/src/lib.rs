use std::fs;

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}

pub async fn run() {
    let home = std::env::var("HOME").unwrap();
    let crosup_dir = format!("{}/.config/crosup", home);
    fs::create_dir_all(&crosup_dir).unwrap();
    let database_url = format!("sqlite:{}/modifications.sqlite3?mode=rwc", crosup_dir);

    std::env::set_var("DATABASE_URL", database_url);
    cli::run_cli(Migrator).await;
}
