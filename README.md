# AgilePlus

> Convert prompts into validated intent graphs, persist them locally,
> query them from anywhere.

[![CI](https://img.shields.io/github/actions/workflow/status/phenotype/agileplus/ci.yml?branch=main&style=flat-square)](https://github.com/phenotype/agileplus/actions)
[![crates.io](https://img.shields.io/crates/v/agileplus-cli?style=flat-square)](https://crates.io/crates/agileplus-cli)
[![version](https://img.shields.io/badge/version-0.3.0-blue?style=flat-square)](CHANGELOG.md)
[![license](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

AgilePlus is an intent-graph workspace. A prompt goes in, a typed DAG
of Intent, Feature, Plan, Story, Bug, Metric, and Hypothesis nodes
comes out, gets validated against the AgilePlus ontology, and lands
in a local SQLite database that the CLI, HTTP server, MCP server, web
UI, and plugin system can all read and write.

## Quickstart

Install the CLI and run the canonical prompt-to-persistence
round-trip:

```bash
# 1. Install the CLI.
cargo install agileplus-cli

# 2. Convert a prompt into a validated intent graph (writes graph.json).
agileplus intent --prompt "Build an OAuth2 authentication system with login and session management."

# 3. Persist the graph into a local SQLite database.
agileplus store --db ./g.db --input graph.json
```

That gives you a queryable, validated, persisted intent graph in a
single SQLite file with no external services required. Run
`agileplus --help` to see every subcommand, or jump to
[`docs/roadmap.md`](docs/roadmap.md) for the long view.

## Architecture

Four binaries and three library crates, all in one Cargo workspace.
The library crates own the domain logic; the binaries are thin
transport adapters that compose them.

```
            ┌─────────────────────────────────────────────┐
            │              Binaries (entry points)        │
            │                                             │
            │   agileplus-cli      agileplus-server      │
            │   (CLI front end)    (HTTP API + SSE)      │
            │                                             │
            │   agileplus-mcp-intent                      │
            │   (MCP tool server / prompt-to-graph)       │
            └────────────┬──────────────┬────────────────┘
                         │              │
                         ▼              ▼
            ┌─────────────────────────────────────────────┐
            │          Library crates (pure logic)       │
            │                                             │
            │   agileplus-trace-validator                │
            │   (ontology, DAG, edge constraints)        │
            │                                             │
            │   agileplus-sqlite                         │
            │   (Storage + migrations + tagged/notes)    │
            └────────────┬────────────────────────────────┘
                         │
                         ▼
            ┌─────────────────────────────────────────────┐
            │   agileplus-domain (foundational types)     │
            │                                             │
            │   NodeType, Edge, Meta, builder API,        │
            │   query/ops, ontology_ext (Stories, Bugs,  │
            │   Metrics, Hypotheses)                      │
            └─────────────────────────────────────────────┘
```

- **Domain types** are pure Rust with no I/O; everything else depends
  on them.
- **Validator** is also pure; the HTTP server and CLI invoke it on
  every write.
- **Storage** is the single persistence boundary; CLI, server, and
  MCP all share the same `Storage` API.
- **Servers** add network-facing concerns: routing, SSE streaming,
  MCP transport, JSON serialization.

## Crates

| Crate | Type | Description |
|-------|------|-------------|
| [`agileplus-cli`](./crates/agileplus-cli) | bin | Command-line entry point: prompt-to-graph, store, list, dump, query, tag, note. |
| [`agileplus-server`](./crates/agileplus-server) | bin | axum-based HTTP API with SSE streaming for live graph changes. |
| [`agileplus-mcp-intent`](./crates/agileplus-mcp-intent) | bin | MCP server and HTTP API for prompt-to-intent-graph conversion. |
| [`agileplus-trace-validator`](./crates/agileplus-trace-validator) | lib | Ontology + DAG + edge-constraint validator; pure Rust, no I/O. |
| [`agileplus-domain`](./crates/agileplus-domain) | lib | Foundational types: `Node`, `Edge`, `IntentGraph`, builder API. |
| [`agileplus-sqlite`](./crates/agileplus-sqlite) | lib | Canonical SQLite storage with migrations, tags, and notes. |

Each crate has its own `README.md` with a purpose statement, install
command, minimal usage example, and links to the per-crate API docs.

## Documentation

The full architecture, ontology spec, ADR, and roadmap live under
[`docs/`](docs/README.md):

- [`docs/README.md`](docs/README.md) — index and reading order.
- [`docs/roadmap.md`](docs/roadmap.md) — Phase 0 (shipped) → Phase 5 (planned).
- [`docs/spec/intent-graph-ontology.md`](docs/spec/intent-graph-ontology.md) — formal v1.0.0 ontology.
- [`docs/research/ontology-expansion.md`](docs/research/ontology-expansion.md) — Phase 1 ontology work.
- [`docs/adr/0001-shard-lock-dag.md`](docs/adr/0001-shard-lock-dag.md) — shard-lock concurrency ADR.

## Versioning and Releases

We follow [Semantic Versioning](https://semver.org/). The current
release is **v0.3.0** (see [`CHANGELOG.md`](CHANGELOG.md)). The
release pipeline (`.github/workflows/release.yml`) builds auditable
binaries for all four binaries on every `v*` tag, attaches them to a
GitHub Release, and publishes all crates to crates.io in dependency
order using `cargo-workspaces`.

## License

This workspace is licensed under the MIT License — see
[`LICENSE`](LICENSE) for the full text. Copyright (c) 2026
Koosha Pari.

## Contributing

Contributions are welcome via pull requests against `main`. Read the
shard-lock ADR at
[`docs/adr/0001-shard-lock-dag.md`](docs/adr/0001-shard-lock-dag.md)
before opening multi-agent work — the workspace uses a 3-shard
allow-list to keep concurrent edits collision-free.