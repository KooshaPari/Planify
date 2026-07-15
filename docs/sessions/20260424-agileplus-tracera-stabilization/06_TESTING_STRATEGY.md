# Testing Strategy

## Gates Run

```bash
cargo metadata --no-deps --format-version 1
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
process-compose --dry-run -e .agileplus/runtime/local-ports.env -f process-compose.yml
cargo test -p agileplus-cli specs --lib
cargo test -p agileplus-cli specs_audit --test cli_integration
cargo test -p agileplus-sqlite sync_mappings --lib
cargo test -p agileplus-sqlite --all-features
cargo test -p agileplus-plane outbound -- --nocapture
cargo test -p agileplus-plane
cargo test -p agileplus-dashboard --all-features
cargo test -p agileplus-cli --lib
cargo test -p agileplus-cli --test cli_integration
cargo test -p agileplus-cli frontend --lib
cargo test -p agileplus-cli --test cli_integration frontend
cargo test -p agileplus-cli --test cli_integration plan_requires_researched_state_unless_forced
cargo test -p agileplus-cli --lib plan
cargo test -p agileplus-cli validate --lib
cargo test -p agileplus-cli ship --lib
cargo test -p agileplus-cli retrospective --lib
cargo clippy -p agileplus-dashboard --all-targets --all-features -- -D warnings
cargo clippy -p agileplus-plane -p agileplus-cli -p agileplus-sqlite --all-targets --all-features -- -D warnings
cargo run -q -p agileplus-cli -- --db .agileplus/agileplus.db specs audit --json
cargo run -q -p agileplus-cli -- --db .agileplus/agileplus.db specs audit --strict
cargo run -q -p agileplus-cli -- frontend audit --strict --json
```

## Next Test Additions

- Spec migration test:
  after moving legacy `kitty-specs` entries into `.agileplus/specs`,
  `agileplus specs audit --strict` must pass against the local DB.

- Runtime topology test:
  `process-compose.yml`, `.agileplus/runtime/local-ports.env`, and Plane env
  files must point to existing directories and consistent ports.

- Frontend topology test:
  `agileplus frontend audit --strict` must pass. Every frontend-labeled
  directory must either have a manifest and runnable root command, or be labeled
  as scaffold/archive with `FRONTEND_STATUS.md`.

- Live Plane smoke:
  with a configured `PLANE_API_KEY`, `PLANE_WORKSPACE`, and `PLANE_PROJECT`,
  run one feature + work-package push and verify the local DB mapping rows.

- Tracera import contract test:
  AgilePlus project, feature, work package, evidence, audit log, and event
  payloads must validate against the receiving Tracera API model.

- Validation/shipping CLI fixture tests:
  full CLI scenarios should assert persisted audit rows for `validate --force`,
  `validate --skip-policies`, and `ship --skip-validate`, complementing the
  command-level unit tests now in place.

## Current Constraints

The SQLite package test now exercises the extracted facade modules and caught a
real `update_feature` persistence gap. Full workspace checks remain the final
gate, but local disk pressure can currently block large Cargo rebuilds.

## DB-002 Focused Gate

- Run workspace `cargo fmt --check` for the four newly enrolled production crates.
- Run package-scoped `cargo check` and `cargo test` for CLI, dashboard, SQLite,
  and MCP intent; unrelated pre-existing compile debt is reported rather than
  repaired in this slice.
- Copy `.agileplus/agileplus.db` to a temporary directory and run
  `scripts/seed_polyrepo_program.py <copy>` twice; never seed the live file.

### 2026-07-15 Result

- Changed-slice `rustfmt` with edition 2024 and `skip_children=true`: pass.
- Root `cargo fmt --all -- --check`: blocked by pre-existing conflict markers in
  `commands/list.rs` and `tests/cli_integration.rs`, plus a missing SQLite test module.
- Package `cargo check`: blocked during dependency resolution because
  `agileplus-git` requires yanked `gix 0.82.0`.
- Temporary-copy seeder passes: identical counts on both runs (`projects=7`,
  `features=27`, `work_packages=84`, `wp_dependencies=6`, `evidence=10`).

## Gix Dependency Gate

- `agileplus-git` used yanked `gix 0.82.0`, preventing workspace dependency
  resolution before any DB-002 code could compile.
- Upstream docs identify `gix 0.85.0` as the current release. The AgilePlus
  crate has no `gix::` callsites, so the forward update is dependency-only.
- Required proof: `cargo tree -i gix`, focused `agileplus-git` check/tests, then
  the four DB-002 package checks/tests. Pre-existing merge markers or missing
  test modules remain separately reported blockers.
