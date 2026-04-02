# Planify PRD - Product Requirements Document

**Version:** 1.0 | **Status:** Active | **Updated:** 2026-04-02

---

## 1. Concept & Vision

Planify is KooshaPari's fork of [makeplane/plane](https://github.com/makeplane/plane), extended with an AgilePlus agent module and Worktrees API for Phenotype ecosystem integration.

**Base Product:** Plane - Modern, open-source project management for all teams
**Phenotype Extensions:** Spec-driven development workflows and git worktree integration

The vision is to leverage Plane's mature project management capabilities while extending it with agent-native features for spec-driven development.

---

## 2. Design Language

### Aesthetic Direction
Modern, clean SaaS aesthetic - following Plane's established design system.

### Technology Stack
- **Frontend:** React 18+, TypeScript 5+, Tailwind CSS
- **Backend:** Django 4+, Python 3.8+
- **Database:** PostgreSQL v14
- **Cache:** Redis v6.2.7
- **Container:** Docker, Docker Compose

---

## 3. Core Features (from Plane)

### 3.1 Work Items
- Rich text editor with file uploads
- Sub-properties and custom fields
- Cross-referencing and relations
- Bulk operations

### 3.2 Cycles
- Sprint planning and tracking
- Burn-down charts
- Capacity planning
- Progress analytics

### 3.3 Modules
- Hierarchical organization
- Progress tracking
- Dependency management
- Milestone grouping

### 3.4 Views
- Customizable filters
- Multiple view types (board, list, calendar, gantt)
- Saved views with sharing
- Quick filters

### 3.5 Pages
- Rich text editor
- AI assistance
- Embedding capabilities
- Template support

### 3.6 Analytics
- Real-time dashboards
- Team velocity
- Cycle time analysis
- Custom reports

---

## 4. Phenotype Extensions (Planned)

### 4.1 AgilePlus Agent Module
**Purpose:** Enable spec-driven development workflows

**Features:**
- Feature entity management linked to Plane issues
- State machine transitions from specs
- Work package tracking
- Agent integration for automated development

**Status:** Planned

### 4.2 Worktrees API
**Purpose:** Git worktree management integrated with project workflows

**Features:**
- Create worktrees from Plane issues
- Track worktree state
- Sync with feature branches
- Cleanup management

**Status:** Planned

---

## 5. User Interactions

### 5.1 Web Interface
- Modern React SPA
- Responsive design
- Real-time updates
- Keyboard shortcuts

### 5.2 API
- RESTful API
- GraphQL support
- Webhooks
- SDK in multiple languages

### 5.3 CLI (Future)
- `planify-cli` for terminal workflows
- Worktree management
- Quick issue operations

---

## 6. Technical Approach

### Monorepo Structure
```
Planify/
├── apps/
│   ├── admin/        # Admin panel
│   ├── api/          # Django API server
│   ├── live/         # Real-time collaboration
│   ├── proxy/        # API proxy
│   ├── space/        # Space management
│   └── web/          # React frontend
├── packages/
│   ├── ui/           # Shared UI components
│   ├── types/         # TypeScript types
│   ├── hooks/         # React hooks
│   └── ...
└── docker-compose.yml
```

### Development Setup
```bash
# Clone
git clone https://github.com/KooshaPari/Planify.git
cd Planify

# Setup
chmod +x setup.sh
./setup.sh

# Start
docker compose -f docker-compose-local.yml up
pnpm dev
```

---

## 7. Roadmap

### Phase 1: Foundation (Current)
- [x] Fork and setup
- [x] CI/CD integration
- [ ] Phenotype CLAUDE.md/AGENTS.md
- [ ] Documentation for extensions

### Phase 2: AgilePlus Module
- [ ] Design agent integration
- [ ] Implement feature mapping
- [ ] Add webhook handlers
- [ ] Test with existing features

### Phase 3: Worktrees API
- [ ] Git worktree endpoints
- [ ] Worktree state tracking
- [ ] Integration with features
- [ ] Cleanup automation

---

## 8. Success Metrics

- All Plane features working
- AgilePlus integration tested
- Worktrees API functional
- Zero regression from upstream

---

**Last Updated:** 2026-04-02
**Product Owner:** Phenotype Core Team
**Forked From:** [makeplane/plane](https://github.com/makeplane/plane)
