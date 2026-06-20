# Phenotype Rust SDK — Origin

The crate source code under `packages/` was migrated from the
`KooshaPari/PhenoLang` repository (archived 2026-06-20).

## Origin

| Crate | Origin | Description |
|-------|--------|-------------|
| `phenotype-core` | SDK workspace | Re-export hub and foundational types |
| `phenotype-async-traits` | PhenoLang | Async iterator, Future helpers, AsyncDrop |
| `phenotype-cache-adapter` | PhenoLang | Multi-backend cache (moka, lru, dashmap) |
| `phenotype-cost-core` | PhenoLang | Cost analysis, budgeting, tracking |
| `phenotype-crypto` | PhenoLang | Hashing (SHA-2, BLAKE3), AES-GCM, Ed25519, HMAC |
| `phenotype-git-core` | PhenoLang | Git porcelain operations |
| `phenotype-http-client-core` | PhenoLang | Connection pooling, retry, HTTP patterns |
| `phenotype-iter` | PhenoLang | Extended iterators, adapters, parallel utils |
| `phenotype-macros` | PhenoLang | Procedural macros (error derive, builder) |
| `phenotype-process` | PhenoLang | Process management, signals, supervision |
| `phenotype-rate-limit` | PhenoLang | Token bucket, sliding window rate limiting |
| `phenotype-retry` | PhenoLang | Configurable retry with backoff policies |
| `phenotype-string` | PhenoLang | String ops: normalization, sanitization, compression |
| `phenotype-test-infra` | PhenoLang | Test fixtures, BDD helpers, assertions |
| `phenotype-time` | PhenoLang | Duration, timestamp, time constants |
| `phenotype-validation` | PhenoLang | Input validation, constraint checking |

## Preservation Notes

- All crates retain their original `Cargo.toml` metadata (version, edition, authors).
- Crates using `version.workspace = true` inherit from the root workspace config.
- The workspace was set to edition 2021 for broad compatibility; upgrade to 2024
  is tracked separately.
