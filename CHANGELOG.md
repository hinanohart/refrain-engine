# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Work in progress toward v0.1.0. See `docs/roadmap.md` for the phase table.

### Added

- Initial Cargo workspace scaffold with five crates (Phase 1).
- `refrain-core`: `Refrain` / `Pattern` / `Op` AST and a hand-rolled
  S-expression parser (~250 LOC, no external parser deps) (Phase 3).
- `refrain-egraph`: `egg`-based equality saturation, two starter rewrite rules
  (`loop-1-identity`, `seq-singleton-identity`), `AstSize` extraction (Phase 4).
- `refrain-rhizome`: scaffold with `--features rhizome` opt-in flag.
- `refrain-ffi`: JSON round-trip helpers; PyO3 boundary lands in Phase 5.
- `refrain-adapters`: `RefrainAdapter` trait; built-ins land in Phases 6–9.
- Python skeletons: `intensity_plane` (JAX, Phase 5b) and `refrain_py`
  (high-level API, Phase 5a).
- MIT license, CI workflow (rust + python + cargo-deny), `cargo-deny` config,
  `.gitignore`, `CHANGELOG`, `CONTRIBUTING`, `CODE_OF_CONDUCT`.
- `docs/roadmap.md`, `docs/philosophy.md`, `docs/risks.md`.

### Tests

20 DSL parser fixtures + 12 unit tests across the workspace + 5 property
tests at 100 cases each on `refrain-egraph::Egraph::normalize`. All passing.

### Deviations from initial design

- **`egg` instead of `egglog`**: the equality saturation library `egg` (the
  foundation of egglog) is used directly. Full egglog Datalog rules are
  scheduled for v0.2; the v0.1 rewrite set fits natively in `egg`. See
  `docs/risks.md`.
- **Hand-rolled parser instead of chumsky**: a ~250 LOC tokenizer plus
  recursive-descent parser is used; this removes a dependency without
  shipping functionality differences.

## [0.1.0] — pending Phase 12

Initial public release of Differential Refrain Engine.
