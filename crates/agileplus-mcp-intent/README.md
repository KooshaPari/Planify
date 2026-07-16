# agileplus-mcp-intent

Model Context Protocol (MCP) server that exposes AgilePlus intent-graph
operations (intent, validate, store, list, dump, delete, query, tag,
note) to MCP-aware clients such as Claude Desktop and other LLM tools.

## Purpose

`agileplus-mcp-intent` lets an LLM agent call into the AgilePlus
workspace as if it were a tool provider. The agent can ask for an
intent graph from a prompt, validate it against the workspace
ontology, persist it, query it, annotate it, and roll it back — all
through the standard MCP `tools/call` interface.

## Installation

Install the MCP server binary from crates.io:

```bash
cargo install agileplus-mcp-intent
```

Or build from source at the workspace root:

```bash
cargo build --release -p agileplus-mcp-intent
```

The binary is placed at `target/release/agileplus-mcp-intent`.

## Minimal Usage

Run the MCP server against a local SQLite database:

```bash
agileplus-mcp-intent --db ./g.db
```

Wire it into an MCP-aware client (for example Claude Desktop) by
adding a server entry to the client's `mcp_servers` config:

```json
{
  "mcp_servers": {
    "agileplus": {
      "command": "agileplus-mcp-intent",
      "args": ["--db", "/absolute/path/to/g.db"]
    }
  }
}
```

After registration, the following tools become available to the
agent: `intent`, `validate`, `store`, `list`, `dump`, `delete`,
`query`, `tag`, `note`. Each tool returns a JSON envelope.

## How It Works

`agileplus-mcp-intent` is a thin MCP transport layer that delegates to
the same library crates used by the CLI and HTTP server.

- [`agileplus-domain`](../agileplus-domain) — graph construction, the
  `IntentGraphBuilder` API, node and edge types.
- [`agileplus-trace-validator`](../agileplus-trace-validator) —
  validation, ontology checks, DAG acyclicity.
- [`agileplus-sqlite`](../agileplus-sqlite) — persistence and queries.
- [`agileplus-cli`](../agileplus-cli) — the canonical subcommand
  implementations that the MCP tools wrap.

## API Reference

Tool schemas live in
[`crates/agileplus-mcp-intent/src/main.rs`](../agileplus-mcp-intent/src/main.rs).
For the underlying Rust API, see the
[workspace `docs/`](../../docs/README.md).

## License

MIT — see [`LICENSE`](../../LICENSE).