#!/usr/bin/env bash
# scripts/release.sh — single-script release runbook for refrain-engine v0.1.0
#
# This script wraps every R11-token-gated step the maintainer must run
# to take the local repo state to a published release on GitHub +
# crates.io + PyPI. It does NOT handle authentication interactively —
# all secrets MUST be present as environment variables before invocation.
#
# Prerequisites (read these BEFORE running):
#   1. gh CLI installed and authenticated: `gh auth login` (one-time)
#   2. CARGO_REGISTRY_TOKEN exported in the current shell
#        (https://crates.io/settings/tokens — scope: publish-new + publish-update)
#   3. PYPI_TOKEN exported in the current shell
#        (https://pypi.org/manage/account/token/ — project-scoped after first publish)
#   4. Working tree clean (`git status` shows nothing)
#   5. Local v0.1.0 tag exists and points to HEAD
#   6. cargo, .venv/bin/maturin, mdbook, cargo-deny installed
#
# Usage:
#   export CARGO_REGISTRY_TOKEN=...
#   export PYPI_TOKEN=...
#   ./scripts/release.sh [--dry-run] [--skip-preflight]
#
# Flags:
#   --dry-run         Run all pre-flight checks and `cargo publish --dry-run`
#                     for each crate, but do NOT push to GitHub, crates.io,
#                     or PyPI. Verifies the release is publishable.
#   --skip-preflight  Skip the local validation suite (fmt/clippy/test/deny).
#                     Only use if you have JUST run them by hand.
#
# Exit codes:
#   0  release succeeded (or dry-run succeeded)
#   1  pre-flight failed
#   2  missing required env var or tool
#   3  push or publish failed
#   4  post-publish verification failed
#
# Rollback (if a crates.io publish lands but the next step fails):
#   cargo yank --version 0.1.0 -p refrain-<crate>
#   PyPI does NOT support unpublish — bump to 0.1.1 with the fix instead.

set -euo pipefail

# --- config ---------------------------------------------------------------
VERSION="0.1.0"
TAG="v${VERSION}"
GITHUB_REPO="hinanohart/refrain-engine"
PUBLISH_ORDER=(refrain-core refrain-egraph refrain-rhizome refrain-adapters)

# refrain-ffi is a cdylib for maturin; NOT a standalone crates.io target.

# --- flags ----------------------------------------------------------------
DRY_RUN=0
SKIP_PREFLIGHT=0
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=1 ;;
        --skip-preflight) SKIP_PREFLIGHT=1 ;;
        -h|--help) sed -n '2,30p' "$0"; exit 0 ;;
        *) echo "unknown flag: $arg (see --help)"; exit 2 ;;
    esac
done

# --- helpers --------------------------------------------------------------
log() { printf '\033[1;34m[release]\033[0m %s\n' "$*"; }
ok()  { printf '\033[1;32m[ ok  ]\033[0m %s\n'  "$*"; }
err() { printf '\033[1;31m[ err ]\033[0m %s\n'  "$*" >&2; }

require_tool() {
    command -v "$1" >/dev/null 2>&1 || { err "missing tool: $1"; exit 2; }
}

require_env() {
    if [[ "$DRY_RUN" -eq 1 ]]; then
        log "dry-run: skipping env var check for $1"
        return 0
    fi
    if [[ -z "${!1:-}" ]]; then err "missing env var: $1"; exit 2; fi
    # never echo the value itself
    local _v="${!1}"
    log "$1 present (${#_v} chars)"
}

# --- 0. environment validation -------------------------------------------
log "=== 0. environment validation ==="
require_tool git
require_tool cargo
require_tool gh
require_tool mdbook
require_tool cargo-deny
[[ -x .venv/bin/maturin ]] || { err "missing .venv/bin/maturin (run: python -m venv .venv && .venv/bin/pip install maturin)"; exit 2; }
[[ -x .venv/bin/pytest  ]] || { err "missing .venv/bin/pytest";  exit 2; }
require_env CARGO_REGISTRY_TOKEN
require_env PYPI_TOKEN
gh auth status >/dev/null 2>&1 || { err "gh not authenticated (run: gh auth login)"; exit 2; }

