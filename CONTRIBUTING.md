# Contributing to Differential Refrain Engine

Thanks for your interest. This project is bus-factor-1, so contributions that
sharpen rather than expand the scope are most welcome.

## Quick start

```bash
git clone https://github.com/runza/refrain-engine
cd refrain-engine
cargo test --all
python -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
pytest
```

## Conventions

- **Rust edition 2021**, `cargo fmt` + `cargo clippy --all-targets -- -D warnings` clean.
- **Python ≥ 3.10**, `ruff` + `mypy --strict` clean on `python/`.
- **Commits**: imperative present tense ("Add X", not "Added X"). Don't reference
  internal rule numbers in commit messages.
- **PRs**: phase-per-branch (`phase/NN-name`). Draft PRs are fine; do not enable
  auto-merge — every merge to `main` is reviewed by hand.
- **License headers**: not required (MIT covers the repo).

## Filing issues

Reproducer over speculation. A failing test case is the best bug report.
