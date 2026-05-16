# Roadmap — v0.1.0

The release tracker for Differential Refrain Engine v0.1.0. Each row maps to a
build phase; once a row clears its acceptance gate it is checked off here.

## Build phases

| # | Phase | Status | Acceptance gate |
|---|---|---|---|
| 0 | verify-stack | done | rustc / cargo / python3 / git / gh available |
| 1 | scaffold | done | `cargo check --workspace` clean |
| 2 | core-skeleton | done | `cargo test` empty pass across all crates |
| 3 | core-refrain-dsl | done | 20 fixtures parse; structural extraction tests pass |
| 4 | core-egglog-bridge | done | property tests at 100 cases (idempotence, no-panic) |
| 5a | adapter-ffi (PyO3) | in progress | `maturin develop` builds; round-trip works |
| 5b | intensity_plane (JAX autodiff) | pending | `dyt` and `ehrhard_regnier_d` working with `pytest` ≥ 20 cases |
| 5c | TopoModelX bridge | pending | cell-complex serialize/deserialize via Arrow |
| 6 | adapter-audio | pending | Strudel JSON sink + OSC; 5 s fixture compare |
| 7 | adapter-visual | pending | wgpu/skia 1-frame PNG SHA-256 golden |
| 8 | adapter-code | pending | text-template golden test |
| 9 | adapter-text | pending | n-gram template golden test |
| 10 | rhizome (Loro) | pending | doc test (default skip), 2-layer HashMap bridge sketch |
| 11 | docs | pending | mdBook builds with 0 warnings |
| 12 | release artifact | pending | `cargo package` + `python -m build` dry-run pass, local `v0.1.0` tag |

## Verification PoCs (from concept doc 2026-05-17)

These five items were flagged as load-bearing technical uncertainties in the
initial concept review. Status snapshot below:

| # | Item | Resolution | Where |
|---|---|---|---|
| 1 | Synthetic Differential Geometry OSS missing for `dy/dx` | abandoned in favor of dual-number forward-mode + Ehrhard-Regnier differential combinators (no external SDG dep needed) | `python/intensity_plane/__init__.py` (Phase 5b); see [philosophy.md](./philosophy.md) |
| 2 | TidalCycles Rust FFI immature | swapped for Strudel JSON over stdin + OSC sink (rosc crate) | Phase 6 (`refrain-adapters::audio`) |
| 3 | eg-walker (Loro) × egglog (egg) bridge | two-layer HashMap mapping `causal_id ↔ eclass_id` | Phase 10 (`refrain-rhizome`); doc'd as opt-in until Loro 1.0 |
| 4 | Loro Beta API churn | pin `1.0.0-beta.5` in `Cargo.toml`; yank-readiness documented in [risks.md](./risks.md) | Phase 10 |
| 5 | TopoModelX × JAX integration | DLPack zero-copy bridge for cell-complex → JAX arrays; cell complex stored as a dict-of-Arrow batches | Phase 5c |

## Out of scope for v0.1.0

- Full egglog Datalog rules (deferred to v0.2 — `egg` covers the v0.1 rewrite set).
- Synthetic Differential Geometry kernels via Agda or Catlab.jl.
- WGPU compute-shader autodiff (CPU-only JAX is sufficient for v0.1).
- crates.io + PyPI publication automation (release commands run by maintainer
  with API tokens; see Phase 12 artifact).
