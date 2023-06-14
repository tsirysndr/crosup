use anyhow::Error;
use crosup_core::config::verify_if_config_file_is_present;
use crosup_repo::{file::FileRepo, modification::ModificationRepo};
use owo_colors::{OwoColorize, Style};
use sea_orm::{Database, DatabaseConnection};
use similar::{ChangeTag, TextDiff};
use std::fmt;

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:>4}", idx + 1),
        }
    }
}

pub async fn execute_diff() -> Result<(), Error> {
    let (mut config, filename, content) = verify_if_config_file_is_present()?;

    let home = std::env::var("HOME").unwrap();
    let crosup_dir = format!("{}/.config/crosup", home);

    let database_url = format!("sqlite:{}/modifications.sqlite3?mode=rwc", crosup_dir);

    let db: DatabaseConnection = Database::connect(&database_url).await?;
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

fn print_diff(previous: &str, current: &str) {
    let diff = TextDiff::from_lines(previous, current);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            println!("{:-^1$}", "-", 80);
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red().bold()),
                    ChangeTag::Insert => ("+", Style::new().green().bold()),
                    ChangeTag::Equal => (" ", Style::new()),
                };
                print!("{}|{}", Line(change.new_index()), style.style(sign),);
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        print!("{}", style.style(value).underline());
                        continue;
                    }
                    print!("{}", style.style(value));
                }
                if change.missing_newline() {
                    println!();
                }
            }
        }
    }
}
