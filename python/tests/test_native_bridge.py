"""Sanity tests for the Rust↔Python PyO3 bridge."""

from __future__ import annotations

import json

import pytest

from refrain_py import _native, native_available


def test_native_available():
    assert native_available() is True


def test_version_string():
    assert isinstance(_native.version(), str)
    assert _native.version() == "0.1.0"


def test_parse_returns_valid_json():
    out = _native.parse_refrain("(refrain a)")
    parsed = json.loads(out)
    assert parsed["name"] == "a"
    assert parsed["territorialize"] is None


def test_parse_handles_full_three_stage_refrain():
    src = "(refrain mel (territorialize (note C4 q)) (deterritorialize (dy/dx i t)) (reterritorialize (quotient ~r)))"
    out = json.loads(_native.parse_refrain(src))
    assert out["name"] == "mel"
    assert out["territorialize"] is not None
    assert out["deterritorialize"] is not None
    assert out["reterritorialize"] is not None


def test_normalize_collapses_loop_one():
    src = "(refrain a (territorialize (loop 1 (note C4 q))))"
    out = json.loads(_native.parse_and_normalize(src))
    # loop-1-identity rule should fire.
    assert out["territorialize"] == {"Op": {"Note": {"pitch": "C4", "dur": "q"}}}


def test_normalize_preserves_loop_n_gt_1():
    src = "(refrain a (territorialize (loop 4 (note C4 q))))"
    out = json.loads(_native.parse_and_normalize(src))
    assert "Loop" in out["territorialize"]["Op"]


def test_parse_raises_value_error_on_garbage():
    with pytest.raises(ValueError):
        _native.parse_refrain("not a refrain")


def test_normalize_raises_value_error_on_invalid_json():
    with pytest.raises(ValueError):
        _native.normalize_refrain("{not json")
