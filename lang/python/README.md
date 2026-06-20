# Phenotype Python SDK

> **Application language** — data science, AI/ML, scripting for the Phenotype ecosystem.

[![Python](https://img.shields.io/badge/python-3.10%2B-3776AB?logo=python)](https://python.org/)
[![uv](https://img.shields.io/badge/uv-latest-8B5CFE)](https://docs.astral.sh/uv/)

## Overview

The Python workspace provides data and AI/ML primitives: authentication kits,
data processing, MCP integration, observability, resilience patterns, and
testing utilities — wrapping core primitives from Rust/Zig crates via FFI.

## Workspace Structure

```
lang/python/
├── pyproject.toml          # uv workspace root
├── uv.lock                 # Lockfile
├── justfile               # Python-specific task runner
├── mcp/
│   └── agentmcp            # Agent MCP framework
├── packages/
│   ├── agentmcp-hex            # Hexagonal DDD adapter layer for MCP
│   ├── auth-kit                # Authentication toolkit
│   ├── data-kit                # Data processing toolkit
│   ├── mcp-kit                 # MCP integration kit
│   ├── observability-kit       # Observability & telemetry
│   ├── pheno-cli-builder       # CLI builder framework
│   ├── pheno-cli-kit           # CLI utilities
│   ├── phenokit-config-kit     # Configuration kit
│   ├── phenotype-config        # Configuration management
│   ├── phenotype-id            # ID generation
│   ├── phenotype-logging       # Structured logging
│   ├── phenotype-py-kit        # General-purpose Python kit
│   ├── phenotype-testing       # Testing utilities
│   ├── resilience-kit          # Resilience patterns (circuit breaker, retry)
│   └── testing-kit             # Testing framework
```

## Quick Start

```bash
# Install all workspace packages
uv sync

# Run lint
uv run ruff check .
uv run ruff format --check .

# Format
uv run ruff format .

# Run tests
uv run pytest

# Type-check
uv run mypy .

# Build
uv build
```

## Adding a New Package

```bash
cd lang/python
uv init --package packages/<name>
```

Then add to `pyproject.toml` under `[tool.uv.sources]` and `[tool.uv.workspace]`.

## License

MIT — see [../../LICENSE](../../LICENSE) for details.
