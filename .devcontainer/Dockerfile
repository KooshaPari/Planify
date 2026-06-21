# syntax=docker/dockerfile:1.7
# Devcontainer image for pheno-predict.
# Base: rust:1.83 (Debian bookworm). Preinstalls the audit + test tooling the
# fleet uses across all v22-T4 work packages. Per-crate components and
# `cargo-mutants` are added by post-create.sh so they always run as the
# non-root devcontainer user.
FROM rust:1.83-bookworm

# Toolchain hygiene + git LFS support.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        git \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Pin the Rust toolchain so the image is reproducible.
RUN rustup default 1.83.0 \
    && rustup --version \
    && cargo --version \
    && rustc --version

# Preinstall the audit + test tooling the fleet expects.
# `--locked` pins transitive deps to the published Cargo.lock; `--root` keeps
# the bins on PATH for the devcontainer user.
ENV CARGO_INSTALL_ROOT=/usr/local/cargo
RUN cargo install --locked --root "${CARGO_INSTALL_ROOT}" cargo-deny \
    && cargo install --locked --root "${CARGO_INSTALL_ROOT}" cargo-nextest \
    && cargo install --list \
    && cargo deny --version \
    && cargo nextest --version

# Drop the cargo registry from the final layer to keep the image lean.
RUN rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git"

# Workspace mount point — matches the devcontainer.json `context`.
WORKDIR /workspace

# Run as the non-root user that the rust:1.83 image already provides so
# post-create.sh can rustup-component-add without sudo.
USER rust
WORKDIR /home/rust/workspace
