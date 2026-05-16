"""Ehrhard-Regnier differential combinator D[·] for Refrain patterns.

The Ehrhard-Regnier differential λ-calculus (2003) extends the simply-typed
λ-calculus with a syntactic differential operator that distributes over
application linearly. Specializing to refrains gives a structural rule
for symbolically lifting a pattern to its differential form:

* D[note p d]     = note p (D[d])     — only the duration field is differentiable.
* D[loop n body]  = loop n (D[body])   — D commutes with iteration.
* D[seq xs]       = seq (D[x] for x in xs)
* D[diff x t]     = diff x t            — left as a primitive; second derivatives
                                          require D's chain rule which v0.1 does not
                                          attempt symbolically.
* D[quotient r…]  = quotient r…         — equivalence classes are invariant.
* D[sym y]        = sym 1 if y == var   — the variable being differentiated lifts
                  = sym 0 otherwise.
* D[call h a…]    = call h (D[a]…)      — opaque heads get their args lifted.

The output of D is a Refrain Pattern in JSON form (matching the
`refrain_core::ast::Pattern` serde layout).
"""

from __future__ import annotations

from typing import Any


def ehrhard_regnier_d(pattern: dict[str, Any] | str, var: str) -> dict[str, Any] | str:
    """Apply D[·] w.r.t. `var` to a Pattern node.

    `pattern` is JSON-shaped per the Rust AST: e.g.
        {"Op": {"Note": {"pitch": "C4", "dur": "q"}}}
        {"Op": {"Loop": {"count": 4, "body": <pattern>}}}
        {"Op": {"Sym": "intensity"}}
        {"Op": {"Call": {"head": "rest", "args": [<pattern>...]}}}
        {"Seq": [<pattern>, ...]}
    """
    if isinstance(pattern, str):
        # Bare atom case (rare; typically Op::Sym wraps atoms).
        return "1" if pattern == var else "0"

    if "Seq" in pattern:
        return {"Seq": [ehrhard_regnier_d(p, var) for p in pattern["Seq"]]}

    if "Op" not in pattern:
        return pattern

    op = pattern["Op"]
    if isinstance(op, str):
        return {"Op": {"Sym": "1" if op == var else "0"}}

    key, body = next(iter(op.items()))

    if key == "Note":
        return {"Op": {"Note": {"pitch": body["pitch"], "dur": f"d_{body['dur']}"}}}
    if key == "Loop":
        return {
            "Op": {
                "Loop": {
                    "count": body["count"],
                    "body": ehrhard_regnier_d(body["body"], var),
                }
            }
        }
    if key == "Diff":
        return {"Op": {"Diff": dict(body)}}
    if key == "Quotient":
        return {"Op": {"Quotient": {"rels": list(body["rels"])}}}
    if key == "Sym":
        return {"Op": {"Sym": "1" if body == var else "0"}}
    if key == "Call":
        return {
            "Op": {
                "Call": {
                    "head": body["head"],
                    "args": [ehrhard_regnier_d(a, var) for a in body["args"]],
                }
            }
        }

    return pattern


__all__ = ["ehrhard_regnier_d"]
