"""The reference-decompiler contract shared by every adapter."""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class ReferenceResult:
    """The decompilation a reference decompiler produced for one function.

    ``text`` is None when the reference failed (see ``error``). ``version`` pins the
    reference decompiler's version so a "the reference is better here" claim is
    reproducible (this matters: decompiler output is version-sensitive). ``meta`` holds
    the raw adapter payload for debugging.
    """
    source: str                       # which reference decompiler ("angr", ...)
    binary: str                       # absolute binary path
    selector: str                     # the requested function selector (name / 0xaddr / @entry)
    text: Optional[str] = None        # decompiled C, or None on failure
    version: Optional[str] = None     # reference decompiler version (e.g. "9.2.213")
    func_name: Optional[str] = None   # resolved function name
    func_addr: Optional[str] = None   # resolved entry address as "0x..."
    error: Optional[str] = None       # failure reason, or None
    meta: dict = field(default_factory=dict)

    @property
    def ok(self) -> bool:
        return self.text is not None and self.error is None

    def to_dict(self) -> dict:
        return {
            "source": self.source, "binary": self.binary, "selector": self.selector,
            "text": self.text, "version": self.version, "func_name": self.func_name,
            "func_addr": self.func_addr, "error": self.error,
        }


class ReferenceDecompiler:
    """Abstract reference decompiler. Subclass per tool (see AngrReference)."""

    name = "reference"

    def available(self) -> bool:
        """True if this reference decompiler can actually run on this machine."""
        raise NotImplementedError

    def version(self) -> Optional[str]:
        """The reference decompiler's version string, or None if unavailable."""
        raise NotImplementedError

    def decompile(self, binary, selector, *, arch=None, load_debug=False,
                  structurer=None, timeout=None) -> ReferenceResult:
        """Decompile one function. ``selector`` is a name, a 0x-hex address, or @entry."""
        raise NotImplementedError
