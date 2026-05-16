"""Forward-mode autodiff dual-number tests."""

from __future__ import annotations

import math

import pytest
from hypothesis import given
from hypothesis import strategies as st

from intensity_plane import Dual, derivative, jacfwd


def test_dual_constructor_defaults_du_zero():
    a = Dual(3.0)
    assert a.re == 3.0
    assert a.du == 0.0


def test_dual_add_dual():
    a = Dual(1.0, 2.0) + Dual(3.0, 4.0)
    assert a == Dual(4.0, 6.0)


def test_dual_mul_obeys_product_rule():
    a = Dual(2.0, 1.0)  # represents x with x' = 1
    b = Dual(3.0, 1.0)  # represents y with y' = 1
    c = a * b
    # (xy)' = x'y + xy' = 3 + 2 = 5
    assert c.re == 6.0
    assert c.du == 5.0


def test_dual_sub_with_scalar():
    a = Dual(10.0, 1.0) - 4
    assert a == Dual(6.0, 1.0)


def test_dual_rsub_with_scalar():
    a = 10 - Dual(3.0, 1.0)
    assert a == Dual(7.0, -1.0)


def test_dual_div_by_dual_quotient_rule():
    a = Dual(6.0, 1.0)  # x = 6, dx = 1
    b = Dual(2.0, 0.0)
    c = a / b
    assert c.re == 3.0
    assert c.du == 0.5


def test_dual_neg_negates_both_parts():
    a = -Dual(1.0, -2.0)
    assert a == Dual(-1.0, 2.0)


def test_dual_pow_polynomial_derivative():
    # d/dx (x^3) at x=2 = 12
    x = Dual(2.0, 1.0)
    y = x**3
    assert y.re == 8.0
    assert y.du == 12.0


def test_derivative_of_constant_is_zero():
    assert derivative(lambda x: Dual(5.0, 0.0), 1.0) == 0.0


def test_derivative_of_identity_is_one():
    assert derivative(lambda x: x, 7.5) == 1.0


def test_derivative_of_square_at_three_is_six():
    assert derivative(lambda x: x * x, 3.0) == 6.0


def test_derivative_of_quartic_at_one():
    # d/dx (x^4 + 2x) at x=1 = 4 + 2 = 6
    assert derivative(lambda x: x**4 + 2 * x, 1.0) == pytest.approx(6.0)


def test_jacfwd_simple_scalar_pair():
    # f(x, y) = (x*y, x+y)
    def f(v):
        x, y = v
        return [x * y, x + y]

    J = jacfwd(f, [2.0, 3.0])
    # ∂(xy)/∂x = y = 3, ∂(xy)/∂y = x = 2
    # ∂(x+y)/∂x = 1, ∂(x+y)/∂y = 1
    assert J == [[3.0, 2.0], [1.0, 1.0]]


def test_jacfwd_handles_single_dual_return():
    def f(v):
        (x,) = v
        return x * x  # single Dual back

    J = jacfwd(f, [4.0])
    assert J == [[8.0]]


def test_div_by_zero_re_raises():
    with pytest.raises(ZeroDivisionError):
        _ = Dual(1.0, 1.0) / Dual(0.0, 1.0)


def test_dual_eq_with_int_when_du_zero():
    assert Dual(5.0, 0.0) == 5
    assert Dual(5.0, 1.0) != 5


def test_dual_repr_round_trip_eval():
    a = Dual(1.5, -2.5)
    assert "Dual(1.5, -2.5)" == repr(a)


@given(
    st.floats(min_value=-1e3, max_value=1e3, allow_nan=False, allow_infinity=False),
    st.integers(min_value=0, max_value=5),
)
def test_polynomial_derivative_matches_power_rule(x, n):
    # Skip x == 0 with n == 0 ambiguity
    if x == 0.0 and n == 0:
        return
    d = derivative(lambda v: v**n, x)
    expected = n * (x ** (n - 1)) if n > 0 else 0.0
    assert math.isclose(d, expected, rel_tol=1e-9, abs_tol=1e-9)


@given(
    st.floats(min_value=-100, max_value=100, allow_nan=False, allow_infinity=False),
    st.floats(min_value=-100, max_value=100, allow_nan=False, allow_infinity=False),
)
def test_linearity_of_derivative(a, b):
    # d/dx (a*x + b) = a
    d = derivative(lambda x: a * x + b, 0.5)
    assert math.isclose(d, a, rel_tol=1e-9, abs_tol=1e-9)
