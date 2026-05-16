# Philosophy footnote — what is and is not claimed

The Differential Refrain Engine borrows several names from Gilles Deleuze and
Félix Guattari. This document is the sober map between the names and the
concrete technical operations they label.

## What is **not** claimed

We do **not** claim mathematical isomorphism between any operation in this
software and any philosophical concept in *Différence et Répétition* (1968) or
*Mille Plateaux* (1980). In particular:

- The `dy/dx` form is **not** Deleuze's "differential of the Idea" in a literal
  sense. It is plain forward-mode automatic differentiation over dual numbers
  with optional Ehrhard-Regnier differential combinator rewrites.
- The `Pattern::Seq` and Loro CRDT layer are **not** "rhizomes." They are an
  ordered list and a causal event-graph DAG.
- The `Op::Quotient { rels }` form is **not** Deleuze's "repetition." It is an
  egglog/egg-style equivalence quotient parametrized by named relations.
- A topology-aware cell complex (`python/intensity_plane`) is **not** Deleuze's
  *multiplicité*. It is a finite cell complex stored as a dict-of-Arrow
  batches, processable by TopoModelX layers.

These earlier framings (treating philosophy as 1:1 mathematical structure) were
rejected during concept review. The current design uses Deleuze's vocabulary
only as **evocative names** for operations that stand on their own technical
merits.

## What the names label

| Refrain DSL keyword | Technical operation | Why this name |
|---|---|---|
| `refrain` | A named, three-stage time-pattern definition | Deleuze & Guattari's "ritornello" (refrain) is the recurring chunk a territory is woven from; for us, it is the unit of compositional reuse. |
| `territorialize` | Establish an initial structured pattern | Initial pattern fixing. |
| `deterritorialize` | Apply differential / decomposition to that pattern | Structural decomposition (in our case, autodiff Jacobian). |
| `reterritorialize` | Re-fix the result under constraints (quotient by relations) | egg-style canonicalization under named equivalences. |
| `dy/dx` | Dual-number forward-mode autodiff | Visually familiar to musicians without prior Deleuze exposure. |
| `quotient` | Equivalence quotient under listed relations | Standard universal-algebra terminology. |

## Why use the names at all

Two reasons:

1. **The names are good user-facing labels.** A musician approaching the system
   can read `(loop 4 (note C4 q))` as a refrain, and `(dy/dx intensity time)` as
   "the rate at which intensity changes over time" — without needing to first
   master e-graphs or category theory.
2. **The names are recognizable to a specific contributor pool.** Computational
   musicologists and live-coders familiar with Deleuze's vocabulary can read
   the source as a coherent system. For everyone else, they are just names.

If the names ever start carrying load — if a feature is justified purely by
analogy to Deleuze rather than by an independent technical argument — they
should be retired.
