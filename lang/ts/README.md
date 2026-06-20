# Phenotype TypeScript SDK

> **Application language** — web UIs, SDK consumers, edge functions for the Phenotype ecosystem.

[![Node](https://img.shields.io/badge/node-22%2B-339933?logo=node.js)](https://nodejs.org/)

## Overview

The TypeScript workspace provides browser and edge-runtime SDKs for consuming
Phenotype APIs, managing authentication, and building web interfaces that
integrate with Phenotype backends.

## Workspace Structure

```
lang/ts/
├── package.json          # npm workspace root
├── tsconfig.json         # TypeScript configuration
└── packages/             # npm workspace members (coming soon)
```

## Quick Start

```bash
# Install dependencies
npm install

# Build
npm run build

# Test
npm test
```

## Adding a New Package

```bash
mkdir -p packages/<name>/src
cd packages/<name>
npm init
```

## Status

The TypeScript SDK is under active development. Packages will be added as
workspace members as they are published.

## License

MIT — see [../../LICENSE](../../LICENSE) for details.
