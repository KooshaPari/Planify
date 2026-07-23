# Known Issues

## Dependency Install
- `pnpm install --frozen-lockfile` fails because the current package override configuration does not match the checked-in lockfile.
- This prevents a fresh checkout from running `task lint` until dependencies are installed by another path or the lockfile is reconciled.

## Out of Scope
- Lockfile regeneration was not included in this Taskfile-only PR.
