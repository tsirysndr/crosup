use std::fmt;

use anyhow::Error;
use owo_colors::{OwoColorize, Style};
use sea_orm::{Database, DatabaseConnection};
use similar::{ChangeTag, TextDiff};

pub mod add;
pub mod diff;
pub mod history;
pub mod init;
pub mod install;
pub mod search;

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:>4}", idx + 1),
        }
    }
}

pub async fn get_database_connection() -> Result<DatabaseConnection, Error> {
    let home = std::env::var("HOME").unwrap();
    let crosup_dir = format!("{}/.config/crosup", home);

    let database_url = format!("sqlite:{}/modifications.sqlite3?mode=rwc", crosup_dir);

    let db: DatabaseConnection = Database::connect(&database_url).await?;
    Ok(db)
}

pub fn print_diff(previous: &str, current: &str) {
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
