# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-05-17

Initial release of the Differential Refrain Engine.

### Added

- `refrain-core`: AST (`Refrain`, `Pattern`, `Op` with `Note`, `Loop`,
  `Diff`, `Quotient`, `Sym`, `Call`) and a hand-rolled S-expression
  parser (~250 LOC, no external parser deps).
- `refrain-egraph`: `egg`-based equality saturation with two starter
  rewrites (`loop-1-identity`, `seq-singleton-identity`), `AstSize`
  extraction, configurable node/iteration limits.
- `refrain-ffi`: PyO3 `cdylib` exposing `parse_refrain`,
  `normalize_refrain`, `parse_and_normalize`, `version` to Python via
  `refrain_py._native`.
- `refrain-adapters`: schedule compiler (`Pattern → Vec<Hap>`) plus four
  built-in adapters:
  - `AudioAdapter` (Strudel JSON / OSC bundle).
  - `VisualAdapter` (deterministic headless PNG, pinned compression).
  - `CodeAdapter` (Python / Rust source emission, round-trips through
    the parser).
  - `TextAdapter` (prose / bullets, LCG-seeded so identical
    `(refrain, seed)` produces byte-identical output).
- `refrain-rhizome`: two-layer HashMap bridge between Loro causal ids
  and egglog e-class ids (`alloc_eclass`, `insert`, `merge_classes`,
  `eclass_of`, `causal_events_of`). Loro itself is gated behind
  `--features rhizome`; the v0.1 build ships an empty Loro module to
  insulate against Loro 1.0 beta yanks.
- `python/intensity_plane`: forward-mode autodiff (`Dual` numbers with
  `derivative` and `jacfwd`), the Ehrhard-Regnier differential
  combinator over Refrain Pattern JSON, and a lightweight `CellComplex`
  with Euler characteristic computation.
- `python/refrain_py`: high-level Python facade importing the PyO3
  native module.
- mdBook user guide under `docs/book/` covering intro, DSL grammar,
  architecture, adapters, API reference, roadmap, philosophy footnote,
  and risks tracker. Builds with zero warnings.
- CI workflow (rustc stable + beta, Python 3.10/3.11/3.12, cargo-deny).
- `cargo-deny` configuration allowing only MIT/Apache-2.0/BSD/ISC family
  licenses; AGPLv3 is denied.
- MIT license, Contributor Covenant CoC, CONTRIBUTING guide.
- `docs/roadmap.md`, `docs/philosophy.md`, `docs/risks.md`.

### Tests

- 68 Rust unit tests + 5 cross-crate integration tests + 500 property
  test cases (5 properties × 100 cases) + 1 doctest, all passing.
- 45 Python tests (`pytest`) covering autodiff, the Ehrhard-Regnier
  combinator, cell complex, and the native bridge.

### Documented deviations from the initial design

- **`egg` instead of `egglog`**: the underlying e-graph crate is used
  directly; full egglog Datalog rules are deferred to v0.2 because the
  egglog 0.4 API was in flux during the build window.
- **Hand-rolled parser instead of chumsky**: a small recursive-descent
  parser ships without an external parser dep.
- **Pure-Python autodiff instead of JAX**: forward-mode dual-number
  autodiff is sufficient for v0.1's compute graph sizes; JAX integration
  (reverse-mode, GPU support) lands in v0.2.
- **TopoModelX DLPack bridge deferred**: `CellComplex` ships a
  lightweight Python representation; the zero-copy TopoModelX bridge is
  scheduled for v0.2.

### Not yet shipped (planned for v0.2)

- Live Loro CRDT collaboration (the bridge is ready; the I/O wiring is
  not).
- JAX-backed reverse-mode autodiff and GPU acceleration.
- TopoModelX cell-complex operators (zero-copy via DLPack).
- Full egglog Datalog rules.
- `inventory::submit!`-based adapter auto-registration.
- Published binaries on crates.io and PyPI.

[0.1.0]: https://github.com/runza/refrain-engine/releases/tag/v0.1.0
