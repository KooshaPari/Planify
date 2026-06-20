# Phenotype SDK

> **Consolidated multi-language SDK** — Rust, Zig, and Mojo as **core languages**,
> with Go, Python, and TypeScript as **application-layer wrappers**.

---

## Philosophy

### Core Languages — Rust, Zig, Mojo

These languages are responsible for **performance-critical, foundational primitives**:
type systems, serialization, identity, crypto primitives, and the cross-language
ABI layer.

| Language | Role | Why |
|----------|------|-----|
| **Rust** | Systems core — memory-safe, zero-cost abstractions | Best-in-class for safe systems programming. Cargo workspace for shared crates. |
| **Zig** | Low-level portable primitives — no libc required, compile-time execution | Ideal for embeddable cross-language libraries, WASM targets, and C ABI surfaces. |
| **Mojo** | MLIR-accelerated AI/ML primitives | Pythonic syntax with Rust-level performance for tensor ops and inference pipelines. |

All three core languages compile to a **stable C ABI** so that application-layer
languages can call them without FFI gymnastics.

### Application Languages — Go, Python, TypeScript

These languages **wrap and extend** the core. They provide ergonomic APIs,
CLI tooling, web frameworks, and data pipelines on top of the core primitives.

| Language | Role | Why |
|----------|------|-----|
| **Go** | Services, CLIs, middleware | Best for gRPC/REST services, operator tooling, and platform infrastructure. |
| **Python** | Data science, AI/ML, scripting | Ubiquitous in ML/data ecosystem. UV-based monorepo for fast dependency resolution. |
| **TypeScript** | Web UIs, SDK consumers, edge functions | Type-safe bridge between Phenotype backends and browser/edge runtimes. |

---

## Repository Structure

```
phenotype-sdk/
├── lang/
│   ├── rust/                  # Cargo workspace (core crates)
│   │   ├── Cargo.toml
│   │   ├── ORIGIN.md
│   │   └── packages/
│   │       ├── phenotype-core/           # Re-export hub & foundational types
│   │       ├── phenotype-async-traits/   # Async iterator, Future, AsyncDrop
│   │       ├── phenotype-cache-adapter/  # Multi-backend cache
│   │       ├── phenotype-cost-core/      # Cost analysis & budgeting
│   │       ├── phenotype-crypto/         # Hashing, encryption, signing
│   │       ├── phenotype-git-core/       # Git porcelain operations
│   │       ├── phenotype-http-client-core/ # HTTP connection pooling
│   │       ├── phenotype-iter/           # Extended iterators & adapters
│   │       ├── phenotype-macros/         # Procedural macros
│   │       ├── phenotype-process/        # Process supervision
│   │       ├── phenotype-rate-limit/     # Token bucket & sliding window
│   │       ├── phenotype-retry/          # Backoff & retry policies
│   │       ├── phenotype-string/         # Normalization & sanitization
│   │       ├── phenotype-test-infra/     # BDD helpers & fixtures
│   │       ├── phenotype-time/           # Duration & timestamp types
│   │       └── phenotype-validation/     # Input validation
│   │
│   ├── zig/                   # Zig build system (portable primitives)
│   │   ├── build.zig
│   │   └── packages/
│   │       └── phenotype-core/
│   │
│   ├── mojo/                  # Mojo 🔥 (MLIR-accelerated primitives)
│   │   └── packages/          # Coming soon
│   │
│   ├── go/                    # Go workspace (application services)
│   │   ├── go.work
│   │   └── packages/
│   │       ├── devhex/
│   │       ├── mcpkit/
│   │       ├── pheno-core-cgo/
│   │       ├── phenotype-go-auth/
│   │       ├── phenotype-go-cli/
│   │       ├── phenotype-go-config/
│   │       ├── phenotype-go-kit/
│   │       ├── phenotype-go-middleware/
│   │       ├── phenotype-id/
│   │       └── platformkit/
│   │
│   ├── python/                # Python monorepo (data/AI)
│   │   ├── pyproject.toml
│   │   ├── uv.lock
│   │   ├── justfile
│   │   ├── mcp/
│   │   │   └── agentmcp/          # Agent MCP framework
│   │   └── packages/
│   │       ├── agentmcp-hex/
│   │       ├── auth-kit/
│   │       ├── data-kit/
│   │       ├── mcp-kit/
│   │       ├── observability-kit/
│   │       ├── pheno-cli-builder/
│   │       ├── pheno-cli-kit/
│   │       ├── phenokit-config-kit/
│   │       ├── phenotype-config/
│   │       ├── phenotype-id/
│   │       ├── phenotype-logging/
│   │       ├── phenotype-py-kit/
│   │       ├── phenotype-testing/
│   │       ├── resilience-kit/
│   │       └── testing-kit/
│   │
│   └── ts/                    # TypeScript monorepo (web SDK)
│       ├── package.json
│       ├── tsconfig.json
│       └── packages/          # Coming soon
│
├── justfile                   # Cross-language task runner
└── README.md                  # This file
```

---

## Quick Start

### Prerequisites

| Tool | Version | For |
|------|---------|-----|
| Rust | 2024 edition (1.85+) | `lang/rust/` |
| Zig | 0.14+ | `lang/zig/` |
| Go | 1.24+ | `lang/go/` |
| Python | 3.12+ | `lang/python/` |
| [uv](https://docs.astral.sh/uv/) | latest | `lang/python/` |
| Node.js | 22+ | `lang/ts/` |
| [just](https://just.systems/) | 1.36+ | Root task runner |
| Mojo | latest | `lang/mojo/` |

### Build All Languages

```bash
just build
```

### Build a Specific Language

```bash
just build-rust
just build-zig
just build-go
just build-python
just build-ts
```

### Run Tests

```bash
just test              # all languages
just test-rust
just test-zig
just test-go
just test-python
just test-ts
```

---

## Cross-Language ABI

Core primitives (Rust, Zig, Mojo) export a stable **C ABI** via:

- **Rust**: `#[repr(C)]` types + `extern "C"` functions, compiled as `cdylib`
- **Zig**: `export fn` with `extern struct` types
- **Mojo**: `@export` decorators for MLIR-generated C-compatible symbols

Application languages consume these via:

- **Go**: `cgo` bindings in `pheno-core-cgo`
- **Python**: `ctypes` / `cffi` wrappers
- **TypeScript**: `ffi` via `bun` or `node-ffi`

---

## Development

### Workspace Commands (via `just`)

```bash
just fmt          # format all languages
just lint         # lint all languages
just clean        # clean all build artifacts
just doctor       # check toolchain availability
```

### Adding a New Package

```bash
# Rust
cd lang/rust && cargo new packages/<name>

# Zig
mkdir lang/zig/packages/<name>/src

# Go
cd lang/go && mkdir packages/<name> && cd packages/<name> && go mod init ...

# Python
cd lang/python && uv init --package packages/<name>

# TypeScript
cd lang/ts && mkdir -p packages/<name>/src
```

---

## License

MIT — see [LICENSE](LICENSE) for details.
