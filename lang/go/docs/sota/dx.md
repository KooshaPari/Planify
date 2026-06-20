# DX — SOTA (phenotype-go-sdk)

## Use case

How developers clone, build, test, and publish Go platform modules in this workspace.

## Requirements

| Requirement | Weight |
|-------------|--------|
| Documented bootstrap path (<15 min to first green test) | must |
| Local dev matches CI (`go test`) | should |
| Genesis docs discoverable from README | must |

## Workflow (chosen)

1. Clone repo and read [charter.md](../../../charter.md) for package boundaries
2. Run `go work sync` from repo root
3. Build/test active package: `go test ./packages/devhex/...`
4. For governance changes, update OKF manifest and SOTA as needed

```bash
git clone https://github.com/KooshaPari/phenotype-go-sdk.git
cd phenotype-go-sdk
go work sync
go test ./packages/devhex/...
```

## Alternatives considered

| Alternative | Pros | Cons | Verdict |
|-------------|------|------|---------|
| Per-package clone (old PlatformKit) | isolation | duplicate governance | rejected |
| README-only onboarding | zero maintenance | agents ignore scope | rejected |
| **Go workspace + genesis docs** | single charter; linked review/SOTA | manual workspace sync | **chosen** |

## Pain points mitigated

| Pain | Mitigation |
|------|------------|
| Duplicate module paths | ADR-011 convergence; charter Block tier |
| Agent scope creep | review.md + charter out-of-scope table |
| Lost session prompts | `docs/intent/prompts/` scraper |

## Evolution triggers

- McpKit modules restored → update workflow with second `go.work` path
- `hexakit genesis init` ships → link from README
