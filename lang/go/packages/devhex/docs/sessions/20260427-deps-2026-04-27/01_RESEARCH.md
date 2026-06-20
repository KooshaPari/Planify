# Research

- Live Dependabot alerts via `gh api repos/KooshaPari/DevHex/dependabot/alerts?state=open&per_page=3`.
- Open alerts found:
  - GHSA-4vq8-7jfc-9cvp / CVE-2025-54410
  - GHSA-pxq6-2prw-chj9 / CVE-2026-33997
  - GHSA-x744-4wpc-v9h2 / CVE-2026-34040
- The old `github.com/docker/docker` module tops out at `v28.5.2` in this repo's current import path, which is still below the advisory fix line.
- The published client module `github.com/moby/moby/client` exposes the same constructors and opts used by DevHex.
