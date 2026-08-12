# Planify PLAN — milestones, dependencies, dates

Last updated: 2026-08-11 (R40 deep-fill).

## Q3-Q4 2026 roadmap

### M0 — Audit + governance foundation (R36-R40) ✅ DONE / IN PROGRESS

- [x] **R36** — Initial audit of 7 repos including Planify2 (drift: missing CLAUDE.md, missing STATUS.md, weak SECRET_KEY entropy, .gitignore dupes, SECURITY.md placeholder GPG, setup.sh `set -e` missing, target/ artifact, 30+ upstream TODOs, HeroScene.astro TODO)
- [x] **R38** — 5 audit fixes committed: CLAUDE.md + STATUS.md + setup.sh + .gitignore + SECURITY.md
- [ ] **R40** — Deep-fill: docs/INDEX.md, PLAN.md (this file), CHANGELOG entries, site/README.md, infra/README.md
- [ ] **R41** — Coordinate canonical-file changes (docker-compose, PATCHES.md for upstream TODOs)

### M1 — Site ship (R42-R44) 🔄 PENDING

- [ ] **R42** — Add `/keyboard.glb` asset OR remove `HeroScene.astro` TODO with canonical placeholder
- [ ] **R43** — `planify.space` domain registration + DNS configuration
- [ ] **R44** — Landing site public beta + waitlist signups

### M2 — Customizations extraction (R45-R48) 🔄 PENDING

- [ ] **R45** — Inventory customizations vs `upstream/` Plane seed
- [ ] **R46** — Move Phenotype-specific Plane customizations to root (site patches, infra additions)
- [ ] **R47** — Document customizations in `CUSTOMIZATIONS.md` (where each patch goes after R47)
- [ ] **R48** — Sync `upstream/preview` and resolve any conflicts with customizations

### M3 — v1.0 quarantine (R50-R54) 🔄 PENDING

- [ ] **R50** — Tag hard-fork point in `upstream/` (v1.3.1 + all customizations absorbed)
- [ ] **R51** — Move `upstream/` to `upstream-archive/` (read-only)
- [ ] **R52** — Replace compose with K8s manifests (Helm chart)
- [ ] **R53** — Public v1.0 release
- [ ] **R54** — Deprecation notice for verbatim Plane seed

## Per-repo dependencies

| Planify2 component | Depends on | Blocked by |
|--------------------|-----------|-----------|
| `infra/docker-compose.plane.yml` | 6 symlinks to fleet repos | M3-R52 K8s manifests |
| `site/` Astro 6 landing | `/keyboard.glb` asset (M1-R42) | — |
| `upstream/setup.sh` SECRET_KEY | python3 or openssl in PATH | — |
| AGENTS.md / CLAUDE.md | upstream Plane sync policy | M2-R47 CUSTOMIZATIONS.md |
| Production secrets | `PLANE_POSTGRES_PASSWORD` / `PLANE_SECRET_KEY` env | M1-R43 domain registration |

## Risks + mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| `docker-compose.plane.yml` weak defaults reach prod | MEDIUM | HIGH (RCE if exposed) | Document in PLAN M1; fail-fast in compose when env unset |
| 30+ upstream TODOs accumulate | HIGH | LOW | PATCHES.md tracking + custom-upstream workflow |
| `upstream/` drift from upstream Plane | MEDIUM | MEDIUM | Quarterly sync via `git fetch upstream preview` |
| Site beta exposes in-dev features | LOW | LOW | Waitlist gate + feature flags |
| Hard-fork at R50 breaks compat | LOW | HIGH | Tag + dual-workspace period before quarantine |

## Acceptance criteria — v1.0

- [ ] Root CLAUDE.md, AGENTS.md, STATUS.md, PLAN.md, CHANGELOG.md all current
- [ ] Zero R38-R40 audit beads open
- [ ] `upstream/` quarantined to `upstream-archive/` (read-only)
- [ ] K8s manifests replace `docker-compose.plane.yml`
- [ ] Site beta public, `/keyboard.glb` shipped (or TODO removed)
- [ ] SECURITY.md has real (non-placeholder) GPG fingerprint
- [ ] All 6 fleet symlinks updated to K8s manifests
- [ ] Customizations documented in `CUSTOMIZATIONS.md`
- [ ] Domain `planify.space` resolves + TLS cert valid
- [ ] v1.0.0 release tag created

## References

- `STATUS.md` — current state + recent merges + roadmap
- `AGENTS.md` — branching, commit, upstream-sync policy
- `CLAUDE.md` — root governance pointer
- `docs/INDEX.md` — docs/ tree landing (R40)
- `MERGES.md` — consolidation provenance
- `UPSTREAM.md` — upstream seeding notes
