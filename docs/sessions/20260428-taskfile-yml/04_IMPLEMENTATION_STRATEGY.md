# Implementation Strategy

## Approach
- Use Taskfile root variables to detect the repository language from common manifest files.
- Route JavaScript/TypeScript repositories to package-manager-native scripts, preferring `pnpm` when `pnpm-lock.yaml` is present.
- Keep Python and Go fallbacks in the same four tasks so the Taskfile remains useful if the repository layout changes.

## Rationale
- The root `package.json` already defines `build`, `check:lint`, and `clean`; calling those scripts avoids duplicating monorepo internals.
- `pnpm-lock.yaml` is the strongest root package-manager signal despite a `bun.lock` also being present.
