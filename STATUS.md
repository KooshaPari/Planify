# Planify STATUS — current state

Last updated: 2026-08-11 (R38 audit close + 6 governance fixes committed; R40 deep-fill).

## TL;DR

- **Repo role:** Web-based project management UI for the Phenotype platform.
- **Upstream:** `makeplane/plane@preview` v1.3.1, verbatim seed in `upstream/`.
- **Phenotype layer:** `site/` (Astro 6 landing) + `infra/` (docker-compose additions).
- **License:** Planify root = Apache 2.0; upstream Plane = AGPL-3.0 (inherited).
- **Branch:** `main` is canonical; all work branches off `main`.
- **Governance drift:** closed — root CLAUDE.md + STATUS.md + PLAN.md added in R38-R40.
- **Security posture:** SECRET_KEY uses `python3 -c "import secrets; ..."`; SECURITY.md placeholder GPG removed; production must override `PLANE_POSTGRES_PASSWORD` and `PLANE_SECRET_KEY`.

## Current state — repositories / components

| Component | Path | State | Owner | Notes |
|-----------|------|-------|-------|-------|
| Upstream Plane seed | `upstream/` | Verbatim snapshot, do not modify | upstream Plane team | Synced at v1.3.1; absorb customizations outside this dir |
| Phenotype landing | `site/` | Astro 6 + Bun + Tailwind 4 | Planify | `HeroScene.astro` has TODO for `/keyboard.glb` placeholder |
| Infra additions | `infra/docker-compose.plane.yml` | Postgres 16 + Dragonfly + Plane services | Planify | Canonical compose; mirrored via symlinks to 6 fleet repos |
| Governance docs | `AGENTS.md`, `CLAUDE.md`, `STATUS.md`, `PLAN.md` | Authoritative | Planify | R38 added CLAUDE.md + STATUS.md; R40 added PLAN.md |
| Provenance | `MERGES.md`, `UPSTREAM.md` | Authoritative | Planify | Documents upstream seeding + customizations |
| Security | `SECURITY.md` | Authoritative | Planify | Reporting via `security@phenotype.space` |
| Landing site docs | `site/src/pages/`, `site/src/components/` | Active | Planify | Astro 6 + Tailwind 4 |
| ADRs | `docs/adr/` | Authoritative | Planify | Architecture decision records |

## Open issues — R38 audit (2026-08-11)

### Closed (5 commits, R38)

- **f7c8f0b6** → DONE: missing root CLAUDE.md (commit `b6925976`)
- **79f2c6e0** → DONE: missing root STATUS.md (this file, R38)
- **2ff568af** → DONE: weak SECRET_KEY entropy (commit `86b2e3c5`)
- **766db6b4** → DONE: .gitignore duplicate OS/editor sections (commit `17f7821e`)
- **eaa30e6d** → DONE: SECURITY.md placeholder GPG fingerprint (commit `fb2c1252`)

### Pending (5 beads, R38 + R40)

- **fef9e4ff** — Pending: `infra/docker-compose.plane.yml` weak defaults (`${PLANE_POSTGRES_PASSWORD:-agileplus-dev}` / `${PLANE_SECRET_KEY:-dev-secret-key-change-in-prod}`). Defer until R41 — this file is mirrored via symlinks to 6 fleet repos (`pheno/`, `pheno/agileplus/`, `HexaKit/`, `HexaKit/agileplus/`, `PhenoDevOps/`, `PhenoDevOps/agileplus/`), so any change must be coordinated.
- **7a385d71** — Pending: `upstream/setup.sh` missing `set -euo pipefail`; `success=false` workaround. Defer — `upstream/` is verbatim Plane seed; customizations land in root scripts (e.g., `scripts/setup-local.sh`).
- **e225075f** — Pending: 30+ inherited upstream TODOs. Accept as known-inherited; document in `PATCHES.md` (R41 candidate).
- **5aed7115** — Pending: `target/` artifact in source tree. Add `target/` to `.gitignore` in R41 (currently only `upstream/` patterns are present).
- **e36282cf** — Pending: `site/src/components/HeroScene.astro` missing `/keyboard.glb` asset. Acknowledged in README Known Gaps. Either ship placeholder geometry or add a real `.glb` (R41).

### Open (R40 polish, this round)

- `docs/INDEX.md` does not exist yet — create in R40.
- `CHANGELOG.md` does not have R38-R40 entries — append in R40.
- `PLAN.md` does not exist yet — create in R40.
- `site/` README is missing — create `site/README.md` for landing-page-specific dev guide.
- `infra/README.md` is missing — create for compose-file-specific ops guide.

## Recent merges (chronological, 2026-08-11)

### R40 deep-fill (in progress)

