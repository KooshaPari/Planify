#!/usr/bin/env bash
# post-create.sh — runs once after the devcontainer is built.
# Installs rustup components and the per-crate cargo tools the v22-T4 fleet
# expects (clippy, rustfmt, cargo-deny, cargo-nextest, cargo-mutants).
set -euo pipefail

echo "[post-create] rustup components: clippy, rustfmt"
rustup component add clippy rustfmt

echo "[post-create] cargo install: cargo-deny, cargo-nextest, cargo-mutants"
# cargo-deny + cargo-nextest are also baked into the Dockerfile; running
# `cargo install` here is idempotent and keeps them aligned with the latest
# compatible version on PATH.
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked cargo-mutants

echo "[post-create] verifying toolchain"
{
  echo "rustc      : $(rustc --version)"
  echo "cargo      : $(cargo --version)"
  echo "clippy     : $(cargo clippy --version)"
  echo "rustfmt    : $(cargo fmt --version)"
  echo "cargo-deny : $(cargo deny --version)"
  echo "nextest    : $(cargo nextest --version)"
  echo "mutants    : $(cargo mutants --version)"
} | sed 's/^/  /'

echo "[post-create] done"
