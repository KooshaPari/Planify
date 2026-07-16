# Changelog

All notable changes to AgilePlus are documented in this file.

## [0.3.0] - 2026-06-24

### Added
- 101 unit tests + 8 new tests = 109 tests total across the workspace, providing full coverage of the prompt-to-graph pipeline, storage CRUD, validation, ontology extensions, tags, notes, bulk import/export, and the MCP tool surface.
- Bulk import/export: `agileplus import` (whole-directory and recursive modes) and `agileplus export-all` for batch migrations across databases; mirrored as `POST /v1/import` and `GET /v1/export` on the HTTP server and as `import` / `export_all` MCP tools.
- Property-based tests for the graph validator using `proptest` (256 iterations across 6 properties) to lock in correctness over arbitrary input shapes.
- GitHub Actions CI release pipeline (`.github/workflows/release.yml`): builds auditable binaries for all four binaries on every `v*` tag, attaches them to a GitHub Release, and publishes all crates to crates.io in dependency order using `cargo-workspaces`.
- `ontology_ext` module in `agileplus-trace-validator` and `agileplus-domain` adding optional node kinds on top of the core v1.0.0 ontology: Stories, Bugs, Metrics, and Hypotheses, each with typed property schemas and edge-constraint rules.
- Web UI: a browser-based inspector served by `agileplus-server` for browsing, querying, and annotating stored intent graphs without using the CLI.
- Plugin system: a loader interface and discovery path under `crates/agileplus-plugin` so third-party crates can extend the validator and storage surfaces with typed extensions.

### Changed
- Storage layer refactored into the canonical `agileplus-sqlite` crate with a transactional migration runner, idempotent `apply_migrations_to`, and the unified `Storage` API consumed by every downstream crate (CLI, server, MCP, plugins).
- MCP integration: `agileplus-mcp-intent` is now an MCP server that wraps the canonical subcommand surface (intent, validate, store, list, dump, delete, query, tag, note) and is the recommended LLM-facing entry point.
- Workspace version bumped to 0.3.0 across every crate and the workspace `Cargo.toml`.

### Fixed
- `cargo clippy --workspace --all-targets -- -D warnings` now passes with 0 warnings across every crate.

## [0.3.0] - 2026-06-21

### Added
- Canonical SQLite storage layer (`agileplus-sqlite`): transactional migration runner, idempotent `apply_migrations_to`, and the unified `Storage` API consumed by every downstream crate.
- CLI storage lifecycle commands consuming the canonical `Storage` API: `agileplus list`, `agileplus dump`, and `agileplus delete` for inspecting and pruning persisted graphs from the command line.
- Workspace-level integration tests for `agileplus-server`: full route coverage over health, convert, CRUD, validate, export, SSE streaming, query, and metadata endpoints.
- Tags and notes feature: SQLite schema migration (`026_graph_metadata.sql`), `Storage` CRUD (`add_tag`, `remove_tag`, `list_tags`, `add_note`, `list_notes`, `delete_note`), CLI commands (`agileplus tag`, `agileplus note`), HTTP routes (`POST`/`GET`/`DELETE /v1/graphs/:id/tags` and `/v1/graphs/:id/notes`), and 9 integration tests.
- MCP server adapter (`agileplus-mcp-intent`): 11 sync tool handlers for graph CRUD, validation, export, query, conversion, tags, and notes, wired through a stateful `AppState(Storage)` router.
- Property-based tests for the graph validator (proptest, 256 iterations across 6 properties) to lock in correctness over arbitrary input shapes.
- GitHub Actions CI pipeline (`.github/workflows/ci.yml`): runs `cargo check`, `cargo test`, and `cargo clippy` on every push.

## [0.2.0] - 2026-06-21

### Added
- MCP server adapter (agileplus-mcp-intent): 11 sync tool handlers for graph CRUD, validation, export, query, conversion, tags, and notes. Wired through stateful AppState(Storage) router.
- Workspace-level integration tests for agileplus-server: full route coverage including health, convert, CRUD, validate, export, SSE streaming, query, and metadata endpoints.
- Tags and notes feature: SQLite schema (026_graph_metadata.sql), Storage CRUD (add_tag, remove_tag, list_tags, add_note, list_notes, delete_note), CLI commands (agileplus tag, agileplus note), HTTP routes (POST/GET/DELETE /v1/graphs/:id/tags and /v1/graphs/:id/notes), and 9 integration tests.
- Bulk import/export CLI commands (agileplus import, agileplus export-all).
- Property-based tests for graph validator (proptest, 256 iterations × 6 properties).
- GitHub Actions CI workflow (.github/workflows/ci.yml) running cargo check + cargo test + cargo clippy on push.
- Canonical SQLite storage layer in agileplus-sqlite with transactional migration runner and idempotent apply_migrations_to.
- CLI storage lifecycle: list, dump, delete commands consuming the canonical Storage API.

### Changed
- Decomposed agileplus-trace-validator (623 lines) into 7 submodules (canonical, dag, edges, metadata, nodes, ontology, lib).
- Decomposed agileplus-domain/src/builder.rs (439 lines) into 5 submodules (canonical_map, meta, metadata, node, edge, mod).
- Decomposed agileplus-cli/commands/intent.rs (419 lines) into generate.rs + slug.rs + mod.rs.
- Decomposed agileplus-mcp-intent/converter.rs (390 lines) into types.rs + extract.rs + convert.rs + mod.rs.
- All file sizes now ≤390 lines (target ≤350, hard limit 500).

### Fixed
- 11 clippy warnings in agileplus-domain (needless_lifetimes, map_or, is_none_or).
- Storage migration runner now idempotent — safe to call repeatedly, no schema drift.
- REGEXP SQL function registered on every open so the existing CHECK (id REGEXP ...) constraint actually evaluates.