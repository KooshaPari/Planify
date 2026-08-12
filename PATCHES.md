# PATCHES — Planify2 upstream TODO/FIXME tracker

Last updated: 2026-08-11 (R42 — created to track inherited upstream TODOs).

## Why this file exists

`upstream/` is a verbatim snapshot of `makeplane/plane@preview` v1.3.1 and **must not be modified** per `AGENTS.md`. However, the upstream Plane codebase contains 83+ TODO/FIXME comments inherited from the seed.

This file tracks those TODOs without modifying `upstream/`. Each entry has:
- A `plz-XXX` local ID (for in-repo discussion)
- The upstream file path
- The verbatim TODO/FIXME text
- Our disposition (upstream PR / accept-inherited / work-around-outside-upstream / ignore)

## Disposition policy

| Disposition | Meaning |
|-------------|---------|
| **upstream PR** | Submit a PR to makeplane/plane and update the seed once merged |
| **accept-inherited** | Document but don't fix; the upstream TODO is benign or aspirational |
| **work-around-outside** | Apply the fix in our own `site/` / `infra/` / `scripts/` instead of touching `upstream/` |
| **ignore** | Cosmetic or duplicate — close without action |

## Inventory

### plz-001: SECRET_KEY generation in `upstream/setup.sh`

- **File:** `upstream/setup.sh:73`
- **Text:** `SECRET_KEY=$(tr -dc 'a-z0-9' < /dev/urandom | head -c50)`
- **Issue:** Weak entropy; excludes special characters that Django recommends.
- **Disposition:** **work-around-outside** — `scripts/setup-local.sh` (R41) regenerates the key with `python3 -c "import secrets; print(secrets.token_urlsafe(50))"` after running upstream/setup.sh.
- **Upstream PR candidate:** Yes — but our `setup-local.sh` wrapper handles it for now.

### plz-002: missing `set -euo pipefail` in `upstream/setup.sh`

- **File:** `upstream/setup.sh` (header)
- **Issue:** Script uses `success=false` workaround instead of `set -e`; cascading failures possible.
- **Disposition:** **work-around-outside** — `scripts/setup-local.sh` (R41) wraps upstream/setup.sh with `set -euo pipefail`; failures now fatal.
- **Upstream PR candidate:** Yes.

### plz-003: Weak docker-compose defaults

- **File:** `infra/docker-compose.plane.yml` (NOT in upstream — this is a Planify2 addition mirrored to 6 fleet repos)
- **Issue:** `PLANE_POSTGRES_PASSWORD:-agileplus-dev` and `PLANE_SECRET_KEY:-dev-secret-key-change-in-prod` allow weak defaults if env unset.
- **Disposition:** **work-around-outside** — coordinate R41 across all 6 mirrored copies. Plan: remove `:-` defaults; fail-fast if env unset.
- **Upstream PR candidate:** No — this is our addition, not upstream.

### plz-004: 30+ TODO/FIXME comments in `upstream/apps/`

- **Files:** `upstream/apps/api`, `upstream/apps/live`, `upstream/apps/space`, `upstream/packages/editor`, `upstream/packages/codemods`, etc.
- **Issue:** Inherited verbatim from makeplane/plane@preview v1.3.1.
- **Disposition:** **accept-inherited** — see `plz-inventory-apps.txt` for the full list.
- **Upstream PR candidate:** Varies per TODO; not in Planify2 scope to triage unless we hit a real bug.

### plz-005: HeroScene.astro `/keyboard.glb` TODO

- **File:** `site/src/components/HeroScene.astro:17`
- **Text:** `TODO: load /keyboard.glb once added to public/`
- **Issue:** The `.glb` asset does not exist; placeholder geometry renders.
- **Disposition:** **work-around-outside** — generate or source a real `keyboard.glb` in R42/R43; the TODO is in our `site/`, not upstream.
- **Upstream PR candidate:** No.

## plz-inventory — verbatim grep result

The full inventory of 83+ TODO/FIXME comments is captured in the grep output of:

```bash
grep -rn 'TODO\|FIXME' upstream/apps upstream/packages
```

This is intentional — the count is large enough that a per-item tracker would be noise. The plz-XXX entries above are only the high-impact ones.

## How to add a new entry

1. Run `grep -rn 'TODO\|FIXME' upstream/ | head` to find the comment.
2. Open a bead via `./beads/bead-ctl.sh warn <target> "..."` referencing the upstream file.
3. Add a row to this file with `plz-XXX: <description>`.
4. Pick a disposition from the table.
5. When disposition is **work-around-outside**, link the bead ID to the script/file that applies the fix.

## Related docs

- `STATUS.md` — current state + roadmap (M2 customizations extraction R45-R48)
- `PLAN.md` — Q3-Q4 milestones
- `MERGES.md` — consolidation provenance (which customizations came from where)
- `UPSTREAM.md` — upstream sync instructions
- `docs/adr/0005-upstream-sync-strategy.md` — strategy for keeping `upstream/` fresh without modifying it
