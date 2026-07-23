## Summary

Restore the from-scratch Geist/Keycap dark Askama dashboard as the primary integration substrate, wire Plane.so settings + agent sessions with real backend calls, and absorb the Electrobun desktop shell, audit CLI cockpit, and theme polish from other frontend attempts.

## Context

AgilePlus accumulated 38+ frontend attempts across repos (Plane forks, React/Vite/Tailwind, Electrobun desktop, spec-kitty migrations, Askama dark dashboard). This PR picks the most solid from-scratch candidate — **PR #198's Askama+HTMX dashboard** (`4f3b19f5`, Mar 27 2026) — as the integration base and wires it with real functionality. The key insight: this dashboard had kanban, Plane.so settings, and agent sessions pages that were stubs; now they make real HTTP calls and read from saved config.

## Changes (3 commits on top of PR #198 base)

### 1. Wire Plane.so settings + agent sessions (`216de2f9`)

- **Plane.so settings page** reads from saved TOML (`~/.agileplus/dashboard.toml`) with env var fallback
- **Test Connection** button makes real HTTP call to Plane API via `reqwest`
- **Agent sessions page** reads from saved TOML config
- **Test Provider** validates OpenAI (`/v1/models`) or Anthropic (`/v1/messages`) API keys via real HTTP
- **Kanban** already wired via `seed_bridge` — shows real kitty-specs as features
- **Planify REST shim** absorbed from `frontend/planify-rest-shim` as startable subprocess with start/stop/status routes

### 2. Absorb Electrobun, audit cockpit, theme polish (`a33464a6`)

- **Electrobun desktop shell** — native WebView2 wrapping dashboard daemon on port 8770
- **Audit CLI cockpit** — `cockpit.rs` (rubric scores → NDJSON), `dashboard.rs` (SQLite DAG view with kanban summary, worklog, events, trace links)
- **Theme polish** — `skeleton.html` (4 variants), `empty-state.html`, `theme-toggle.html` (light/dark/system with localStorage), updated `base.html` nav

### 3. Fix compilation (`599e4454`)

- Commented out 3 private git plugin deps (optional, not used by dashboard)
- Fixed Askama 0.12 incompatible `{% include %}` syntax in `base.html`
- Simplified `skeleton.html` to hardcoded 3-line text variant
- `cargo check -p agileplus-dashboard` passes cleanly (0 errors, 0 warnings)

## Key Files

| File | What |
|---|---|
| `crates/agileplus-dashboard/src/routes.rs` | 39 async handlers — Plane settings, agent sessions, Planify shim management |
| `crates/agileplus-dashboard/src/templates.rs` | 36 Askama template structs |
| `templates/pages/dashboard.html` + `partials/kanban.html` | Kanban view of projects |
| `templates/pages/settings-plane.html` | Plane.so integration (api_url, api_key, workspace_slug, project_id) |
| `templates/pages/settings-agents.html` | Agent sessions (live polling, pool config, dispatch mode) |
| `templates/partials/skeleton.html` | Loading placeholder (text/card/image/block variants) |
| `templates/partials/theme-toggle.html` | Light/dark/system toggle |
| `templates/static/style.css` | Geist/Keycap dark theme (`#09090b`/`#18181b` bg, zinc-700/800/900/950 grayscale, JetBrains Mono) |
| `desktop-electrobun/` | Native desktop shell (Electrobun WebView2) |
| `tools/planify-shim/` | Planify REST shim (Node.js, startable subprocess) |

## Testing

```bash
# Verify dashboard compiles
cargo check -p agileplus-dashboard  # 0 errors, 0 warnings

# Run tests
cargo test -p agileplus-dashboard  # 16/16 pass

# Start the dashboard
cargo run -p agileplus-dashboard

# Open browser
open http://localhost:8770

# Test Plane.so connection (after setting TOML config)
curl -X POST http://localhost:8770/api/settings/plane/test

# Test agent provider (after setting TOML config)
curl -X POST http://localhost:8770/api/settings/agents/test
```

## Links

- Priority branch: `frontend/from-scratch-geist-dashboard` (38 other `frontend/*` branches preserved in Planify2)
- Original PR #198: `fix(dashboard): implement service controls and agent settings UI`
- AgilePlus dashboard crate: `crates/agileplus-dashboard/`
