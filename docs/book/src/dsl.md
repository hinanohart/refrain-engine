# The Refrain DSL

The Refrain DSL is an S-expression language. A program is a single
`refrain` form.

## Grammar

```text
refrain    ::= "(" "refrain" IDENT stage* ")"
stage      ::= "(" stage-kind pattern+ ")"
stage-kind ::= "territorialize" | "deterritorialize" | "reterritorialize"
pattern    ::= ATOM | "(" op-head pattern* ")"
op-head    ::= "note" | "loop" | "dy/dx" | "quotient" | IDENT
```

Comments start with `;` and extend to the end of the line.

## Stages

A `refrain` has up to three stages. Each is optional.

| Stage | Meaning |
|---|---|
| `territorialize` | Establish an initial structured pattern. |
| `deterritorialize` | Apply a differential or decomposition. |
| `reterritorialize` | Re-fix the result under named equivalences. |

The stages are evaluated in order. A pattern inside a stage is either a
single pattern or a whitespace-separated sequence (parsed as a `Seq`).

## Built-in operators

### `note PITCH DUR`

A single sound event. `PITCH` is an atom (`C4`, `G#3`, etc.). `DUR` is
one of `w`, `h`, `q`, `e`, `s` (whole, half, quarter, eighth,
sixteenth), or a numeric cycle fraction (e.g. `0.5`).

```lisp
(note C4 q)
```

### `loop COUNT BODY`

Repeat `BODY` `COUNT` times consecutively.

```lisp
(loop 4 (note C4 q))
```

The rewrite rule `(loop 1 ?x) → ?x` fires during normalization.

### `dy/dx X T`

A differential placeholder. Marks "rate of change of `X` with respect to
`T`." The intensity-plane Python layer interprets this symbolically.

```lisp
(dy/dx intensity time)
```

### `quotient REL...`

Identify patterns under named equivalence relations.

```lisp
(quotient ~rotation ~transpose)
```

### Generic `Call` forms

Any other parenthesized form is preserved as `Op::Call { head, args }`:

```lisp
(rest h)
```

This lets future adapters introduce new operators without bumping the
parser.

## Examples

```lisp
(refrain melody-a
  (territorialize (loop 4 (note C4 q)))
  (deterritorialize (dy/dx intensity time))
  (reterritorialize (quotient ~rotation ~transpose)))
```

```lisp
(refrain pulse
  (territorialize (loop 2 (loop 3 (note G4 e))))
  (deterritorialize (dy/dx pitch time)))
```
