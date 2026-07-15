# tracera-pr-worktree-20260703-0014-archive-2026-07-14

Working-tree snapshot of `repos/Tracera-pr-worktree-20260703-0014/` whose git pointer was broken.

## Why archived

The worktree's `.git` file points to `/Users/kooshapari/CodeProjects/Phenotype/repos/Tracera/.git/worktrees/Tracera-pr-worktree-20260703-0014` but that path does not exist — the parent repo `repos/Tracera/` no longer has this worktree registered. The 207 MB on disk contains 1119 files; we exclude the `target/` Rust build output (207 MB) but keep everything else.

## Contents

- `tracera-pr-worktree-working-tree.tar.gz` — full working tree minus `target/` (Rust build cache)

## File summary

- Source dirs: `.deploy/`, `.github/`, `crates/`, `deploy/`, `docs/`, `frontend/`, `public/`, `src/`, `workspace/`
- Top-level files: `Cargo.lock` (32 KB), `Cargo.toml`, `SECURITY.md`, `vercel.json`, `wrangler.toml`, `.gitignore`

## Note on git history

The git history refs for this worktree were stored inside `Tracera/.git/worktrees/Tracera-pr-worktree-20260703-0014/`, which is gone. The .git file shows the intended pointer but the target dir does not exist. To recover history, look at the live worktrees of `repos/Tracera-recovery-20260713/` (which absorbed most of the Tracera work post-2026-07-13).

Created 2026-07-14 by KooshaPari repo-audit script.
