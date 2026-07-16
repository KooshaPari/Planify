# agileplus-domain

Core domain model for the AgilePlus intent graph: node kinds, edge
kinds, property schemas, and the `IntentGraphBuilder` API used by
every other workspace crate.

## Purpose

`agileplus-domain` defines the data shapes that flow through the rest
of the workspace. It owns the node and edge enums, the typed property
schemas (Intent, Feature, Plan, plus optional Stories, Bugs, Metrics,
and Hypotheses from `ontology_ext`), and the `IntentGraphBuilder` that
the CLI, server, MCP, and web UI all use to construct graphs from a
prompt or from user input.

## Installation

This is a library crate. There is no binary to install. Add it to
your `Cargo.toml`:

```toml
[dependencies]
agileplus-domain = "0.3"
```

Or build the workspace from source:

```bash
cargo build -p agileplus-domain
```

## Minimal Usage

Build a small intent graph programmatically:

```rust
use agileplus_domain::intent_graph::IntentGraphBuilder;

let graph = IntentGraphBuilder::new("Build OAuth2 login")
    .feature("Login endpoint")
    .feature("Session manager")
    .plan("OAuth2 plan")
    .build();

println!("{}", serde_json::to_string_pretty(&graph).unwrap());
```

The builder returns an `IntentGraph` value that can be handed to
[`agileplus-trace-validator`](../agileplus-trace-validator) for
validation and to [`agileplus-sqlite`](../agileplus-sqlite) for
persistence.

## Crate Layout

- `src/intent_graph/` — graph, nodes, edges, builders, ontology
  helpers.
- `src/intent_graph/ontology_ext.rs` — optional node kinds
  (Stories, Bugs, Metrics, Hypotheses) shared with the validator.

## API Reference

See the [workspace `docs/`](../../docs/README.md) for the typed
schemas and the ontology spec.

## License

MIT — see [`LICENSE`](../../LICENSE).