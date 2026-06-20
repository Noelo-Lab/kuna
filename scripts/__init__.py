"""scripts -- Python helpers for the Rust kuna decompiler.

The user-facing CLI is the Rust `kuna` binary; what remains here is the still-Python
feature pipeline (``scripts/pipeline/``) and the thin ``decompile`` library shim it
depends on. Run from the repo root (``python -m scripts.pipeline.<mod>``).

Public API (lazily imported):
    from scripts import decompile, DecompileError, FunctionNotFound, LoadError
"""

__version__ = "0.1.0"

_LAZY = {
    "decompile": "decompile",
    "DecompileError": "decompile",
    "FunctionNotFound": "decompile",
    "LoadError": "decompile",
    "SpecsNotFound": "decompile",
    "NoOutput": "decompile",
    "DecompileTimeout": "decompile",
}

__all__ = list(_LAZY)


def __getattr__(name):
    # PEP 562: import the submodule only on first attribute access, so importing
    # the package does not eagerly load scripts.decompile.
    if name in _LAZY:
        import importlib
        mod = importlib.import_module("." + _LAZY[name], __name__)
        return getattr(mod, name)
    raise AttributeError("module %r has no attribute %r" % (__name__, name))
