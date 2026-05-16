"""Ehrhard-Regnier D combinator tests on Refrain Pattern JSON."""

from __future__ import annotations

import json


from intensity_plane import ehrhard_regnier_d
from refrain_py import _native


def parse_pattern(src: str) -> dict:
    """Parse a refrain and return its territorialize stage body."""
    rj = json.loads(_native.parse_refrain(src))
    return rj["territorialize"]


def test_d_on_constant_note_marks_duration():
    src = "(refrain a (territorialize (note C4 q)))"
    p = parse_pattern(src)
    d = ehrhard_regnier_d(p, "time")
    assert d == {"Op": {"Note": {"pitch": "C4", "dur": "d_q"}}}


def test_d_on_loop_commutes():
    src = "(refrain a (territorialize (loop 4 (note C4 q))))"
    p = parse_pattern(src)
    d = ehrhard_regnier_d(p, "time")
    assert d == {
        "Op": {
            "Loop": {
                "count": 4,
                "body": {"Op": {"Note": {"pitch": "C4", "dur": "d_q"}}},
            }
        }
    }


def test_d_on_diff_left_abstract():
    src = "(refrain a (deterritorialize (dy/dx intensity time)))"
    rj = json.loads(_native.parse_refrain(src))
    p = rj["deterritorialize"]
    d = ehrhard_regnier_d(p, "time")
    assert d == {"Op": {"Diff": {"x": "intensity", "t": "time"}}}


def test_d_on_quotient_invariant():
    src = "(refrain a (reterritorialize (quotient ~rot ~scale)))"
    rj = json.loads(_native.parse_refrain(src))
    p = rj["reterritorialize"]
    d = ehrhard_regnier_d(p, "time")
    assert d == {"Op": {"Quotient": {"rels": ["~rot", "~scale"]}}}


def test_d_on_sym_matches_var():
    p = {"Op": {"Sym": "time"}}
    assert ehrhard_regnier_d(p, "time") == {"Op": {"Sym": "1"}}


def test_d_on_sym_does_not_match_var():
    p = {"Op": {"Sym": "pitch"}}
    assert ehrhard_regnier_d(p, "time") == {"Op": {"Sym": "0"}}


def test_d_on_call_distributes_over_args():
    p = {"Op": {"Call": {"head": "rest", "args": [{"Op": {"Sym": "h"}}]}}}
    d = ehrhard_regnier_d(p, "h")
    assert d == {"Op": {"Call": {"head": "rest", "args": [{"Op": {"Sym": "1"}}]}}}


def test_d_on_seq_distributes_element_wise():
    src = "(refrain a (territorialize (note C4 q) (note D4 q)))"
    p = parse_pattern(src)
    d = ehrhard_regnier_d(p, "time")
    assert d == {
        "Seq": [
            {"Op": {"Note": {"pitch": "C4", "dur": "d_q"}}},
            {"Op": {"Note": {"pitch": "D4", "dur": "d_q"}}},
        ]
    }


def test_d_idempotent_on_quotient():
    p = {"Op": {"Quotient": {"rels": ["~r"]}}}
    once = ehrhard_regnier_d(p, "x")
    twice = ehrhard_regnier_d(once, "x")
    assert once == twice


def test_d_idempotent_on_diff():
    p = {"Op": {"Diff": {"x": "i", "t": "t"}}}
    once = ehrhard_regnier_d(p, "t")
    twice = ehrhard_regnier_d(once, "t")
    assert once == twice
