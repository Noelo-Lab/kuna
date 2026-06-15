"""Reference decompilers: the "what does a better decompiler produce here" side.

A ReferenceDecompiler maps (binary, function) -> ReferenceResult (the decompiled C text
plus provenance). angr is the first implementation; the rest of the pipeline
(worklist/compare/sweep/worker) is decompiler-agnostic, so adding reko / Binary Ninja /
IDA later is just a new adapter here.
"""
from .base import ReferenceDecompiler, ReferenceResult
from .angr_adapter import AngrReference
from .kuna_adapter import KunaReference, KunaCppReference, KunaRustReference

#: Registry of available reference decompilers by name.
#:
#: ``kuna-cpp`` / ``kuna-rust`` make a kuna engine itself a reference, so the same
#: compare machinery produces a kuna-rust-vs-kuna-cpp differential on a real ELF
#: (the W11 Rust-engine pipeline binding) -- not just angr-vs-kuna.
REGISTRY = {
    "angr": AngrReference,
    "kuna-cpp": KunaCppReference,
    "kuna-rust": KunaRustReference,
}


def get_reference(name="angr", **kw):
    """Instantiate a reference decompiler by name."""
    try:
        cls = REGISTRY[name]
    except KeyError:
        raise ValueError("unknown reference decompiler %r (have: %s)"
                         % (name, ", ".join(sorted(REGISTRY))))
    return cls(**kw)


__all__ = ["ReferenceDecompiler", "ReferenceResult", "AngrReference",
           "KunaReference", "KunaCppReference", "KunaRustReference",
           "REGISTRY", "get_reference"]
