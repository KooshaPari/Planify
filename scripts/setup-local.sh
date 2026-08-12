#!/bin/bash
# Planify local setup wrapper.
#
# Wraps upstream/setup.sh (which we cannot modify per AGENTS.md) and
# adds `set -euo pipefail` + better secret generation. R41 audit
# bead 7a385d71 — original upstream/setup.sh is verbatim Plane seed;
# all customizations land here in scripts/.
#
# Usage:
#   bash scripts/setup-local.sh        # full setup
#   bash scripts/setup-local.sh --key  # only regenerate SECRET_KEY
#
# Differences from upstream/setup.sh:
#   - set -euo pipefail (was missing)
#   - python3 secrets.token_urlsafe(50) for SECRET_KEY (was tr -dc a-z0-9)
#   - Pre-flight check for python3 or openssl availability
#   - Drops a marker file at .setup-local.complete for tooling

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

MARKER="${REPO_ROOT}/.setup-local.complete"

echo "Planify local setup wrapper"
echo "  repo:    ${REPO_ROOT}"
echo "  marker:  ${MARKER}"
echo

# ---- Pre-flight ----
if ! command -v python3 >/dev/null 2>&1 && ! command -v openssl >/dev/null 2>&1; then
    echo "ERROR: neither python3 nor openssl is on PATH" >&2
    echo "  Install one to generate cryptographically secure secrets." >&2
    exit 1
fi

# ---- Run upstream/setup.sh (verbatim Plane — DO NOT MODIFY) ----
# We invoke it as a child process but with our environment. Any failure
# from upstream/setup.sh is now fatal because of `set -e`.
if [ -f "${REPO_ROOT}/upstream/setup.sh" ]; then
    echo "→ Running upstream/setup.sh (verbatim Plane seed)"
    bash "${REPO_ROOT}/upstream/setup.sh"
else
    echo "WARN: upstream/setup.sh not found; skipping" >&2
fi

# ---- Regenerate SECRET_KEY with python3 secrets ----
if [ -f "${REPO_ROOT}/apps/api/.env" ]; then
    echo
    echo "→ Regenerating SECRET_KEY with python3 secrets (R38 bead 2ff568af)"

    # Remove any existing SECRET_KEY line
    if [ -f "${REPO_ROOT}/apps/api/.env" ]; then
        # Use sed to drop the existing line (macOS-compatible)
        sed -i.bak '/^SECRET_KEY=/d' "${REPO_ROOT}/apps/api/.env" && rm -f "${REPO_ROOT}/apps/api/.env.bak"
    fi

    if command -v python3 >/dev/null 2>&1; then
        NEW_KEY=$(python3 -c "import secrets; print(secrets.token_urlsafe(50))")
    else
        NEW_KEY=$(openssl rand -base64 50 | tr -d '=+/' | head -c50)
    fi

    echo "SECRET_KEY=\"${NEW_KEY}\"" >> "${REPO_ROOT}/apps/api/.env"
    echo "  → wrote new SECRET_KEY to apps/api/.env"
else
    echo "WARN: apps/api/.env not found; SECRET_KEY not regenerated" >&2
fi

# ---- Marker ----
date -u +"%Y-%m-%dT%H:%M:%SZ" > "${MARKER}"
echo
echo "✓ Planify local setup complete"
echo "  marker: ${MARKER}"
