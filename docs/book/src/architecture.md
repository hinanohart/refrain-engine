# Architecture overview

Two languages, seven units, MIT licensed throughout.

```text
┌──────────────────────────────────────────────────┐
│ refrain-adapters (Rust)                          │
│  audio / visual / code / text emitters           │
└──────────────────────────────────────────────────┘
              ↑
┌──────────────────────────────────────────────────┐
│ refrain-ffi (Rust, cdylib)                       │
│  PyO3 _native module                             │
└──────────────────────────────────────────────────┘
       ↑                          ↑
┌──────────────────┐       ┌──────────────────────┐
│ refrain-egraph   │       │ python/              │
│ (egg rewrite)    │       │  intensity_plane     │
└──────────────────┘       │  refrain_py          │
       ↑                   └──────────────────────┘
┌──────────────────┐
│ refrain-core     │       ┌──────────────────────┐
│ (parser + AST)   │       │ refrain-rhizome      │
└──────────────────┘       │ (Loro bridge, opt-in)│
                           └──────────────────────┘
```

## Why two languages

- **Rust** for everything that touches the AST, the e-graph, and
  binary output formats (audio, visual). This is where determinism,
  performance, and zero-cost abstraction matter.
- **Python** for the intensity plane (autodiff, cell-complex topology)
  because the eventual users — computational musicologists and
  composers — live in the Python ecosystem. The boundary is a thin
  PyO3 module exporting JSON-stringified refrains; the heavy lifting
  stays in Rust.

## Why `egg` and not `egglog`

The architecture document originally specified `egglog` (the Datalog
front-end on top of e-graphs). During the Phase 4 implementation
window the `egglog` 0.4 API was in flux, so the implementation uses
`egg` (the underlying e-graph crate) directly. Full egglog Datalog
rules are scheduled for v0.2 — the v0.1 rewrite set fits natively in
`egg`.

## Why a hand-rolled parser

The S-expression grammar is small and stable enough that a hand-rolled
tokenizer plus recursive-descent parser is shorter, more predictable,
and dependency-free. `chumsky` would not change a user-visible
behaviour.

## Why `feature = "rhizome"`

Loro is in `1.0.0-beta`; betas on crates.io can be **yanked**. By
hiding the Loro integration behind an opt-in feature flag the default
build is unaffected by upstream churn. See [Risks](./risks.md) for
the full mitigation plan.
