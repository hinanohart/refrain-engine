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

- 85 Rust unit + integration tests across all crates (Refrain DSL fixtures,
  parser robustness, e-graph normalization, audio/visual/code/text adapters,
  rhizome bridge merge invariants, cross-crate integration smoke tests) plus
  19 property-based test functions covering scheduler totality, JSON
  emission validity, OSC framing, PNG header preservation, code-emission
  round-trip, text-adapter seed determinism, parser non-panic on arbitrary
  text, comment transparency, and rhizome merge idempotence. Property
  functions exercise ~64–100 cases each (≈1500 case runs total). One
  doctest. All passing.
- A true SHA-256 golden test fixes the byte-deterministic visual adapter
  output for a canonical refrain at a fixed canvas size.
- 53 Python tests (`pytest`) including 8 `hypothesis` property tests
  covering the sum, product, and chain rules of the dual-number autodiff
  and structural properties of the Ehrhard-Regnier combinator.
- `cargo-fuzz` nightly is deferred to v0.2 (see `docs/roadmap.md`); v0.1
  ships proptest + hypothesis coverage instead.

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
- **`inventory::submit!` auto-registration deferred**: v0.1.0 adapters
  are constructed by hand; trait-based plug-in discovery via
  `inventory::submit!` is wired up in v0.2 when third-party adapters
  become a load-bearing concern.
- **JSON instead of Arrow IPC on the PyO3 boundary**: the
  `refrain_py._native` module ships JSON-stringified refrains in v0.1.0.
  Arrow IPC zero-copy lands when an adapter needs to round-trip binary
  buffers across the boundary.

### Not yet shipped (planned for v0.2)

- Live Loro CRDT collaboration (the bridge is ready; the I/O wiring is
  not).
- JAX-backed reverse-mode autodiff and GPU acceleration.
- TopoModelX cell-complex operators (zero-copy via DLPack).
- Full egglog Datalog rules.
- `inventory::submit!`-based adapter auto-registration.
- Published binaries on crates.io and PyPI.

[0.1.0]: https://github.com/hinanohart/refrain-engine/releases/tag/v0.1.0
