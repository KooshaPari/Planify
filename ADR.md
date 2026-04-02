# Architecture Decision Records (ADRs)

This directory contains architectural decisions for the Planify project.

## ADR Index

| Number | Title | Status | Date |
|--------|-------|--------|------|
| ADR-001 | AgilePlus Agent Module Architecture | Proposed | 2026-01-01 |
| ADR-002 | Worktrees API Design | Proposed | 2026-01-01 |

## ADR Template

```markdown
# ADR-XXX: Title

## Status
Accepted | Proposed | Deprecated | Superseded

## Context
[What is the issue that we're seeing that is motivating this decision or change?]

## Decision
[What is the change that we're proposing and/or doing?]

## Consequences
[What becomes easier or more difficult because of this change?]

### Positive
### Negative
### Neutral
```

## ADR-001: AgilePlus Agent Module Architecture

### Status
Proposed

### Context
Planify needs to integrate with the Phenotype AgilePlus system for work package tracking and sprint management.

### Decision
Implement an AgilePlus agent module that:
- Syncs work packages bidirectionally
- Maps Planify sprints to AgilePlus sprints
- Provides CLI for AgilePlus operations
- Maintains offline capability

### Consequences

#### Positive
- Unified work tracking across Phenotype
- CLI-driven workflow for agents

#### Negative
- Potential sync conflicts need resolution strategy
- Additional dependency on AgilePlus service

#### Neutral
- May require breaking changes if AgilePlus API evolves

## ADR-002: Worktrees API Design

### Status
Proposed

### Context
Phenotype agents need programmatic access to git worktree operations from Planify.

### Decision
Expose a Worktrees API that:
- Creates/removes worktrees
- Lists existing worktrees
- Syncs worktree state with Planify
- Provides branch management

### Consequences

#### Positive
- Centralized worktree management
- Integration with Planify project structure

#### Negative
- Git operation failures can be complex to handle
- Branch naming conflicts

#### Neutral
- Requires git 2.6+ for optimal functionality
