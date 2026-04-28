# Planify

A Plane.so fork — modern project management for all teams. Self-hosted project, program, and portfolio management with cycles, issue tracking, views, and GitHub/Jira integrations.

## Stack
| Layer | Technology |
|-------|------------|
| Backend | Python (Django, Celery) |
| Frontend | React (TypeScript) |
| DB | PostgreSQL |
| Cache | Redis |
| File Storage | S3-compatible (MinIO, AWS S3) |
| Container | Docker, Docker Compose |
| CI | GitHub Actions |

## Key Commands
```bash
# Development (Docker)
docker compose up
docker compose up --build

# Frontend
cd ui
npm install
npm run dev

# Backend
pip install -r requirements.txt
python ../../plane/processes/mgt/commands/run_migrations.py

# Lint (frontend)
cd ui && npm run lint

# Run tests
docker compose -f docker-compose.yml run --rm -e CYPRESS_BASE_URL=http://web:3000 ui npx cypress run
```

## Key Files
- `ui/` — React TypeScript frontend
- `apps/` — Backend Django apps
- `apiserver/` — API server code
- `processes/` — Core process management
- `scripts/` — Operational scripts
- `docker-compose.yml` — Full stack
- `package.json` — Frontend dependencies

## Reference
Global Phenotype rules: see `~/.claude/CLAUDE.md` or `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md`
