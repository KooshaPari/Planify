# pheno-drift-detector

**App-substrate drift detector (ADR-049, L74).**

`pheno-drift-detector` scans **PAUSED / CONDITIONAL / CAPSTONE** app repos
for **2+ non-trivial capabilities** that match the substrate pattern (per
[ADR-023 Rule 3](../../AGENTS.md)). When detected, it outputs
GitHub-issue-ready JSON or Markdown for the weekly heavy-runner cron →
issue auto-creation.

This is one of three companion substrate-scanners in the v8 weekly
heavy-runner cron bundle:

- `pheno-predict` — similar-code scanner (companion)
- `pheno-drift-detector` — **app-substrate drift detector** (this tool)
- `pheno-framework-lint` — tier-convention enforcer (companion)

## Install

```bash
chmod +x pheno_drift_detector.py
ln -s "$(pwd)/pheno_drift_detector.py" /usr/local/bin/pheno-drift-detector
./pheno_drift_detector.py --help
```

No external dependencies — stdlib only.

## Usage

### Scan the fleet for drift hits

```bash
pheno-drift-detector scan \
    --root .. \
    --format gh-issues \
    --out drift-hits.md
```

`--root` is the directory containing the app repos. The detector walks
each subdirectory, infers its ADR-023 bucket from the repo name
(PAUSED / CONDITIONAL / CAPSTONE), and applies the 4-criterion candidate
profile.

### Score a single candidate repo

```bash
pheno-drift-detector score --candidate ../Dino --format md
```

Useful for ad-hoc evaluation without bucket-name requirements — score any
directory that may contain an extractable capability.

### List known ADR-023 buckets

```bash
# Show canonical bucket definitions
pheno-drift-detector list-buckets

# Show canonical buckets + which repos under --root fall into each
pheno-drift-detector list-buckets --root ..
```

### Validate a single hit (HITL gate)

```bash
pheno-drift-detector validate --hit drift-hits/hit-0.json --yes
```

HITL gate: human must confirm before extraction PR is opened.

## Subcommands

| Subcommand | Purpose |
|---|---|
| `scan` | Walk `--root`, find ADR-023-bucket app repos, score each, output hits above threshold. |
| `score` | Compute drift score for a single repo path (no bucket-name requirement). |
| `list-buckets` | List known ADR-023 buckets (`paused`, `conditional`, `capstone`) with optional root annotation. |
| `validate` | Re-print a saved hit JSON for HITL confirmation. |

## Algorithm (3 passes per ADR-049 §3)

### Pass 1 — Discover app repos

Walk `--root`; for each subdirectory, check if its name matches an ADR-023
bucket (see `PAUSED_APPS`, `CONDITIONAL_APPS`, `CAPSTONE_APPS` in
`pheno_drift_detector.py`). If yes, schedule for scanning. The `score`
subcommand skips this pass entirely.

### Pass 2 — Find non-trivial capabilities

For each candidate app repo, group source files by top-level directory.
A "non-trivial capability" must have:

- ≥ 3 source files
- ≥ 5 KB total
- at least one file matching a Port trait pattern
  (`trait Foo {`, `interface Foo {`, `protocol Foo`, `impl X for Y`, etc.)

### Pass 3 — Score + suggest

Drift score = `1.0·n + 0.4·n_ports + 0.3·n_adapters + 0.3·n_tests`.
Threshold: **1.5** (per `DRIFT_THRESHOLD`). Hits above the threshold get:

- **Target substrate**: `pheno-*-lib` (Port only) / `phenotype-*-sdk` (Port + Adapter)
  / `phenotype-*-framework` (≥ 2 Ports + ≥ 2 Adapters) / federated-service.
- **Suggested action**: extract `cap[0].dir` (and related) into the suggested substrate.

## Output formats

- **`json`** — raw `DriftHit` objects, machine-readable (default for `scan` and `score`).
- **`md`** — human-readable summary table.
- **`gh-issues`** — Markdown formatted for `gh issue create --body-file -`. Each
  hit is a separate issue body, separated by `<!-- drift-hit: N -->` markers.

## Cron integration

The canonical cron recipe lives in `ops/heavy-runner-cron/INSTALL.md`
(scheduled for first run on **2026-06-23 09:00 PDT** on the heavy-runner).
This tool does **not** itself post to GitHub; it produces an issue-ready
Markdown render for the consumer (`phenotype-org-audits`) to file.

## Exit codes

- **0** — no drift hits (or `score` is below threshold)
- **1** — scan error
- **2** — drift hits found (CI can fail on this); or `score` is above threshold

## Schema

The drift detector implements the **substrate-extraction signal** from
[ADR-023 Rule 3](../../AGENTS.md). The scoring rubric is documented in
`pheno_drift_detector.py:91-99` (`W_CAPABILITY`, `W_PORT_MATCH`,
`W_ADAPTER_MATCH`, `W_TEST_MATCH`, `DRIFT_THRESHOLD`).

The 71-pillar framework is described in
`findings/71-pillar-2026-06-17-schema.md`. This tool is a **companion to
the framework**, not a pillar of it (the pillars are L1–L71).

## Retrospective hits (proof-of-value)

The first run of this tool against the v8 monorepo (2026-06-18) flagged
**`HwLedger`** as a CONDITIONAL drift candidate — the `pheno-capacity`
math lib extraction (later executed 2026-06-19, see
`findings/2026-06-19-L5-110-112-second-half-4-repo-absorption-audit.md`).
This confirmed the 3-pass algorithm catches real substrate candidates.

## Related tools

- [`pheno-predict`](../pheno-predict/) — companion L72 tool (similar-code scanner).
- [`pheno-framework-lint`](../pheno-framework-lint/) — companion L73 tool (substrate tier enforcer).

## License

MIT — see [`LICENSE`](LICENSE).