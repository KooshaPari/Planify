# justfile for phenotype-go-sdk
# Standard recipes for the Hygiene bundle (2026-06-08).
# https://github.com/casey/just

set dotenv-load := true
set shell := ["bash", "-uc"]
set positional-arguments := true

# Default: list available recipes.
_default:
	@just --list

# Sync the Go workspace.
sync:
	go work sync

# Build all packages in the workspace.
build:
	go build ./...

# Run vet + the test suite.
test:
	go vet ./...
	go test ./...

# Format the workspace.
format:
	gofmt -s -w .
	goimports -w .

# Lint with staticcheck (when installed) and go vet.
lint:
	go vet ./...
	staticcheck ./... || true

# Tidy go.mod / go.sum for every module.
tidy:
	find . -name go.mod -not -path './vendor/*' -execdir go mod tidy \;

# Clean test caches.
clean:
	go clean -testcache
	rm -rf dist

# CI target: vet + build + test.
ci: lint build test
