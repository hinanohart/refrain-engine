# Roadmap — v0.1.0 (shipped) and v0.2.0 (planned)

## v0.1.0 — released 2026-05-17

All 12 build phases are complete. See `CHANGELOG.md` for the released
inventory and the documented deviations from the initial design.

| # | Phase | Status |
|---|---|---|
| 0 | verify-stack | done |
| 1 | scaffold | done |
| 2 | core-skeleton | done |
| 3 | core-refrain-dsl | done |
| 4 | core-egglog-bridge | done (using `egg`, not `egglog`) |
| 5a | adapter-ffi (PyO3) | done |
| 5b | intensity_plane (forward-mode autodiff) | done (pure-Python; JAX deferred) |
| 5c | cell complex layer | done (lightweight; TopoModelX deferred) |
| 6 | adapter-audio | done (Strudel JSON + OSC) |
| 7 | adapter-visual | done (deterministic PNG) |
| 8 | adapter-code | done (Python + Rust emission) |
| 9 | adapter-text | done (prose + bullets) |
| 10 | rhizome (Loro) | bridge done; Loro wiring gated behind `--features rhizome`, deferred |
| 11 | docs | done (mdBook + rustdoc, 0 warnings) |
| 12 | release artifact | done — local `v0.1.0` tag, `cargo package --list` succeeds for all four publishable crates, `maturin develop` builds the Python wheel |

## v0.2.0 — planned

The deferrals listed in `CHANGELOG.md` collect here:

| Item | Why deferred | Target |
|---|---|---|
| Full `egglog` Datalog rules | egglog 0.4 API was in flux during the v0.1 build window | v0.2 |
| JAX-backed reverse-mode autodiff + GPU support | wider compute graphs only justify the install footprint in v0.2 | v0.2 |
| TopoModelX × JAX zero-copy bridge (DLPack) | requires JAX in place first | v0.2 |
| `inventory::submit!` adapter auto-registration | not load-bearing for the four built-ins; lands when third-party adapters arrive | v0.2 |
| Live Loro CRDT collaboration (`refrain-rhizome` wired with `loro` crate) | Loro 1.0 beta can yank; wait for stable | v0.2 |
| Arrow IPC zero-copy on the PyO3 boundary | JSON suffices until adapters need to ship binary buffers | v0.2 |
| Standalone runnable examples in `examples/` | the workspace integration test covers the same ground for v0.1 | v0.2 |
| Published binaries on crates.io and PyPI | requires user-set API tokens (R11 boundary) | done outside the build pipeline |

## Verification PoCs (from concept doc 2026-05-17)

| # | Item | Resolution |
|---|---|---|
| 1 | Synthetic Differential Geometry OSS missing for `dy/dx` | resolved: dual-number forward-mode + Ehrhard-Regnier combinator (`python/intensity_plane`) |
| 2 | TidalCycles Rust FFI immature | resolved: Strudel JSON + OSC bundle output (`refrain-adapters/audio.rs`) |
| 3 | eg-walker × egglog bridge | resolved: two-layer HashMap (`refrain-rhizome`); Loro wiring deferred |
| 4 | Loro Beta API churn | mitigated: opt-in `rhizome` feature flag, yank-readiness in `docs/risks.md` |
| 5 | TopoModelX × JAX integration | deferred to v0.2 with a lightweight in-tree `CellComplex` in the meantime |

## Out of scope

- Full egglog Datalog rules (deferred to v0.2).
- Synthetic Differential Geometry kernels via Agda or Catlab.jl.
- WGPU compute-shader autodiff.
- crates.io + PyPI publication automation (release commands run by the
  maintainer with API tokens; see CHANGELOG `user-intervention` notes).
