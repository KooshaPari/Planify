# Planify — Agent Development Guide

## Repo Overview

Planify is the web-based project management UI for the Phenotype platform, derived from [Plane](https://github.com/makeplane/plane) (AGPL-3.0). It powers the AgilePlus dashboard and integrates with the broader Phenotype ecosystem. The repo is a **fork** of `makeplane/plane@preview` (v1.3.1) + Phenotype landing page + infra additions.

```
planify/
├── upstream/           # Verbatim seed from makeplane/plane@preview (DO NOT MODIFY)
│   ├── apps/           # admin, api, live, proxy, space, web
│   ├── packages/       # 15 shared TS packages
│   └── ...             # pnpm workspace root, turbo.json, etc.
├── site/               # planify.space landing page (Astro + Bun + Tailwind)
├── infra/              # Phenotype-specific docker-compose additions
├── MERGES.md           # Consolidation provenance
└── UPSTREAM.md         # Upstream seeding notes
```

## Agent Workflow

### Branching

- Branch from `main` for all work
- Use semantic branch names: `feat/short-description`, `fix/issue-description`, `chore/task-name`
- Never commit directly to `main`

### Commit Conventions

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

feat:  new feature
fix:   bug fix
chore: maintenance (deps, tooling, infra)
docs:  documentation only
refactor: code change with no behavior change
test:  adding or fixing tests
```

Keep commits small and scoped. Each commit should represent a single logical change.

### Upstream Sync

The `upstream/` directory is a verbatim Plane seed and **must not be modified directly**. Any customizations by Phenotype should land outside `upstream/`. To sync from upstream Plane:

```bash
cd upstream
git remote add upstream https://github.com/makeplane/plane.git
git fetch upstream preview
git merge upstream/preview
```

## CI Commands

| Command | Description |
|---------|-------------|
| `cd upstream && pnpm dev` | Start all Plane dev servers |
| `cd upstream && pnpm build` | Turbo build all packages and apps |
| `cd upstream && pnpm check` | Run format, lint, and type checks |
| `cd site && bun run dev` | Start landing page dev server (localhost:4321) |
| `cd site && bun run build` | Build landing site |
| `cd site && bun run check` | Run Astro type checking |

## Architecture Notes

- **Frontend layer**: Plane apps (web, space, admin) — Next.js + MobX state management
- **Backend layer**: Plane API — Python/Django (inside `upstream/apps/api`)
- **Phenotype layer**: Custom customizations go outside `upstream/` — currently `site/` and `infra/`
- **Landing page**: Astro 6 + Bun + Tailwind 4 — matches sibling Phenotype landings in `phenotype-landing`
- **Infra**: Docker Compose — Postgres 16 + Dragonfly + plane-api/worker/beat + plane-web
- **Upstream license**: AGPL-3.0 (inherited) — see `upstream/LICENSE.txt`. Planify root is Apache 2.0.

## Key Files to Know

| File | Purpose |
|------|---------|
| `README.md` | Project overview, layout, quick start |
| `MERGES.md` | Consolidation provenance — every source that contributed code |
| `UPSTREAM.md` | Upstream seeding notes and sync instructions |
| `upstream/turbo.json` | Turborepo pipeline config |
| `upstream/package.json` | pnpm workspace root |
| `site/astro.config.mjs` | Astro landing page config |
| `infra/docker-compose.plane.yml` | Canonical compose for Plane stack |
