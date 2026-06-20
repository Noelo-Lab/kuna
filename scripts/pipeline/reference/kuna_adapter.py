"""kuna-engine reference-decompiler adapter.

The pipeline's comparison machinery (``compare``/``sweep``/``rank``/``worker``) is
decompiler-agnostic: it maps a target to a :class:`ReferenceResult` and diffs that
against kuna-under-test, structurally. This adapter exposes the kuna engine itself as
a reference (registry key ``kuna``), parallel to :class:`AngrReference`.

It runs in the same interpreter by calling :func:`decompile` directly. The "version"
pinned into every result is the resolved ``decomp_dbg`` path, so a claim is
reproducible against a known build.

Selectors: a function name, a ``0x``-hex address, or the literal ``@entry`` (resolved
via the conventional ELF entry symbol ``_start``).
"""
from __future__ import annotations

import os

from .. import config
from .base import ReferenceDecompiler, ReferenceResult
from ... import decompile as kdecompile
from ... import paths


class KunaReference(ReferenceDecompiler):
    """The kuna decompiler, used as a comparison reference (registry key ``kuna``)."""

    name = "kuna"

    def __init__(self, timeout=None):
        self.timeout = timeout or config.KUNA_TIMEOUT

    # -- ReferenceDecompiler protocol ---------------------------------------

    def _binary_path(self):
        return paths.decomp_dbg()

    def available(self) -> bool:
        return os.path.exists(str(self._binary_path()))

    def version(self):
        # No semantic version for the in-tree build; pin the resolved binary path
        # so a differential is reproducible against a known artifact.
        return "kuna (%s)" % self._binary_path()

    def decompile(self, binary, selector, *, arch=None, load_debug=False,
                  structurer=None, timeout=None) -> ReferenceResult:
        binary = os.path.abspath(str(binary))
        selector = str(selector)
        res = ReferenceResult(source=self.name, binary=binary, selector=selector,
                              version=self.version())

        if not os.path.exists(binary):
            res.error = "binary not found: %s" % binary
            return res
        if not self.available():
            res.error = ("kuna engine not built at %s -- run `make binaries`"
                         % self._binary_path())
            return res

        # @entry: kuna's console has no e_entry selector; resolve via the conventional
        # ELF entry symbol, leaving an explicit failure for the caller otherwise.
        sel = selector
        by_address = False
        if sel == "@entry":
            sel = "_start"
        elif sel.lower().startswith("0x"):
            by_address = True

        try:
            text = kdecompile.decompile(
                binary, sel, by_address=by_address, timeout=timeout or self.timeout,
            )
        except kdecompile.DecompileError as e:
            res.error = "kuna: %s" % e
            return res

        res.text = text
        res.func_name = sel if not by_address else None
        res.func_addr = sel if by_address else None
        res.meta = {"binary_path": str(self._binary_path())}
        return res
