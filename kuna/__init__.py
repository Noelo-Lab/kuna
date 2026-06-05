"""kuna -- a standalone extraction of Ghidra's C++ decompiler with a Python wrapper.

Public API (lazily imported so `python -m kuna.decompile` doesn't double-import):
    from kuna import decompile, DecompileError, FunctionNotFound, LoadError
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
    # the package does not eagerly load kuna.decompile (which would trigger a
    # RuntimeWarning under `python -m kuna.decompile`).
    if name in _LAZY:
        import importlib
        mod = importlib.import_module("." + _LAZY[name], __name__)
        return getattr(mod, name)
    raise AttributeError("module %r has no attribute %r" % (__name__, name))
