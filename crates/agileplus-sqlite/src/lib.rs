pub mod migrations;
pub mod storage;

use std::path::Path;

pub use storage::{Note, Storage};

/// Apply all pending migrations to the given SQLite database path.
pub fn apply_migrations(db_path: impl AsRef<Path>) -> anyhow::Result<()> {
    let storage = Storage::open(&db_path)?;
    storage.migrate()?;
    Ok(())
}

/// Returns the list of (id, sql) migration pairs in apply order.
pub fn migration_files() -> &'static [(&'static str, &'static str)] {
    migrations::ALL
}
