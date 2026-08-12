# Changelog

All notable changes to Planify will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `upstream/` — seeded with verbatim snapshot of `makeplane/plane@preview` v1.3.1 (AGPL-3.0)
  - Plane apps: admin, api, live, proxy, space, web
  - Plane packages (15): codemods, constants, decorators, editor, hooks, i18n, logger, propel, services, shared-state, tailwind-config, types, typescript-config, ui, utils
  - Root pnpm workspace, Turbo config, Docker Compose manifests
- `site/` — Astro 6 + Bun + Tailwind 4 landing page scaffolded (planify.space)
  - Hero section with Three.js 3D canvas (placeholder keyboard geometry)
  - Feature grid, CTA section, footer
  - Vercel deployment config
- `infra/` — Docker Compose for Plane stack mirrored from AgilePlus
  - Postgres 16 + Dragonfly + plane-api/worker/beat + plane-web
- Root README, UPSTREAM.md, MERGES.md — project overview, seeding notes, consolidation provenance
- Root LICENSE (Apache 2.0), AGENTS.md, CONTRIBUTING.md, CHANGELOG.md, SECURITY.md — foundational repo docs
- `.gitignore` — patterns for build artifacts, secrets, and OS files

### Known Gaps

- `.glb` keyboard model for hero 3D scene — missing from assets; placeholder geometry renders
- `pnpm install` and `bun install` deferred due to disk pressure (42 GiB free at seed time)
- No custom Phenotype features beyond scaffolding — upstream Plane code is unmodified

## [0.1.1] - 2026-08-11 (R38 + R40 audit close)

### Security

- **86b2e3c5** fix(setup): use cryptographically secure SECRET_KEY entropy
  - Replaced `tr -dc 'a-z0-9' < /dev/urandom | head -c50` with `python3 -c "import secrets; print(secrets.token_urlsafe(50))"` (fallback: `openssl rand -base64 50`).
  - Django SECRET_KEY now benefits from URL-safe base64 + 64 chars of entropy.
  - Closes R38 audit bead `2ff568af`.
- **fb2c1252** fix(security): remove placeholder GPG fingerprint
  - Replaced all-zeros GPG fingerprint in `SECURITY.md` with "TBD" + note that real key will land before external disclosure.
  - Closes R38 audit bead `eaa30e6d`.

### Documentation

- **b6925976** docs(CLAUDE): add root CLAUDE.md mirroring AGENTS.md scope
  - Closes R38 audit bead `f7c8f0b6` (highest-priority governance drift).
- **f9a86532** docs(STATUS): add root STATUS.md (initial 38-line state)
  - Closes R38 audit bead `79f2c6e0`.
- **3efca3bb** docs(STATUS,PLAN): R40 deep-fill — comprehensive state + Q3-Q4 roadmap
  - STATUS.md: 38 → 167 lines (TL;DR, component table, closed/pending issues, architecture layers, roadmap, ops notes).
  - PLAN.md: new file (M0 audit, M1 site ship, M2 customizations, M3 v1.0 quarantine).
  - Closes R40 deep-fill bead.
- `infra/README.md` — new file, compose-file ops guide with services table, env vars, healthchecks, K8s migration plan.
- `site/README.md` — expanded from 35 → 130+ lines with deploy workflow, env vars, common tasks, troubleshooting.

### Housekeeping

- **17f7821e** chore(gitignore): dedupe OS/editor section
  - Closes R38 audit bead `766db6b4`.

### Pending (R41+)

- `fef9e4ff` — docker-compose weak defaults (canonical file; defer for R41)
- `7a385d71` — upstream/setup.sh `set -e` (verbatim Plane; defer)
- `e225075f` — 30+ upstream TODOs (PATCHES.md in R41)
- `5aed7115` — `target/` artifact (add to .gitignore in R41)
- `e36282cf` — `/keyboard.glb` asset (M1-R42)

## [0.1.0] - Unreleased

### Added

- Initial repository seeding and scaffolding
- Plane.so fork structure with upstream/ subtree
- Astro landing page
- Docker infra
- Foundational documentation and tooling
