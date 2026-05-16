"""Forward-mode automatic differentiation via dual numbers.

A `Dual` is a pair (re, du) with arithmetic obeying ε² = 0, so the `du`
component of a Dual-valued function evaluates to its derivative. This
module provides the primitive type plus single-variable `derivative` and
multi-variable `jacfwd` (forward-mode Jacobian) helpers.

Pure Python, no JAX dependency. JAX-backed reverse-mode autodiff lands
in v0.2 when wider compute graphs justify the install footprint.
"""

from __future__ import annotations

from typing import Callable, Iterable, Sequence

Number = int | float


class Dual:
    """A dual number x = re + du · ε with ε² = 0."""

    __slots__ = ("re", "du")

    def __init__(self, re: Number, du: Number = 0.0) -> None:
        self.re = float(re)
        self.du = float(du)

    def __repr__(self) -> str:
        return f"Dual({self.re!r}, {self.du!r})"

    def __eq__(self, other: object) -> bool:
        if isinstance(other, Dual):
            return self.re == other.re and self.du == other.du
        if isinstance(other, (int, float)):
            return self.re == other and self.du == 0.0
        return NotImplemented

    def __hash__(self) -> int:
        return hash((self.re, self.du))

    def _coerce(self, other: object) -> "Dual | None":
        if isinstance(other, Dual):
            return other
        if isinstance(other, (int, float)):
            return Dual(other, 0.0)
        return None

    def __add__(self, other: object) -> "Dual":
        o = self._coerce(other)
        if o is None:
            return NotImplemented  # type: ignore[return-value]
        return Dual(self.re + o.re, self.du + o.du)

    __radd__ = __add__

    def __sub__(self, other: object) -> "Dual":
        o = self._coerce(other)
        if o is None:
            return NotImplemented  # type: ignore[return-value]
        return Dual(self.re - o.re, self.du - o.du)

    def __rsub__(self, other: object) -> "Dual":
        o = self._coerce(other)
        if o is None:
            return NotImplemented  # type: ignore[return-value]
        return Dual(o.re - self.re, o.du - self.du)

    def __mul__(self, other: object) -> "Dual":
        o = self._coerce(other)
        if o is None:
            return NotImplemented  # type: ignore[return-value]
        return Dual(self.re * o.re, self.re * o.du + self.du * o.re)

    __rmul__ = __mul__

    def __truediv__(self, other: object) -> "Dual":
        o = self._coerce(other)
        if o is None:
            return NotImplemented  # type: ignore[return-value]
        if o.re == 0.0:
            raise ZeroDivisionError("Dual division by Dual with re == 0")
        return Dual(
            self.re / o.re,
            (self.du * o.re - self.re * o.du) / (o.re * o.re),
        )

    def __rtruediv__(self, other: object) -> "Dual":
        o = self._coerce(other)
        if o is None:
            return NotImplemented  # type: ignore[return-value]
        return o.__truediv__(self)

    def __neg__(self) -> "Dual":
        return Dual(-self.re, -self.du)

    def __pos__(self) -> "Dual":
        return Dual(self.re, self.du)

    def __pow__(self, n: Number) -> "Dual":
        if not isinstance(n, (int, float)):
            return NotImplemented  # type: ignore[return-value]
        if n == 0:
            return Dual(1.0, 0.0)
        # General real power: d/dx (x^n) = n * x^(n-1)
        re_pow = self.re**n
        if self.re == 0.0 and n < 1:
            raise ZeroDivisionError("Dual: 0^negative")
        du_part = n * (self.re ** (n - 1)) * self.du if self.re != 0.0 or n >= 1 else 0.0
        return Dual(re_pow, du_part)


def derivative(f: Callable[[Dual], Dual], x: Number) -> float:
    """Forward-mode derivative of `f` at scalar `x`."""
    result = f(Dual(x, 1.0))
    if not isinstance(result, Dual):
        raise TypeError(f"f must return a Dual, got {type(result).__name__}")
    return result.du


def jacfwd(
    f: Callable[[Sequence[Dual]], Sequence[Dual] | Dual],
    x: Sequence[Number],
) -> list[list[float]]:
    """Forward-mode Jacobian of vector-valued `f` at `x`.

    Returns a list-of-rows representation; J[i][j] = ∂fᵢ/∂xⱼ.
    """
    n_in = len(x)
    cols: list[list[float]] = []
    n_out = -1
    for j in range(n_in):
        duals: list[Dual] = [Dual(v, 1.0 if i == j else 0.0) for i, v in enumerate(x)]
        ys = f(duals)
        if isinstance(ys, Dual):
            ys_iter: Iterable[Dual] = [ys]
        else:
            ys_iter = ys
        col = [y.du for y in ys_iter]
        if n_out == -1:
            n_out = len(col)
        elif len(col) != n_out:
            raise ValueError(f"f produced inconsistent output arity ({len(col)} vs {n_out})")
        cols.append(col)
    # transpose cols (n_in, n_out) -> rows (n_out, n_in)
    return [[cols[j][i] for j in range(n_in)] for i in range(n_out)]


__all__ = ["Dual", "derivative", "jacfwd", "Number"]
