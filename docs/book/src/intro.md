# Differential Refrain Engine

A unified DSL for time-pattern composition across audio, visual,
code-rewrite, and text media.

```lisp
(refrain melody-a
  (territorialize (loop 4 (note C4 q)))
  (deterritorialize (dy/dx intensity time))
  (reterritorialize (quotient ~rotation ~transpose)))
```

The engine compiles a refrain through these layers, in order:

1. **Parser** (`refrain-core`) — hand-rolled S-expression front-end.
2. **Normalizer** (`refrain-egraph`) — equality saturation via `egg`.
3. **Intensity plane** (`python/intensity_plane`) — forward-mode dual
   number autodiff plus an Ehrhard-Regnier differential combinator.
4. **Adapters** (`refrain-adapters`) — audio (Strudel JSON / OSC),
   visual (deterministic PNG), code (Python / Rust source), text
   (prose / bullets).
5. **Rhizome** (`refrain-rhizome`, opt-in) — two-layer HashMap bridge
   between Loro CRDT causal ids and egglog e-classes.

## Quickstart

```bash
git clone https://github.com/runza/refrain-engine
cd refrain-engine
cargo test --workspace
python -m venv .venv && source .venv/bin/activate
pip install maturin pytest hypothesis
maturin develop --release
pytest python/tests
```

## Where to go next

- The [Refrain DSL](./dsl.md) — the language reference.
- The [architecture overview](./architecture.md) — how the crates fit
  together.
- The [adapters chapter](./adapters.md) — emitting refrains to media.
