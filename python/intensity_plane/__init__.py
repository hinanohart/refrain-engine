"""intensity_plane: forward-mode autodiff + cell complex layer for Refrain.

The intensity plane sits between the Rust core (`refrain_core` AST) and
the four media adapters. It provides:

* `Dual`, `derivative`, `jacfwd` — forward-mode autodiff over dual
  numbers (pure Python; ε² = 0).
* `ehrhard_regnier_d` — symbolic differential combinator over Refrain
  Pattern JSON.
* `CellComplex`, `Cell` — finite cell complex with boundary references.

JAX (reverse-mode autodiff over compute graphs) and TopoModelX
(higher-order topology layers via DLPack zero-copy) integrate in v0.2;
see `docs/roadmap.md`.
"""

from __future__ import annotations

from .autodiff import Dual, derivative, jacfwd
from .complex import Cell, CellComplex
from .ehrhard import ehrhard_regnier_d

__version__ = "0.1.0"
__all__ = [
    "Dual",
    "derivative",
    "jacfwd",
    "ehrhard_regnier_d",
    "Cell",
    "CellComplex",
]
