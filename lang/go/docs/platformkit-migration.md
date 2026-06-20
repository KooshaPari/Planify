# PlatformKit migration

`KooshaPari/PlatformKit` (archived) is absorbed into this repo and [nanovms](https://github.com/KooshaPari/nanovms).

| PlatformKit path | Successor | Status (2026-06-17) |
|------------------|-----------|---------------------|
| `go/devhex/` | `packages/devhex/` (this repo) | **Absorbed** — byte-identical registry |
| `go/devenv/` | [nanovms](https://github.com/KooshaPari/nanovms) sandbox adapters | **Canonical** — PlatformKit SPEC deprecates devenv in favor of NanoVMS; do not re-import broken lowercase-path copy |
| Docs / CHARTER | This `docs/` tree | **Partial** — genesis + platformkit package docs |

## Verification

- `packages/devhex/pkg/domain/registry.go` — byte-identical to PlatformKit source
- `packages/platformkit/` — consolidated platform tooling
- `packages/mcpkit/` — MCP Go bindings from org consolidation

PlatformKit repo remains archived. **DevHex** consumers must repoint to `packages/devhex` module path. **devenv** consumers must use **nanovms** (see PlatformKit `go/devenv/SPEC.md` deprecation notice).
