# Research

## Repository signals
- `package.json` exists at the repository root.
- `pnpm-lock.yaml` exists at the repository root.
- `bun.lock` also exists, but the workspace scripts are defined for `pnpm`.
- Nested Python manifests exist under `apps/api`, so the repository is polyglot, but the root toolchain is JavaScript/TypeScript.

## Existing root scripts
- `build` maps to `turbo run build`.
- `check:lint` maps to workspace lint checks.
- `clean` removes turbo, Next.js, dist, and node_modules artifacts.

## Taskfile direction
- Keep the four requested common tasks.
- Use repo detection to route to the correct language/runtime command.
- Prefer the root `pnpm` scripts when the repository is detected as JavaScript/TypeScript.
