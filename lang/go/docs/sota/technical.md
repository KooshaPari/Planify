# Technical — SOTA (phenotype-go-sdk)

## Use case

Deliver consolidated Go platform modules (devhex, devenv abstractions, MCP Go workspace) as the canonical `platform` role owner for Phenotype-org.

**AgilePlus / FR trace:** ADR-011 Go convergence

## Requirements

| Requirement | Weight |
|-------------|--------|
| Single canonical module path per platform concern | must |
| `go work sync` clean for active packages | must |
| Tier 3 Go justification documented | must |
| Genesis governance linked from root | must |
| Rust rewrite of devenv without SOTA | nice |

## Language placement

| Component | Lang | Tier | Rationale |
|-----------|------|------|-----------|
| devhex / devenv-abstraction | Go | 3 | Existing Go module surface; dev-tooling ecosystem; PlatformKit absorption |
| mcpkit Go workspace | Go | 3 | MCP Go SDK ecosystem; deferred until modules restored |
| platformkit | docs only | — | Specs retained; Go dupes removed per ADR-011 |
| Future platform core | Rust | 1 | Only if SOTA shows parity and migration plan |

## Alternatives considered

| Alternative | Type | Pros | Cons | Verdict |
|-------------|------|------|------|---------|
| Keep PlatformKit standalone | internal | zero migration | duplicate modules, agent confusion | rejected — ADR-011 |
| Rust rewrite of devenv | internal | Tier 1 alignment | high cost; breaks existing Go consumers | rejected — no consumer migration plan |
| Split Go modules per GitHub repo | internal | isolation | N× governance; module path drift | rejected |
| Python platform kit | internal | uv velocity | wrong tier for devenv/K8s adjacency | rejected |
| **Go workspace monorepo (`phenotype-go-sdk`)** | chosen | absorption + single charter | Tier 3 edge requires SOTA maintenance | **chosen** |

Research sources: [phenotype-registry LANGUAGE_PLACEMENT](https://github.com/KooshaPari/phenotype-registry/blob/main/LANGUAGE_PLACEMENT.md), ADR-011, PlatformKit audit.

## Chosen strategy

Root `go.work` includes active packages (`packages/devhex` today). Absorbed PlatformKit Go duplicates removed; platformkit holds docs/specs. McpKit Go deferred until `pheno-mcp-*` modules compile. All new Go surfaces require Tier 3 paragraph in this file before merge.

Link: [charter.md](../../../charter.md) · [intent.md](../../../intent.md)

## Evolution triggers

Re-open this dimension when:

- McpKit Go modules restored
- Rust platform crate reaches feature parity with devhex
- Go workspace std or module policy changes fleet layout

Update [alternatives.md](alternatives.md) and [../../../SOTA.md](../../../SOTA.md) executive table when verdict changes.
