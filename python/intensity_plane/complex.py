"""Lightweight cell complex for the intensity plane.

A *cell complex* is a finite set of cells of varying dimension, with each
cell of dimension d carrying boundary references to (d-1)-cells. This
v0.1 representation is intentionally minimal — TopoModelX integration
(zero-copy via DLPack) is scheduled for v0.2 (see `docs/roadmap.md`).
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class Cell:
    """A single cell in a cell complex."""

    id: int
    dim: int
    boundary: tuple[int, ...] = ()
    payload: dict = field(default_factory=dict)


@dataclass
class CellComplex:
    """A finite cell complex indexed by cell id."""

    cells: dict[int, Cell] = field(default_factory=dict)

    def add(self, dim: int, boundary: tuple[int, ...] = (), payload: dict | None = None) -> int:
        cid = len(self.cells)
        self.cells[cid] = Cell(id=cid, dim=dim, boundary=boundary, payload=payload or {})
        return cid

    def of_dim(self, d: int) -> list[Cell]:
        return [c for c in self.cells.values() if c.dim == d]

    def boundary_of(self, cid: int) -> tuple[int, ...]:
        return self.cells[cid].boundary

    def chain_complex_size(self) -> dict[int, int]:
        """Count cells per dimension — handy for sanity tests."""
        out: dict[int, int] = {}
        for c in self.cells.values():
            out[c.dim] = out.get(c.dim, 0) + 1
        return out

    def euler_characteristic(self) -> int:
        """χ = Σ_d (-1)^d · |C_d|."""
        total = 0
        for d, n in self.chain_complex_size().items():
            total += (-1) ** d * n
        return total


__all__ = ["Cell", "CellComplex"]
