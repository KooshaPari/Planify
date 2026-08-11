# Planify STATUS — current state

Last updated: 2026-08-11 (R38 audit + 5 fixes committed).

## Current state

- **Upstream:** `makeplane/plane@preview` v1.3.1 seeded in `upstream/` (verbatim, do not modify)
- **Landing page:** `site/` (Astro 6 + Bun + Tailwind 4) — scaffolded, `HeroScene.astro` has TODO for `/keyboard.glb` placeholder
- **Infra:** `infra/docker-compose.plane.yml` (Postgres 16 + Dragonfly + plane-api/worker/beat + plane-web)
- **License:** Planify root = Apache 2.0; upstream Plane = AGPL-3.0
- **Branch:** `main` (default; all work branches off main)

## Open issues (R38 audit, 2026-08-11)

- **f7c8f0b6** — DONE: missing root CLAUDE.md (commit b6925976)
- **79f2c6e0** — DONE: missing root STATUS.md (this file)
- **2ff568af** — DONE: weak SECRET_KEY entropy (commit 86b2e3c5)
- **766db6b4** — DONE: .gitignore duplicate OS/editor sections (commit 17f7821e)
- **eaa30e6d** — DONE: SECURITY.md placeholder GPG fingerprint (commit fb2c1252)
- **fef9e4ff** — Pending: docker-compose.plane.yml weak defaults (`PLANE_POSTGRES_PASSWORD:-agileplus-dev` / `PLANE_SECRET_KEY:-dev-secret-key-change-in-prod`); remove `:-` fallback or fail-fast on unset
- **7a385d71** — Pending: upstream/setup.sh missing `set -euo pipefail`; success=false workaround should be removed
- **e225075f** — Pending: 30+ inherited upstream TODOs (apps/api, apps/live, apps/space, packages/editor); consider UPSTREAM_TODOS.md or document in PATCHES.md
- **5aed7115** — Pending: target/ artifact in source tree (no Cargo.toml in repo); add `target/` to .gitignore
- **e36282cf** — Pending: site/src/components/HeroScene.astro missing `/keyboard.glb` asset (acknowledged in README Known Gaps)

## Recent merges (R38)

- `b6925976` docs(CLAUDE): add root CLAUDE.md
- `fb2c1252` fix(security): remove placeholder GPG fingerprint
- `17f7821e` chore(gitignore): dedupe OS/editor section
- `86b2e3c5` fix(setup): use cryptographically secure SECRET_KEY entropy

## Migration plan

When Planify reaches v1.0:
- Remove or quarantine `upstream/` and absorb customizations into root
- Replace verbatim Plane with hard-fork at a tagged commit
- Migrate compose to Kubernetes manifests
