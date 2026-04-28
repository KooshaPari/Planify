# Session Overview

## Goal
Add a root `Taskfile.yml` that exposes common `build`, `test`, `lint`, and `clean` tasks and dispatches by detected repository language.

## Scope
- Inspect the repository stack from manifests.
- Update the root Taskfile for language-aware task execution.
- Prepare the branch, PR, and merge flow after validation.

## Result Target
- A single Taskfile at the repository root.
- Commands that work for JavaScript/TypeScript repositories and fall back cleanly for Python and Go layouts.
