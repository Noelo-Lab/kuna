"""kuna — a standalone extraction of Ghidra's C++ decompiler with a Python wrapper.

Public API:
    from kuna import decompile, DecompileError, FunctionNotFound, LoadError
"""
from .decompile import (
    decompile,
    DecompileError,
    FunctionNotFound,
    LoadError,
    SpecsNotFound,
    NoOutput,
    DecompileTimeout,
)

__all__ = [
    "decompile",
    "DecompileError",
    "FunctionNotFound",
    "LoadError",
    "SpecsNotFound",
    "NoOutput",
    "DecompileTimeout",
]

__version__ = "0.1.0"
