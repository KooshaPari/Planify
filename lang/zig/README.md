# Phenotype Zig SDK

> **Core language** — low-level portable primitives for the Phenotype ecosystem.

[![Zig](https://img.shields.io/badge/zig-0.14%2B-F7A41D?logo=zig)](https://ziglang.org/)

## Overview

Zig provides embeddable cross-language libraries, WASM targets, and C ABI
surfaces — no libc required, compile-time execution for zero-cost abstractions.

## Workspace Structure

```
lang/zig/
├── build.zig                    # Build system root
└── packages/
    └── phenotype-core/          # Core primitives (in development)
        └── src/main.zig
```

## Quick Start

```bash
# Build
zig build

# Run tests
zig build test

# Format
zig fmt packages/**/*.zig
```

## Status

The Zig SDK is under active development. Core primitives will include:
- C ABI exports for interoperability with Go/Python/TS
- WASM targets for edge runtime deployment
- Cross-language type definitions

## License

MIT — see [../../LICENSE](../../LICENSE) for details.
