use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Canonical local default database path.
pub(crate) const DEFAULT_DB_PATH: &str = agileplus_sqlite::DEFAULT_DB_PATH;

/// Resolve an explicit override, `AGILEPLUS_DB`, or the canonical default.
pub(crate) fn resolve_db_path(override_path: Option<&Path>) -> PathBuf {
    resolve_db_path_with_env(override_path, std::env::var_os("AGILEPLUS_DB"))
}

fn resolve_db_path_with_env(override_path: Option<&Path>, env_path: Option<OsString>) -> PathBuf {
    agileplus_sqlite::resolve_db_path(override_path, env_path)
}

pub(crate) fn default_db_path() -> PathBuf {
    PathBuf::from(DEFAULT_DB_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_db_path_prefers_explicit_override() {
        let resolved = resolve_db_path_with_env(
            Some(std::path::Path::new("/tmp/explicit.db")),
            Some(OsString::from("/tmp/from-env.db")),
        );
        assert_eq!(resolved, PathBuf::from("/tmp/explicit.db"));
    }

    #[test]
    fn resolve_db_path_prefers_env() {
        assert_eq!(
            resolve_db_path_with_env(None, Some(OsString::from("/tmp/from-env.db"))),
            PathBuf::from("/tmp/from-env.db")
        );
    }

    #[test]
    fn resolve_db_path_uses_canonical_default() {
        assert_eq!(resolve_db_path_with_env(None, None), default_db_path());
    }
}
