# Phenotype Rust SDK

> **Core language** — memory-safe, zero-cost abstractions for the Phenotype ecosystem.

[![Cargo](https://img.shields.io/badge/rustc-1.75%2B-orange)](https://rustup.rs/)

## Overview

The Rust workspace provides foundational primitives: type systems, serialization,
identity, cryptography, and the cross-language ABI layer. All crates compile to
a stable C ABI (`cdylib`) for interop with application-layer languages.

## Workspace Structure

```
lang/rust/
├── Cargo.toml              # Workspace root
├── packages/
│   ├── phenotype-core      # Re-export hub, foundational types, Error, Result
│   ├── phenotype-async-traits    # AsyncIterator, Future helpers, AsyncDrop
│   ├── phenotype-cache-adapter   # Multi-backend cache (moka, lru, dashmap)
│   ├── phenotype-cost-core       # Cost analysis, budgeting, tracking
│   ├── phenotype-crypto          # SHA-2, BLAKE3, AES-GCM, Ed25519, HMAC
│   ├── phenotype-git-core        # Git porcelain operations
│   ├── phenotype-http-client-core # Connection pooling, retry, HTTP patterns
│   ├── phenotype-iter            # Extended iterators, adapters, parallel utils
│   ├── phenotype-macros          # Proc macros: error derive, builder
│   ├── phenotype-process         # Process management, signals, supervision
│   ├── phenotype-rate-limit      # Token bucket, sliding window rate limiting
│   ├── phenotype-retry           # Configurable retry with backoff policies
│   ├── phenotype-string          # Normalization, sanitization, compression
│   ├── phenotype-test-infra      # Test fixtures, BDD helpers, assertions
│   ├── phenotype-time            # Duration, timestamp, time constants
│   └── phenotype-validation      # Input validation, constraint checking
```

## Quick Start

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Check without building
cargo check --workspace

# Lint
cargo clippy --workspace -- -D warnings
```

## Adding a New Crate

```bash
cargo new packages/<name>
# Or add to workspace:
#   echo '    "packages/<name>",' >> Cargo.toml
```

## Conventional Commits

This workspace uses squash-merge with conventional commit messages:

```
feat(crypto): add Ed25519 key generation
fix(time): correct DST boundary in timestamp
chore(deps): bump serde to 1.0.200
```

See [ORIGIN.md](./ORIGIN.md) for crate provenance.

## License

MIT — see [../../LICENSE](../../LICENSE) for details.
