# Specifications

## Acceptance Criteria
- Provide root Taskfile tasks named `build`, `test`, `lint`, and `clean`.
- Detect the repository language from root manifests.
- Prefer the root JavaScript/TypeScript monorepo scripts when `package.json` and `pnpm-lock.yaml` are present.
- Keep fallback behavior for Python and Go repositories.

## Assumptions, Risks, Uncertainties
- Assumption: The root workspace toolchain is authoritative for this repo.
- Risk: Dependency installation is currently blocked by a lockfile configuration mismatch.
- Mitigation: Keep this change scoped to task orchestration and do not modify the lockfile in this PR.
