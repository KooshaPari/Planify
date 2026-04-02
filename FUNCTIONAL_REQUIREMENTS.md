# Planify Functional Requirements

**Version:** 1.0 | **Status:** Planned | **Updated:** 2026-04-02
**Branch:** `main` (canonical)
**Format:** Specification-driven requirements with test traces

---

## Overview

Planify is a fork of [makeplane/plane](https://github.com/makeplane/plane) with Phenotype-specific extensions:
- AgilePlus agent module for spec-driven development
- Worktrees API for Phenotype ecosystem integration

**Original Project:** Plane - Open-source project management (Jira alternative)
**Phenotype Extensions:** Planned additions for ecosystem integration

---

## Architecture

```
/
├── apps/
│   ├── admin/           # Admin panel (TypeScript)
│   ├── api/            # API server (Django/Python)
│   ├── live/           # Live collaboration
│   ├── proxy/          # Proxy service
│   ├── space/          # Space management
│   └── web/            # React frontend (TypeScript)
├── packages/           # Shared packages
└── README.md
```

### Technology Stack
- **Frontend:** React, TypeScript, Tailwind CSS
- **Backend:** Django, Python 3.8+
- **Database:** PostgreSQL v14
- **Cache:** Redis v6.2.7
- **Container:** Docker, Docker Compose

---

## Master FR Registry

### FR-PLANIFY-001: Project Management Core

**Requirement:** System SHALL provide core project management features: work items, cycles, modules, and views.
**Traces To:** E1.1
**Code Location:** `apps/web/`, `apps/api/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** Plane test suite

---

### FR-PLANIFY-002: AgilePlus Agent Module

**Requirement:** System SHALL integrate an AgilePlus agent module for spec-driven development workflows.
**Traces To:** E2.1
**Code Location:** `packages/agileplus/` (planned)
**Repository:** Planify
**Status:** Planned
**Test Traces:** TBD

---

### FR-PLANIFY-003: Worktrees API Integration

**Requirement:** System SHALL expose Worktrees API endpoints for git worktree management integrated with project workflows.
**Traces To:** E3.1
**Code Location:** `apps/api/worktrees/` (planned)
**Repository:** Planify
**Status:** Planned
**Test Traces:** TBD

---

### FR-PLANIFY-004: Issue and Work Item Management

**Requirement:** System SHALL allow creation and management of work items with rich text, file uploads, sub-properties, and cross-referencing.
**Traces To:** E1.1
**Code Location:** `apps/space/`, `apps/api/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** Plane test suite

---

### FR-PLANIFY-005: Cycle and Sprint Tracking

**Requirement:** System SHALL track progress using cycles with burn-down charts and other analytical tools.
**Traces To:** E1.2
**Code Location:** `apps/space/cycles/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** Plane test suite

---

### FR-PLANIFY-006: Module Organization

**Requirement:** System SHALL support organizing complex projects into manageable modules with hierarchical structure.
**Traces To:** E1.3
**Code Location:** `apps/space/modules/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** Plane test suite

---

### FR-PLANIFY-007: Custom Views and Filtering

**Requirement:** System SHALL provide customizable views with filtering, sorting, and saved filter combinations.
**Traces To:** E1.4
**Code Location:** `apps/space/views/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** Plane test suite

---

### FR-PLANIFY-008: Pages and Documentation

**Requirement:** System SHALL provide rich text pages with AI capabilities for documentation and knowledge management.
**Traces To:** E1.5
**Code Location:** `apps/space/pages/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** Plane test suite

---

### FR-PLANIFY-009: Analytics and Insights

**Requirement:** System SHALL provide real-time analytics across all project data with visualization and trend analysis.
**Traces To:** E1.6
**Code Location:** `apps/space/analytics/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** Plane test suite

---

### FR-PLANIFY-010: REST API

**Requirement:** System SHALL expose REST API endpoints for all core operations with proper authentication and rate limiting.
**Traces To:** E4.1
**Code Location:** `apps/api/`
**Repository:** Planify
**Status:** Active (from Plane)
**Test Traces:** API integration tests

---

## Cross-Repo Dependencies

| From | To | Dependency Type | Status |
|------|----|-----------------|--------|
| FR-PLANIFY-* | AgilePlus | Agent module integration | Planned |
| FR-PLANIFY-* | thegent | Agent orchestration | Planned |

---

## Validation Rules

### Traceability
Every FR MUST:
1. Have a unique ID in the format `FR-PLANIFY-NNN`
2. Contain a SHALL or MUST statement
3. Reference an epic
4. Have a code location
5. List test traces or mark as Planned

---

**Last Updated:** 2026-04-02
**Maintained By:** Phenotype Core Team
