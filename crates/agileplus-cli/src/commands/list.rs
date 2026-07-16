use std::path::PathBuf;

use agileplus_sqlite::Storage;
use anyhow::{Context, Result};
use clap::Args;

/// List all intent graph ids stored in a SQLite database.
#[derive(Debug, Args)]
#[command(name = "list")]
pub struct ListCommand {
    /// Path to the SQLite database.
    #[arg(short, long)]
    db: PathBuf,
}

impl ListCommand {
    pub fn run(&self) -> Result<()> {
        let storage = Storage::open(&self.db)
            .with_context(|| format!("Failed to open storage at: {}", self.db.display()))?;

        let ids = storage
            .list_graphs()
            .with_context(|| format!("Failed to list graphs in: {}", self.db.display()))?;

        if ids.is_empty() {
            println!("(no graphs stored in {})", self.db.display());
            return Ok(());
        }

        println!("{} graph(s) in {}:", ids.len(), self.db.display());
        for id in &ids {
            println!("  {id}");
        }
        Ok(())
    }
}
