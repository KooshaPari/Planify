# AGENTS.md — PlatformKit

## Project Overview

- **Name**: PlatformKit
- **Description**: Platform abstraction toolkit for cross-platform development
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/PlatformKit`
- **Language Stack**: Go (primary)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/PlatformKit

# Go components
cd go && go build ./...
```

## Architecture

```
PlatformKit/
└── go/                       # Go implementations
    └── devenv/               # Development environment (has AGENTS.md)
```

## Quality Standards

### Go Standards
- **Line length**: 100 characters
- **Formatter**: `gofmt`, `goimports`
- **Linter**: `golangci-lint`
- **Tests**: `go test ./...`

## Git Workflow

### Branch Naming
Format: `platformkit/<type>/<description>`

Examples:
- `platformkit/feat/abstraction-layer`
- `platformkit/fix/os-detection`

## File Structure

```
PlatformKit/
└── go/
    └── devenv/
        └── AGENTS.md
```

## CLI Commands

```bash
# Go operations
cd go && go build ./...
cd go && go test ./...
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Platform detection fails | Check OS-specific code |
| Check devenv AGENTS.md | For detailed guidance |

## Dependencies

- **devenv-abstraction**: Related dev environment work
- **PhenoDevOps**: Platform deployment

## Agent Notes

When working in PlatformKit:
1. Check `go/devenv/AGENTS.md` for detailed rules
2. Platform abstraction layer
3. Coordinate with deployment targets
