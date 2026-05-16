# Release procedure — v0.1.0 onward

This file is the runbook the maintainer follows to take the local
`v0.1.0` tag to a published release on GitHub + crates.io + PyPI.
Each step depends on a secret the maintainer holds; none of these
steps can be automated by Claude (R11 boundary).

## Pre-flight

```bash
# Working tree clean?
cd ~/refrain-engine && git status

# Tests green?
cargo test --workspace
.venv/bin/pytest python/tests

# Docs build clean?
cargo doc --no-deps --workspace
(cd docs/book && mdbook build)

# Linters clean?
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
.venv/bin/ruff check python/

# License audit clean?
cargo deny check licenses
.venv/bin/pip-licenses --format=plain
```

## 1. Push the source and tag to GitHub

```bash
gh repo create refrain-engine --public --source=. \
  --description "Differential Refrain Engine — unified time-pattern DSL"
git push -u origin main
git push origin v0.1.0
```

If `gh auth status` reports unauthenticated, run `gh auth login` first.

## 2. Publish the Rust crates to crates.io

Crates must be published in topological order. `CARGO_REGISTRY_TOKEN`
must be exported in the shell.

```bash
export CARGO_REGISTRY_TOKEN=...   # do NOT commit this
for c in refrain-core refrain-egraph refrain-rhizome refrain-adapters; do
  cargo publish -p "$c" --allow-dirty
done
```

`refrain-ffi` is a cdylib bound to maturin; it is **not** published as
a standalone crate.

## 3. Publish the Python wheel to PyPI

```bash
.venv/bin/maturin publish --username __token__ --password "$PYPI_TOKEN"
```

`PYPI_TOKEN` must be set; use a project-scoped API token, not the
account-level one.

## 4. Create the GitHub release

```bash
gh release create v0.1.0 \
  --title "v0.1.0 — initial release" \
  --notes-file CHANGELOG.md \
  --verify-tag
```

## 5. Verify the release

- `pip install refrain-py==0.1.0` (in a fresh venv) imports cleanly.
- `cargo add refrain-core@0.1.0` resolves.
- The GitHub release page links the `v0.1.0` tag commit.

## Rollback

If `cargo publish` succeeds and you must withdraw, **yanking** (not
deleting) is the only supported path:

```bash
cargo yank --version 0.1.0 refrain-core
cargo yank --version 0.1.0 refrain-egraph
cargo yank --version 0.1.0 refrain-rhizome
cargo yank --version 0.1.0 refrain-adapters
```

PyPI does not support unpublish; bump to `0.1.1` with the fix instead.

## What this runbook deliberately does NOT do

- Hand any API token to Claude or to a CI runner the maintainer does
  not control (R11 boundary).
- Auto-merge any PR.
- Push to `main` from CI.
