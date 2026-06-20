# Cost — SOTA (phenotype-go-sdk)

## Use case

Total cost of ownership: infrastructure, API usage, CI minutes, maintainer time, and **duplicate governance** across repos.

## Requirements

| Requirement | Weight |
|-------------|--------|
| Minimize duplicate doc/CI copies across fleet | should |
| CI cost proportional to change type | must |
| Avoid N× Kit repo pattern without justification | must (Phenotype org) |

## Comparison model

| Model | Governance copies | CI / maintenance | Verdict |
|-------|-------------------|------------------|---------|
| Separate PlatformKit + McpKit repos | N× governance | N× CI | rejected — ADR-011 |
| Rust rewrite without migration | 1× governance | high rewrite cost | rejected |
| **Single Go workspace + genesis docs** | 1× charter | targeted `go test` | **chosen** |

Fill with real numbers where available:

| Cost driver | Monthly estimate | Notes |
|-------------|------------------|-------|
| GitHub Actions minutes | low | doc-only PRs skip full build |
| Cloud / API | none | library repo |
| Maintainer hours (governance) | low | genesis bootstrap once |

## Alternatives considered

| Alternative | Cost profile | Verdict |
|-------------|--------------|---------|
| Duplicate `*Kit` archived repos | High — 9× governance per audit | rejected |
| SaaS doc portal only | subscription + lock-in | rejected |
| **Shared HexaKit genesis + optional SDK monorepos** | Lower — single scrape/review standard | **chosen** |

## Chosen strategy

Consolidating PlatformKit/McpKit Go into one workspace reduces duplicate governance and CI while preserving Tier 3 Go only where ecosystem lock-in applies (devhex/devenv).

## Evolution triggers

- SDK monorepo exceeds ~30 packages → evaluate feature-group publishing (not new Kit repos)
- CI minutes exceed budget → tighten smoke matrix
- Fleet doubles → automate OKF validate in CI

Update [alternatives.md](alternatives.md) when cost model changes.
