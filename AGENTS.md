# AGENTS.md - Planify

## Project Overview

- **Name**: Planify
- **Description**: Fork of makeplane/plane with AgilePlus agent module and Worktrees API
- **Language**: TypeScript (frontend), Python (backend)
- **Location**: Phenotype repos shelf

## Phenotype Extensions

This fork includes:
- AgilePlus agent module (planned)
- Worktrees API for Phenotype ecosystem integration (planned)

## Architecture

```
/
├── apps/
│   ├── admin/           # Admin panel (TypeScript)
│   ├── api/             # Django API server (Python)
│   ├── live/            # Live collaboration
│   ├── proxy/           # Proxy service
│   ├── space/           # Space management
│   └── web/             # React frontend (TypeScript)
├── packages/            # Shared packages
│   ├── ui/              # UI components
│   ├── types/           # TypeScript types
│   ├── hooks/           # React hooks
│   └── ...
└── docker-compose.yml
```

## Development Commands

```bash
# Frontend
npm install
pnpm dev

# Backend (inside apiserver/)
pip install -r requirements.txt
python manage.py runserver

# Full stack
docker compose -f docker-compose-local.yml up
```

## Agent Rules

### Project-Specific Rules

1. **Monorepo Structure**
   - Backend in `apps/api/` (Django)
   - Frontend in `apps/web/` (React)
   - Shared packages in `packages/`

2. **Fork Discipline**
   - Track upstream changes from makeplane/plane
   - Keep Phenotype extensions isolated
   - Test with upstream before merging

3. **Configuration**
   - Environment variables in `.env.example`
   - Docker Compose for local development
   - PostgreSQL v14 required

### Phenotype Org Standard Rules

1. **UTF-8 encoding** in all text files
2. **Worktree discipline**: canonical repo stays on `main`
3. **CI completeness**: fix all CI failures before merging
4. **Never commit** agent directories (`.claude/`, `.codex/`, `.cursor/`)

## Quality Standards

### Frontend
```bash
# Linting
npm run lint

# Formatting
npm run format

# Type checking
npm run check
```

### Backend
```bash
# Django checks
python manage.py check

# Migrations
python manage.py makemigrations
python manage.py migrate

# Tests
python manage.py test
```

## Phenotype Extension Guidelines

### AgilePlus Module
When implementing the AgilePlus agent module:

1. Create `packages/agileplus/` for shared code
2. Add Django models in `apps/api/agileplus/`
3. Create React components in `apps/web/components/agileplus/`
4. Add REST endpoints in `apps/api/agileplus/urls.py`
5. Write integration tests

### Worktrees API
When implementing the Worktrees API:

1. Add endpoints in `apps/api/worktrees/`
2. Create git operations service
3. Add React UI in `apps/web/worktrees/`
4. Document API in OpenAPI spec
5. Test with real git repositories

## Git Workflow

1. Create feature branch: `git checkout -b feat/phenotype-extension`
2. Make changes, add tests
3. Run linting and type checking
4. Create PR with description of Phenotype additions
5. Ensure upstream compatibility

## Key Files

- `docker-compose-local.yml` - Local development setup
- `apps/api/` - Django backend
- `apps/web/` - React frontend
- `packages/` - Shared packages
- `.env.example` - Environment template