# --- 1. pre-flight validation --------------------------------------------
if [[ "$SKIP_PREFLIGHT" -eq 0 ]]; then
    log "=== 1. pre-flight validation ==="

    log "git working tree clean?"
    if [[ -n "$(git status --porcelain)" ]]; then err "working tree dirty"; exit 1; fi
    ok "clean"

    log "local tag $TAG points to HEAD?"
    if ! git rev-parse "$TAG^{commit}" >/dev/null 2>&1; then err "tag $TAG missing"; exit 1; fi
    if [[ "$(git rev-parse "$TAG^{commit}")" != "$(git rev-parse HEAD)" ]]; then
        err "$TAG does not point to HEAD"; exit 1;
    fi
    ok "$TAG → HEAD"

    log "cargo fmt --all -- --check"
    cargo fmt --all -- --check
    ok "fmt"

    log "cargo clippy -D warnings"
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    ok "clippy"

    log "cargo test --workspace"
    cargo test --workspace --all-features
    ok "rust tests"

    log "cargo doc --no-deps"
    cargo doc --no-deps --workspace --all-features
    ok "rustdoc"

    log "cargo deny check"
    cargo deny check
    ok "deny"

    log "ruff + mypy + pytest"
    .venv/bin/ruff check python/
    .venv/bin/mypy  python/ --ignore-missing-imports
    .venv/bin/pytest python/tests -q
    ok "python lints + tests"

    log "mdbook build"
    (cd docs/book && mdbook build)
    ok "mdbook"
else
    log "=== 1. pre-flight SKIPPED (--skip-preflight) ==="
fi

# --- 2. dry-run sanity for crates.io --------------------------------------
# Only the leaf crate (refrain-core, no intra-workspace deps) can be
# dry-run before the first real publish — downstream dry-runs fail
# because refrain-core@0.1.0 does not yet exist on the index. For the
# other crates we verify the package contents via `cargo package --list`
# (does not require dep resolution).
log "=== 2. dry-run sanity ==="
log "refrain-core (cargo publish --dry-run)"
cargo publish -p refrain-core --dry-run --allow-dirty
ok "refrain-core publishable"
for crate in refrain-egraph refrain-rhizome refrain-adapters; do
    log "$crate (cargo package --list)"
    cargo package --list -p "$crate" --allow-dirty >/dev/null
    ok "$crate package contents OK"
done

if [[ "$DRY_RUN" -eq 1 ]]; then
    log "=== DRY RUN COMPLETE — nothing was pushed or published ==="
    exit 0
fi

# --- 3. push source + tag to GitHub --------------------------------------
log "=== 3. push source + tag to GitHub ==="
if ! git remote get-url origin >/dev/null 2>&1; then
    log "creating GitHub repo: $GITHUB_REPO"
    gh repo create "$GITHUB_REPO" --public --source=. --remote=origin \
        --description "Differential Refrain Engine — unified time-pattern DSL"
fi
git push -u origin main
git push origin "$TAG"
ok "pushed main + $TAG"

# --- 4. publish Rust crates (topological order) --------------------------
log "=== 4. publish Rust crates ==="
for crate in "${PUBLISH_ORDER[@]}"; do
    log "publish $crate"
    cargo publish -p "$crate"
    ok "$crate published"
    # crates.io needs a moment before dependents can resolve
    log "sleeping 12s for crates.io index propagation"
    sleep 12
done

# --- 5. publish Python wheel ---------------------------------------------
log "=== 5. publish refrain-py to PyPI ==="
.venv/bin/maturin publish --username __token__ --password "$PYPI_TOKEN"
ok "refrain-py 0.1.0 published"

# --- 6. create GitHub release --------------------------------------------
log "=== 6. create GitHub release ==="
gh release create "$TAG" \
    --title "v${VERSION} — initial release" \
    --notes-file CHANGELOG.md \
    --verify-tag
ok "GitHub release $TAG created"

# --- 7. post-publish verification ----------------------------------------
log "=== 7. post-publish verification ==="
log "cargo search refrain-core (should show ${VERSION})"
cargo search refrain-core | head -1
log "pip-installing refrain-py in fresh venv"
TMPVENV="$(mktemp -d)/verify"
python -m venv "$TMPVENV"
"$TMPVENV/bin/pip" install --quiet "refrain-py==${VERSION}"
"$TMPVENV/bin/python" -c "import refrain_py; print('refrain_py.version =', refrain_py.version())"
rm -rf "$TMPVENV"
ok "post-publish verification passed"

# --- done -----------------------------------------------------------------
log "=== RELEASE COMPLETE ==="
log "GitHub:    https://github.com/${GITHUB_REPO}/releases/tag/${TAG}"
log "crates.io: https://crates.io/crates/refrain-core"
log "PyPI:      https://pypi.org/project/refrain-py/${VERSION}/"
