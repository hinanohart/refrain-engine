# Differential Refrain Engine

A unified DSL for time-pattern composition across audio, visual, code-rewrite, and text media,
built on Rust **egg** equality saturation with planned integration of **Loro CRDT** (eg-walker)
collaborative editing, **JAX** forward-mode automatic differentiation, and **cell complex**
topology layers (via TopoModelX / GUDHI). The frontend is a small S-expression refrain language.

## Status

**Work in progress, pre-release.** The crate scaffold, AST, parser, and core e-graph
normalization are implemented. JAX bindings, Loro integration, and the four media adapters
are scheduled for v0.1.0 — see `docs/roadmap.md` and `CHANGELOG.md` for details.

| Component | Status |
|---|---|
| `refrain-core` (DSL parser + AST) | implemented |
| `refrain-egraph` (egg-based normalization) | implemented |
| `refrain-ffi` (PyO3 + Arrow IPC) | scaffolded (Phase 5) |
| `refrain-rhizome` (Loro CRDT, opt-in) | scaffolded (Phase 10) |
| `refrain-adapters` (audio/visual/code/text) | trait only (Phases 6–9) |
| `intensity_plane` (JAX + topology) | scaffolded (Phase 5) |

## What it does

```lisp
(refrain melody-a
  (territorialize (loop 4 (note C4 q)))
  (deterritorialize (dy/dx intensity time))
  (reterritorialize (quotient ~rotation ~transpose)))
```

A `refrain` is a named, structured time-pattern. The engine:

1. **Parses** the S-expression DSL (`refrain-core`, hand-rolled tokenizer + recursive-descent
   parser, ~250 LOC, zero external parser deps).
2. **Normalizes** via `egg` rewrite rules with an `AstSize` extraction cost model
   (`refrain-egraph`). Full `egglog`-style Datalog rules are deferred to v0.2.
3. **Differentiates** via dual-number forward-mode autodiff (no JAX dep in v0.1.0) plus an
   Ehrhard-Regnier differential combinator (`python/intensity_plane`). JAX-backed
   reverse-mode lands in v0.2 — see `docs/roadmap.md`.
4. **Emits** to a pluggable adapter (audio via Strudel/OSC, visual via deterministic PNG,
   code via Python/Rust source templates, text via seeded prose/bullets). Adapters
   implement a small `RefrainAdapter` trait and are constructed by hand in v0.1.0;
   `inventory::submit!`-based auto-registration is scheduled for v0.2.
5. **Optionally syncs** patterns across collaborators via Loro CRDT eg-walker
   (`refrain-rhizome`, opt-in `--features rhizome`) — *in progress*.

## Crate layout

| Crate | LOC (target) | Role |
|---|---|---|
| `refrain-core` | ~3500 | DSL parser, AST, public API |
| `refrain-egraph` | ~2000 | egg-based rewrite engine, extraction |
| `refrain-rhizome` | ~1500 | Loro CRDT bridge (feature = `rhizome`) |
| `refrain-ffi` | ~800 | PyO3 boundary (Arrow IPC zero-copy) |
| `refrain-adapters` | ~1200 | adapter trait + 4 built-ins |
| `python/intensity_plane` | ~2500 | JAX autodiff + cell-complex topology |
| `python/refrain_py` | ~500 | high-level Python API |

## Install (post-v0.1.0)

```bash
cargo add refrain-core           # Rust core
pip install refrain-py           # Python bindings (maturin-built)
```

## Quickstart

The five-test integration suite in
[`crates/refrain-adapters/tests/integration.rs`](./crates/refrain-adapters/tests/integration.rs)
walks a canonical refrain through every adapter end-to-end and is the most
useful executable example in v0.1.0. Standalone runnable example files in
`examples/` are scheduled for v0.2.

## Philosophy footnote

The names *refrain*, *territorialization*, *deterritorialization*, and *intensive multiplicity*
are inspired by Gilles Deleuze and Félix Guattari's *Mille Plateaux* (1980) and
Deleuze's *Différence et Répétition* (1968). The implementation does **not** claim
mathematical isomorphism with their philosophical concepts — these are evocative names for
concrete technical operations (looped pattern primitive, structural decomposition,
recomposition with constraints, autodiff Jacobian directions). See
[`docs/philosophy.md`](./docs/philosophy.md) for a sober mapping.

## License

Licensed under either of

- MIT License ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements

Built on: [egg](https://github.com/egraphs-good/egg), [Loro](https://github.com/loro-dev/loro),
[JAX](https://github.com/google/jax), [TopoModelX](https://github.com/pyt-team/TopoModelX),
[PyO3](https://github.com/PyO3/pyo3), [Arrow](https://github.com/apache/arrow-rs).