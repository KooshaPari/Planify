//! Phenotype SDK Core
//!
//! Foundational types and prelude for the Phenotype Rust SDK. Re-exports
//! commonly used types from workspace sibling crates so consumers can
//! depend on `phenotype-core` alone for most use cases.

/// Crate re-exports — use `phenotype_core::time::*`, etc.
pub use phenotype_cache_adapter as cache;
pub use phenotype_cost_core as cost;
pub use phenotype_crypto as crypto;
pub use phenotype_iter as iter;
pub use phenotype_process as process;
pub use phenotype_rate_limit as rate_limit;
pub use phenotype_retry as retry;
pub use phenotype_string as string;
pub use phenotype_time as time;
pub use phenotype_validation as validation;

/// Error type for core Phenotype operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("timeout: {0}")]
    Timeout(String),
    #[error("unavailable: {0}")]
    Unavailable(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("unauthenticated: {0}")]
    Unauthenticated(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Prelude module — import with `use phenotype_core::prelude::*`.
pub mod prelude {
    pub use chrono::{DateTime, Utc};
    pub use serde::{Deserialize, Serialize};
    pub use uuid::Uuid;

    pub use crate::{Error, Result};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::Internal("test".into());
        assert_eq!(format!("{}", err), "internal error: test");
    }

    #[test]
    fn test_result_type() {
        fn works() -> Result<i32> {
            Ok(42)
        }
        assert_eq!(works().unwrap(), 42);
    }
}
