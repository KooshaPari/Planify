# Alternatives index — phenotype-go-sdk

Master comparison index across SOTA dimensions. PRs that change strategic choices must update the relevant dimension file **and** this index.

## Dimension decisions

| Dimension | File | Decision (one line) | Confidence |
|-----------|------|---------------------|------------|
| Technical | [technical.md](technical.md) | Go workspace monorepo; Tier 3 justified | high |
| DX | [dx.md](dx.md) | `go work sync` + genesis docs | med |
| UX | [ux.md](ux.md) | N/A — library repo | n/a |
| AX | [ax.md](ax.md) | Genesis doc set + OKF + scraper | high |
| Security | [security.md](security.md) | Kilo Code Stand + CI secret scan | med |
| Ops | [ops.md](ops.md) | Targeted `go test` per package | med |
| Cost | [cost.md](cost.md) | One Go SDK vs N PlatformKit repos | high |

Executive summary: [../../../SOTA.md](../../../SOTA.md)

## Cross-cutting alternatives

| Decision | Alternatives rejected | Primary reason | ADR link |
|----------|----------------------|----------------|----------|
| Go platform monorepo | Rust rewrite, split repos | absorption cost + ecosystem lock-in | ADR-011 |
| devhex canonical module | platformkit Go dupes | duplicate import paths | ADR-011 |

## Fork repos

Not a fork — see [fork-rationale.md](fork-rationale.md).

## Research refresh log

| Date | Researcher | Dimensions updated | Notes |
|------|------------|-------------------|-------|
| 2026-06-16 | agent | all (bootstrap) | Genesis rollout from HexaKit template |

## Enforcement

[review.md](../../../review.md) Block tier: new Go module or dependency without updating this index or SOTA technical.md → fail PR.
