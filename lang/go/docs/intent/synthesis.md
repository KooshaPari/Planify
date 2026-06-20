# Intent synthesis — phenotype-go-sdk

> Generated from prompt provenance in `prompts/`. Last updated: 2026-06-16.

## Themes (from prompts)

### Theme: Platform role Go consolidation

**Prompts:** genesis rollout (cursor)

**User language (paraphrase with citations):**

- "role platform, boundary go edge justified" — genesis rollout directive

## Confirmed goals

Goals explicitly stated by the user:

1. **Own `platform` domain role for Go modules** — devhex, devenv, MCP Go workspace
2. **Bootstrap genesis docs from HexaKit template** — charter, intent, SOTA, OKF, review

## Inferred goals

Agent interpretation — **requires user validation**:

| Inferred goal | Evidence prompts | Agent action taken | Validate? |
|---------------|------------------|--------------------|-----------|
| McpKit Go restore when modules available | README workspace notes | charter lists mcpkit deferred | pending |
| platformkit stays docs-only | ADR-011 | charter in-scope table | pending |

## Conflicts / tensions

| Tension | Prompts | Resolution |
|---------|---------|------------|
| Tier 1 default vs Go platform edge | LANGUAGE_PLACEMENT | SOTA technical.md Tier 3 table |
| Duplicate module paths | ADR-011 | Removed platformkit Go dupes |

## Rejected / deferred

- Rust rewrite of devenv without migration plan — deferred
- New standalone PlatformKit repo — rejected (absorbed)

## Recommended next actions (for agents)

1. Restore McpKit Go workspace when modules compile — aligns with [charter.md](../../../charter.md)
2. Run prompt scraper after significant sessions — update [intent.md](../../../intent.md)

## LLM grounding notes

When acting on this repo, agents should:

1. Read `charter.md` before adding Go packages
2. Prefer `docs/sota/technical.md` language placement over ad-hoc Rust/Python ports
3. Append new user prompts to `prompts/` before large pivots
