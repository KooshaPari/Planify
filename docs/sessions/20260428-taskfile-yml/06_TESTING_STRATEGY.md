# Testing Strategy

## Checks
- Run `task --list` to validate Taskfile syntax and task discovery.
- Run `task lint` to verify the lint task dispatches through the detected JavaScript/TypeScript path.

## Results
- `task --list` passed and showed `build`, `test`, `lint`, and `clean`.
- `task lint` reached `pnpm check:lint` and then failed because Turbo is unavailable without installed dependencies.
- `pnpm install --frozen-lockfile` failed due to the current lockfile configuration mismatch.
