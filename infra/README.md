# Planify infra — compose file ops guide

This directory contains Phenotype-specific infrastructure additions on top of the upstream Plane stack.

## Files

| File | Purpose |
|------|---------|
| `docker-compose.plane.yml` | Canonical compose for the Plane stack — Postgres 16 + Dragonfly + Plane API/worker/beat + Plane web |

## Canonical-file constraint

`docker-compose.plane.yml` is the **canonical** compose file for the Plane stack and is **mirrored via symlinks** to 6 fleet repos:

```
pheno/docker-compose.plane.yml
pheno/agileplus/docker-compose.plane.yml
HexaKit/docker-compose.plane.yml
HexaKit/agileplus/docker-compose.plane.yml
PhenoDevOps/docker-compose.plane.yml
PhenoDevOps/agileplus/docker-compose.plane.yml
```

> **Edit ONLY this file** (`Planify2/infra/docker-compose.plane.yml`). The symlinks propagate changes automatically.

## Services

| Service | Image | Port | Purpose |
|---------|-------|------|---------|
| `plane-db` | postgres:16-alpine | 5432 | Postgres database for Plane |
| `dragonfly` | docker.dragonflydb.io/dragonflydb/dragonfly:latest | 6379 | Redis-compatible cache |
| `plane-api` | makeplane/plane-backend:stable | 8000 | Plane backend API |
| `plane-web` | makeplane/plane-frontend:stable | 3100→3000 | Plane frontend |
| `plane-worker` | makeplane/plane-backend:stable | — | Celery worker |
| `plane-beat` | makeplane/plane-backend:stable | — | Celery scheduler |

## Quick start

```bash
# Full Plane stack (all services as containers)
docker compose -f docker-compose.plane.yml up -d

# Dragonfly + DB only (Plane runs natively via process-compose)
docker compose -f docker-compose.plane.yml up dragonfly plane-db -d

# Tail logs
docker compose -f docker-compose.plane.yml logs -f

# Teardown
docker compose -f docker-compose.plane.yml down
```

## Environment variables

| Variable | Default | Purpose | Production |
|----------|---------|---------|------------|
| `POSTGRES_USER` | `agileplus` | DB user | override per deployment |
| `PLANE_POSTGRES_PASSWORD` | `agileplus-dev` | DB password | **MUST override** (currently weak default — see R38 audit bead fef9e4ff) |
| `PLANE_SECRET_KEY` | `dev-secret-key-change-in-prod` | Django SECRET_KEY | **MUST override** (currently weak default — R38 audit) |
| `PLANE_WEB_URL` | `http://localhost:3100` | Public web URL | set to production domain |
| `PLANE_CORS_ALLOWED_ORIGINS` | `http://localhost:3100,http://localhost:3000` | CORS origins | set to production domains |
| `PLANE_DEBUG` | `0` | Django debug mode | `0` in production |
| `NEXT_PUBLIC_API_BASE_URL` | `http://localhost:8000` | Frontend → API URL | set to production API URL |

## Known issues

- **Weak defaults** (`PLANE_POSTGRES_PASSWORD:-agileplus-dev`, `PLANE_SECRET_KEY:-dev-secret-key-change-in-prod`) — tracked as R38 audit bead `fef9e4ff`. Plan: remove `:-` fallback or fail-fast when env unset. Coordinated change required since the file is mirrored to 6 repos.
- **Symlink drift** — if any of the 6 symlinked copies diverges from canonical, run `git diff` against `Planify2/infra/docker-compose.plane.yml` to detect.

## Healthchecks

Each service has a healthcheck:

- `plane-db`: `pg_isready` on port 5432
- `dragonfly`: `redis-cli ping` on port 6379
- `plane-api`: `curl /api/health` on port 8000
- `plane-web`: `curl /` on port 3000

`plane-api` and `plane-web` wait for `plane-db` and `dragonfly` to be healthy before starting.

## Migration to Kubernetes

Per `PLAN.md` M3-R52, this compose file will be replaced with a Helm chart. Until then, this file is the canonical reference.

## Related docs

- [`../README.md`](../README.md) — repo overview
- [`../AGENTS.md`](../AGENTS.md) — branching, commit, upstream-sync policy
- [`../STATUS.md`](../STATUS.md) — current state + roadmap
- [`../PLAN.md`](../PLAN.md) — Q3-Q4 milestones (R52 K8s migration)
- [`../site/README.md`](../site/README.md) — landing-page dev guide
- [`../docs/adr/`](../docs/adr/) — architecture decision records
