"""Additional hypothesis property tests across autodiff and ehrhard."""

from __future__ import annotations

import math

from hypothesis import given
from hypothesis import strategies as st

from intensity_plane import Dual, derivative, ehrhard_regnier_d


@given(
    st.floats(min_value=-100, max_value=100, allow_nan=False, allow_infinity=False),
    st.floats(min_value=-100, max_value=100, allow_nan=False, allow_infinity=False),
    st.floats(min_value=-10, max_value=10, allow_nan=False, allow_infinity=False),
)
def test_dual_sum_rule(a, b, x):
    # d/dx (a + b·x) = b
    d = derivative(lambda v: a + b * v, x)
    assert math.isclose(d, b, rel_tol=1e-9, abs_tol=1e-9)


@given(
    st.floats(min_value=-50, max_value=50, allow_nan=False, allow_infinity=False),
    st.floats(min_value=-50, max_value=50, allow_nan=False, allow_infinity=False),
)
def test_dual_product_rule(a, b):
    # d/dx (x · (x + a + b)) at x=2 = 2 · 2 + a + b = 4 + a + b
    x = 2.0
    d = derivative(lambda v: v * (v + a + b), x)
    assert math.isclose(d, 4.0 + a + b, rel_tol=1e-9, abs_tol=1e-9)


@given(
    st.integers(min_value=0, max_value=4),
    st.floats(min_value=0.1, max_value=10.0, allow_nan=False, allow_infinity=False),
)
def test_dual_chain_rule_with_pow(n, x):
    # d/dx ((x + 1)^n) at x = n · (x + 1)^(n - 1) by chain rule
    d = derivative(lambda v: (v + 1) ** n, x)
    expected = n * ((x + 1) ** (n - 1)) if n > 0 else 0.0
    assert math.isclose(d, expected, rel_tol=1e-9, abs_tol=1e-9)


@given(st.text(alphabet="abcdefgh", min_size=1, max_size=8))
def test_ehrhard_sym_match_returns_one(var):
    p = {"Op": {"Sym": var}}
    out = ehrhard_regnier_d(p, var)
    assert out == {"Op": {"Sym": "1"}}


@given(
    st.text(alphabet="abcdefgh", min_size=1, max_size=8),
    st.text(alphabet="ijklmnop", min_size=1, max_size=8),
)
def test_ehrhard_sym_nomatch_returns_zero(sym, var):
    p = {"Op": {"Sym": sym}}
    out = ehrhard_regnier_d(p, var)
    assert out == {"Op": {"Sym": "0"}}


@given(
    st.integers(min_value=1, max_value=10),
    st.text(alphabet="ABCDEFG", min_size=1, max_size=1),
    st.sampled_from(["w", "h", "q", "e", "s"]),
)
def test_ehrhard_on_loop_preserves_count(count, pitch, dur):
    p = {
        "Op": {
            "Loop": {
                "count": count,
                "body": {"Op": {"Note": {"pitch": f"{pitch}4", "dur": dur}}},
            }
        }
    }
    out = ehrhard_regnier_d(p, "time")
    assert out["Op"]["Loop"]["count"] == count


def test_dual_equality_is_reflexive():
    a = Dual(3.0, 1.0)
    assert a == a


def test_dual_eq_uses_re_when_compared_to_int():
    assert Dual(7.0, 0.0) == 7
    assert Dual(7.0, 0.0) != 8
