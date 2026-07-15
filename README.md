# chatta-archive-2026-07-14

Archive of `repos/chatta/` whose origin returned 404 (deleted on GitHub).

## ⚠️ Status: PARTIAL — local repo has 1 missing object

The local clone at `repos/chatta/` is corrupted: it references object `fc0d1a7f79744c170d8ef3a0db23093f537ed7fd` as the parent of commit `b02a42c5b08296eb35642a5bc6e128548b160e13`, but that object itself is not present in the local `.git/objects/` store. This blocks `git push --mirror`.

**178 reachable commits are intact**; 73 unreachable commits in the local repo include the broken chain.

## Contents

- `chatta-working-tree.tar.gz` — full working tree at 2026-07-14 (140 MB → compressed)
- `chatta-git-dir.tar.gz` — full `.git/` directory (34 MB → compressed) including all refs and pack files

## Recovery options (for future)

1. If the original GitHub repo had a fork on another account, clone from there and patch.
2. Use `git replace --graft` to substitute the broken commit's parent with a known-good commit.
3. Reconstruct from `chatta-git-dir.tar.gz` with `tar -xzf chatta-git-dir.tar.gz -C /tmp/recover-chatta`, then `cd /tmp/recover-chatta/chatta/.git && git fsck --full` to see what's salvageable.

Created 2026-07-14 by KooshaPari repo-audit script.
