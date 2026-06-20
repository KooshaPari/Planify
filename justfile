# Phenotype SDK — Cross-language build & task runner
# Usage: just <recipe>

# ─── Toolchain Checks ───────────────────────────────────────────

_rustup_available := ""
_zig_available := ""
_go_available := ""
_uv_available := ""
_node_available := ""

[private]
has-rust: && (
    _rustup_available := if "" == "" { "1" } else { "" }
)
    which rustup >/dev/null 2>&1
    @echo "✓ rustup found"

[private]
has-zig:
    which zig >/dev/null 2>&1
    @echo "✓ zig found"

[private]
has-go:
    which go >/dev/null 2>&1
    @echo "✓ go found"

[private]
has-uv:
    which uv >/dev/null 2>&1
    @echo "✓ uv found"

[private]
has-node:
    which node >/dev/null 2>&1
    @echo "✓ node found"

# ─── Doctor — Check All Toolchains ──────────────────────────────

doctor: has-rust has-zig has-go has-uv has-node
    @echo ""
    @echo "── Toolchain Versions ──"
    rustc --version
    zig version
    go version
    uv --version
    node --version
    just --version

# ─── Rust ───────────────────────────────────────────────────────

build-rust:
    cd lang/rust && cargo build --workspace

test-rust:
    cd lang/rust && cargo test --workspace

fmt-rust:
    cd lang/rust && cargo fmt --all

lint-rust:
    cd lang/rust && cargo clippy --workspace -- -D warnings

clean-rust:
    cd lang/rust && cargo clean

# ─── Zig ────────────────────────────────────────────────────────

build-zig:
    cd lang/zig && zig build

test-zig:
    cd lang/zig && zig build test

fmt-zig:
    cd lang/zig && zig fmt packages/**/*.zig

lint-zig: test-zig

clean-zig:
    cd lang/zig && rm -rf zig-cache zig-out

# ─── Go ─────────────────────────────────────────────────────────

build-go:
    cd lang/go && go build ./...

test-go:
    cd lang/go && go test ./...

fmt-go:
    cd lang/go && go fmt ./...

lint-go:
    cd lang/go && go vet ./...

clean-go:
    cd lang/go && go clean -cache

# ─── Python ─────────────────────────────────────────────────────

build-python:
    cd lang/python && uv build

test-python:
    cd lang/python && uv run pytest

fmt-python:
    cd lang/python && uv run ruff format

lint-python:
    cd lang/python && uv run ruff check

clean-python:
    cd lang/python && rm -rf dist .ruff_cache

# ─── TypeScript ─────────────────────────────────────────────────

build-ts:
    cd lang/ts && npm install && npm run build

test-ts:
    cd lang/ts && npm test

fmt-ts:
    cd lang/ts && npx prettier --write packages/

lint-ts:
    cd lang/ts && npx tsc --noEmit

clean-ts:
    cd lang/ts && rm -rf node_modules dist

# ─── Mojo ───────────────────────────────────────────────────────

build-mojo:
    @echo "Mojo build — coming soon"

test-mojo:
    @echo "Mojo tests — coming soon"

clean-mojo:
    @echo "Mojo clean — coming soon"

# ─── Cross-Language Aggregates ──────────────────────────────────

build: build-rust build-zig build-go build-python build-ts

test: test-rust test-zig test-go test-python test-ts

fmt: fmt-rust fmt-zig fmt-go fmt-python fmt-ts

lint: lint-rust lint-zig lint-go lint-python lint-ts

clean: clean-rust clean-zig clean-go clean-python clean-ts

# ─── Git Helpers ────────────────────────────────────────────────

check: doctor build test
    @echo ""
    @echo "✓ All checks passed"
