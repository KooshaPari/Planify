# justfile for Planify (plane fork) — https://just.systems
# Run `just` (or `just default`) to list recipes.

set dotenv-load
set shell := ["bash", "-uc"]

default:
    @just --list

# Start the dev server via turbo
dev:
    pnpm dev

# Produce release artifacts
build:
    pnpm build

# Run the test suite (no top-level test script — defer to turbo)
test:
    pnpm test || pnpm turbo run test

# Lint the project
lint:
    pnpm check:lint

# Apply formatter
fmt:
    pnpm fix:format

# Remove build artifacts and caches
clean:
    pnpm clean
