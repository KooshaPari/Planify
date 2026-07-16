# agileplus-sqlite

Canonical SQLite storage backend for the AgilePlus intent-graph
workspace. Implements the shared `Storage` trait used by the CLI,
HTTP server, MCP server, and bulk import/export pipelines.

## Purpose

`agileplus-sqlite` is the only persistence implementation in the
workspace. It owns the schema migrations, the typed `Storage` API
(create, list, dump, delete, query, tag, note), bulk import and
export, and the connection pooling used by long-running servers.

## Installation

This is a library crate. There is no binary to install. Add it to
your `Cargo.toml`:

```toml
[dependencies]
agileplus-sqlite = "0.3"
```

Or build the workspace from source:

```bash
cargo build -p agileplus-sqlite
```

## Minimal Usage

Open a database, store a graph, list it back, then close:

```rust
use agileplus_sqlite::{Storage, StorageConfig};
use agileplus_domain::intent_graph::IntentGraphBuilder;

let store = Storage::open(StorageConfig::new("./g.db"))?;
let graph = IntentGraphBuilder::new("Build OAuth2 login")
    .feature("Login endpoint")
    .build();

let id = store.create(&graph)?;
let graphs = store.list()?;
assert_eq!(graphs.len(), 1);
store.close()?;
```

Bulk import a directory of `graph.json` files:

```rust
let count = store.bulk_import_dir("./incoming/", true /* recursive */)?;
println!("imported {count} graphs");
```

## Crate Layout

- `src/lib.rs` — `Storage`, `StorageConfig`, the trait surface.
- `src/migrations/` — versioned, idempotent SQL migrations.
- `src/queries/` — typed query builders used by the CLI and server.

## API Reference

See the [workspace `docs/`](../../docs/README.md) for the bulk
import/export contract and the migration policy.

## License

MIT — see [`LICENSE`](../../LICENSE).