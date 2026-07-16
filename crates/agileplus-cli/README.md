# agileplus-cli

Command-line entry point for the AgilePlus intent-graph workspace:
convert prompts to graphs, validate them, persist into SQLite, list,
dump, delete, query, tag, and annotate.

## Purpose

`agileplus-cli` is the developer-facing front end for AgilePlus. It wraps
every operation the workspace supports into a single `agileplus` binary
with subcommands (`intent`, `validate`, `store`, `list`, `dump`, `delete`,
`query`, `import`, `export`, `tag`, `note`) so you can drive the
intent-graph lifecycle without writing Rust.

## Installation

Install the prebuilt binary from crates.io:

```bash
cargo install agileplus-cli
```

Or build from source at the workspace root:

```bash
cargo build --release -p agileplus-cli
```

The binary is placed at `target/release/agileplus`.

## Minimal Usage

A full prompt-to-persistence round-trip:

```bash
# 1. Convert a prompt to a validated intent graph (writes graph.json).
agileplus intent --prompt "Build an OAuth2 authentication system with login and session management."

# 2. Persist the graph into a local SQLite database.
agileplus store --db ./g.db --input graph.json
```

Inspect what is stored:

```bash
agileplus list --db ./g.db                    # List all graph ids.
agileplus dump  --db ./g.db --id <GRAPH_ID>    # Write graph JSON to stdout.
agileplus query --db ./g.db --kind Intent      # Query nodes by type.
```

Add structured metadata to a stored graph:

```bash
agileplus tag  --db ./g.db --id <GRAPH_ID> --tag backend
agileplus note --db ./g.db --id <GRAPH_ID> --body "Round-trip OK on 2026-06-24"
```

See `agileplus --help` and `agileplus <subcommand> --help` for the full
flag list, including bulk `import` / `export-all` for batch migrations
across databases.

## How It Works

`agileplus-cli` delegates all heavy lifting to the workspace's library
crates:

- [`agileplus-domain`](../agileplus-domain) — graph construction, node
  and edge types, the `IntentGraphBuilder` API.
- [`agileplus-trace-validator`](../agileplus-trace-validator) — ontology,
  DAG acyclicity, and edge-constraint checks. The `intent` subcommand
  runs the validator before writing output.
- [`agileplus-sqlite`](../agileplus-sqlite) — the canonical `Storage` API
  used by `store`, `list`, `dump`, `delete`, `query`, `tag`, and `note`.

## API Reference

The subcommand surface is defined in
[`crates/agileplus-cli/src/main.rs`](../agileplus-cli/src/main.rs) and the
[`commands/`](../agileplus-cli/src/commands/) module tree. For the
underlying Rust API, see the [workspace `docs/`](../../docs/README.md).

## License

MIT — see [`LICENSE`](../../LICENSE).