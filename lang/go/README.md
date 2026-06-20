# Phenotype Go SDK

> **Application language** — services, CLIs, middleware for the Phenotype ecosystem.

[![Go](https://img.shields.io/badge/go-1.24+-00ADD8?logo=go)](https://go.dev/)

## Overview

The Go workspace provides service-layer primitives: authentication, CLI tooling,
configuration management, middleware, and platform orchestration — wrapping
core primitives from the Rust/Zig crates via CGo.

## Workspace Structure

```
lang/go/
├── go.work          # Go workspace root
├── justfile         # Go-specific task runner
├── packages/
│   ├── devhex                  # Dev environment abstraction
│   ├── mcpkit                  # MCP kit (Go runtime)
│   ├── pheno-core-cgo          # CGo bindings to Rust/Zig core
│   ├── phenotype-go-auth       # Authentication & token management
│   ├── phenotype-go-cli        # CLI framework & utilities
│   ├── phenotype-go-config     # Configuration management
│   ├── phenotype-go-kit        # General-purpose Go kit
│   ├── phenotype-go-middleware # HTTP/gRPC middleware
│   ├── phenotype-id            # ID generation (UUID, ULID)
│   └── platformkit             # Platform orchestration toolkit
```

## Quick Start

```bash
# Build all packages
go build ./...

# Run all tests
go test ./...

# Format
go fmt ./...

# Vet
go vet ./...
```

## Adding a New Package

```bash
mkdir packages/<name>
cd packages/<name>
go mod init github.com/KooshaPari/<name>
```

Then add to `go.work`:

```
use ./packages/<name>
```

## License

MIT — see [../../LICENSE](../../LICENSE) for details.
