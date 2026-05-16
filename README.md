# Differential Refrain Engine

A unified DSL for time-pattern composition across audio, visual, code-rewrite, and text media —
integrating Rust **egglog** equality saturation, **Loro CRDT** (eg-walker) collaborative editing,
**JAX** forward-mode automatic differentiation, and **cell complex** topology layers
(via TopoModelX / GUDHI) behind a single S-expression refrain language.

## Status

`v0.1.0` — initial public release. Bus factor 1; expect API churn.

## What it does

```lisp
(refrain melody-a
  (territorialize (loop 4 (note C4 q)))
  (deterritorialize (dy/dx intensity time))
  (reterritorialize (quotient ~rotation ~transpose)))
```

A `refrain` is a named, structured time-pattern. The engine:

1. **Parses** the S-expression DSL (`refrain-core`, chumsky-based parser, ~300 LOC).
2. **Normalizes** via `egglog` rewrite rules with an extraction cost model (`refrain-egraph`).
3. **Differentiates** via dual-number forward-mode + Ehrhard-Regnier differential combinators
   (`python/intensity_plane`, JAX-backed).
4. **Emits** to a pluggable adapter (audio via Strudel/OSC, visual via wgpu, code via templates,
   text via simple n-gram), trait-registered through `inventory::submit!`.
5. **Optionally syncs** patterns across collaborators via Loro CRDT eg-walker
   (`refrain-rhizome`, opt-in feature flag).

## Crate layout

| Crate | LOC | Role |
|---|---|---|
| `refrain-core` | ~3500 | DSL parser, AST, public API |
| `refrain-egraph` | ~2000 | egglog FFI, rewrite rules, extraction |
| `refrain-rhizome` | ~1500 | Loro CRDT bridge (feature = `rhizome`) |
| `refrain-ffi` | ~800 | PyO3 boundary (Arrow IPC zero-copy) |
| `refrain-adapters` | ~1200 | adapter trait + 4 built-ins |
| `python/intensity_plane` | ~2500 | JAX autodiff + TopoModelX cell complex |
| `python/refrain_py` | ~500 | high-level Python API |

## Install (post-v0.1)

```bash
# Rust core
cargo add refrain-core

# Python bindings
pip install refrain-py
```

## Quickstart

See [`examples/`](./examples/) for working refrains across all 4 adapters.

## Philosophy footnote

The names *refrain*, *territorialization*, *deterritorialization*, and *intensive multiplicity*
are inspired by Gilles Deleuze and Félix Guattari's *Mille Plateaux* (1980) and
Deleuze's *Différence et Répétition* (1968). The implementation does **not** claim
mathematical isomorphism with their philosophical concepts — these are evocative names for
concrete technical operations (looped pattern primitive, structural decomposition,
recomposition with constraints, autodiff Jacobian directions). See `docs/philosophy.md` for
a sober mapping.

## License

MIT — see [LICENSE](./LICENSE).

## Acknowledgements

Built on: [chumsky](https://github.com/zesterer/chumsky), [egglog](https://github.com/egraphs-good/egglog),
[Loro](https://github.com/loro-dev/loro), [JAX](https://github.com/google/jax),
[TopoModelX](https://github.com/pyt-team/TopoModelX), [PyO3](https://github.com/PyO3/pyo3),
[Arrow](https://github.com/apache/arrow-rs).