- TBD: `docs/INDEX.md` — landing-page for the docs/ tree
- TBD: `PLAN.md` — milestones, dates, dependencies
- TBD: `CHANGELOG.md` — R38-R40 entries with bead refs
- TBD: `site/README.md` — landing-page dev guide
- TBD: `infra/README.md` — compose-file ops guide

### R38 (audit + 5 fixes)

- `b6925976` docs(CLAUDE): add root CLAUDE.md (mirrors AGENTS.md scope)
- `f9a86532` docs(STATUS): add root STATUS.md (initial 38-line state)
- `fb2c1252` fix(security): remove placeholder GPG fingerprint
- `17f7821e` chore(gitignore): dedupe OS/editor section
- `86b2e3c5` fix(setup): use cryptographically secure SECRET_KEY entropy

### Pre-R38 (existing baseline)

- `e6b8e235` fix(dual-harness): make fixture path resolution robust across repo root & worktrees
- ... (see `git log --oneline` for full pre-R38 history)

## Architecture layers

| Layer | Tech | Owner | Notes |
|-------|------|-------|-------|
| Frontend (Plane apps) | Next.js + MobX | upstream Plane team | web/space/admin in `upstream/apps/` |
| Backend (Plane API) | Python/Django | upstream Plane team | `upstream/apps/api` |
| Phenotype landing | Astro 6 + Bun + Tailwind 4 | Planify | `site/` |
| Database | Postgres 16 | infra | `infra/docker-compose.plane.yml` |
| Cache | Dragonfly (Redis-compatible) | infra | `infra/docker-compose.plane.yml` |
| Phenotype integrations | TBD | Planify | site/ + Phenotype workspace + backplane |

## Migration plan — to v1.0

When Planify reaches v1.0:

1. **Quarantine `upstream/`** — extract customizations into root, leave upstream as read-only reference.
2. **Hard-fork at tagged commit** — replace verbatim Plane with a tagged snapshot.
3. **Compose → Kubernetes** — migrate `docker-compose.plane.yml` to K8s manifests.
4. **Site consolidation** — merge `site/` into a Phenotype-wide landing hub.
5. **Domain registration** — `planify.space` (per site/data/config.json) registered + DNS configured.
6. **Public beta** — open the landing site to waitlist signups.

## Roadmap (Q3-Q4 2026)

| Milestone | Date | Status | Notes |
|-----------|------|--------|-------|
| R38 — root CLAUDE.md + STATUS.md + 5 audit fixes | 2026-08-11 | DONE | Closed 5 of 10 R38 audit beads |
| R40 — deep-fill governance | 2026-08-11 | IN PROGRESS | docs/INDEX, PLAN.md, CHANGELOG, site/README, infra/README |
| R41 — coordinate canonical-file changes | TBD | PENDING | docker-compose + .gitignore target/ + PATCHES.md for upstream TODOs |
| R42 — landing-site ship | TBD | PENDING | Add /keyboard.glb or remove TODO; site beta |
| R50 — v1.0 quarantine | TBD | PENDING | upstream/ quarantine, hard-fork tag, K8s manifests |

## Operational notes

- **Daily daemon:** auto-commit daemon may write wip commits to `chore/wbs-*` branches; do not cherry-pick these onto `main`.
- **Worktrees:** use `repos/planify-wtrees/<topic>/` for feature work; canonical repo stays on `main`.
- **Bead discipline:** every non-trivial change MUST append a bead via `./beads/bead-ctl.sh`; reference bead ID in PR description.
- **Subagent daemon:** occasionally degrades — pivot to direct exploration if `TaskOutput` times out.
- **Cockpit HTML SSOT:** `cockpit/bead-cockpit-20260809-191131-f5ca38f7.html` is the temporary portfolio SSOT; edit in place, do not regenerate.

## How to contribute

1. Branch from `main`: `feat/<topic>`, `fix/<topic>`, `chore/<topic>`, `docs/<topic>`, `refactor/<topic>`.
2. Append a bead: `./beads/bead-ctl.sh claim <target> "<text>"`.
3. Make changes; small, scoped commits with Conventional Commits messages.
4. Reference bead ID in commit footer: `Bead: <id>`.
5. Push branch; open PR; link bead in PR description.
6. After merge, run `./beads/bead-ctl.sh complete <target> "<commit-hash> summary>"`.

## References

- `AGENTS.md` — branching, commit, upstream-sync policy
- `CLAUDE.md` — root governance pointer
- `PLAN.md` — milestones + dates (R40+)
- `MERGES.md` — consolidation provenance
- `UPSTREAM.md` — upstream seeding notes
- `SECURITY.md` — security reporting channels
- `docs/INDEX.md` — docs/ tree landing (R40)
