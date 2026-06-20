# G2 chokepoint audit — phenotype-go-sdk

**Date:** 2026-06-17  
**Chokepoint:** `phenotype-go-sdk` (registry/chokepoints.json)  
**Blocks sources:** McpKit

## Findings

| Check | Result |
|-------|--------|
| HexaKit `phenotype-mcp` git/path deps | **None** |
| McpKit packages | Local under `packages/mcpkit/` with repository → `KooshaPari/McpKit` |
| Cross-package refs | Self-contained Go/Rust/Python McpKit workspace |

## Status

**verified-clean** — McpKit domain already canonical in-repo; no HexaKit monorepo paths.
