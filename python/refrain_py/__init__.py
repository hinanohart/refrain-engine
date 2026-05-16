"""refrain_py: high-level Python API for Differential Refrain Engine.

Imports the PyO3-built native module and exposes a sober Python surface.
Phase 5 fills out the bridge.
"""

from __future__ import annotations

__version__ = "0.1.0"

# The PyO3 native module is built by maturin; it is intentionally optional
# here so this package can be imported during scaffold/test phases.
try:
    from . import _native  # type: ignore[attr-defined]

    _NATIVE = True
except ImportError:
    _native = None  # type: ignore[assignment]
    _NATIVE = False


def native_available() -> bool:
    return _NATIVE


__all__ = ["native_available"]
