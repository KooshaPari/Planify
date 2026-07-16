use std::path::PathBuf;

use agileplus_domain::intent_graph::IntentGraph;
use agileplus_sqlite::Storage;
use anyhow::{Context, Result};
use clap::Args;

/// Import intent graphs from JSON files into a SQLite database.
#[derive(Debug, Args)]
#[command(name = "import-all")]
pub struct ImportAll {
    /// Path to the SQLite database.
    #[arg(short, long)]
    pub db: PathBuf,

    /// Input directory containing JSON graph files.
    #[arg(short, long)]
    pub dir: PathBuf,
}

impl ImportAll {
    pub fn run(&self) -> Result<()> {
        let storage = Storage::open(&self.db)
            .with_context(|| format!("Failed to open storage at: {}", self.db.display()))?;

        let entries = std::fs::read_dir(&self.dir)
            .with_context(|| format!("Failed to read directory: {}", self.dir.display()))?;

        let mut count = 0;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(true, |e| e != "json") {
                continue;
            }

            let id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            let json = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read: {}", path.display()))?;

            let graph: IntentGraph = serde_json::from_str(&json)
                .with_context(|| format!("Failed to parse JSON from: {}", path.display()))?;

            storage
                .save_graph(&id, &graph)
                .with_context(|| format!("Failed to save graph {id}"))?;

            count += 1;
        }

        println!("Imported {count} graph(s) from {}", self.dir.display());
        Ok(())
    }
}
