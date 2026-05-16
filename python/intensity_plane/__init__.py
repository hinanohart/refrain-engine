"""intensity_plane: JAX autodiff + cell-complex topology layer for Refrain.

Phase 5 wires up the actual implementation. This module currently exposes
public type stubs so import succeeds.
"""

from __future__ import annotations

__version__ = "0.1.0"
__all__ = ["dyt", "ehrhard_regnier_d", "IntensityPlane"]


class IntensityPlane:
    """Placeholder for the topology-aware intensity field (Phase 5)."""

    def __init__(self) -> None:
        self._cells: list = []


def dyt(_y, _t):  # noqa: ANN001,ANN201 — fully typed in Phase 5
    """Forward-mode dual-number Jacobian d y / d t (Phase 5 stub)."""
    raise NotImplementedError("dyt lands in Phase 5")


def ehrhard_regnier_d(_term):  # noqa: ANN001,ANN201
    """Ehrhard-Regnier differential combinator D[term] (Phase 5 stub)."""
    raise NotImplementedError("ehrhard_regnier_d lands in Phase 5")
