"""kuna-engine reference-decompiler adapter (the W11 Rust-engine pipeline binding).

The pipeline's comparison machinery (``compare``/``sweep``/``rank``/``worker``) is
decompiler-agnostic: it maps a target to a :class:`ReferenceResult` and diffs that
against kuna-under-test, structurally. This adapter makes a *kuna engine itself* a
reference, so the same machinery can produce a **kuna-rust vs kuna-cpp** row (a pure
differential between the two ports on the same real ELF) -- or feed either engine into
the existing angr comparison.

Two engines are exposed through the registry:

  * ``kuna-cpp``  -- the C++ decompiler (the untouched oracle / W11 baseline).
  * ``kuna-rust`` -- the Rust port (``KUNA_ENGINE=rust``; W11 object-crate ELF
    ``LoadImage``).

Unlike :class:`AngrReference`, no second interpreter is needed: this runs in the kuna
venv by calling :func:`kuna.decompile.decompile` directly with an explicit ``engine=``
(concurrency-safe -- it never mutates ``KUNA_ENGINE``). The "version" pinned into every
result is the engine tag plus the resolved ``decomp_dbg`` path, so a "kuna-rust differs
from kuna-cpp here" claim is reproducible against a known build.

Selectors: a function name, a ``0x``-hex address, or the literal ``@entry`` (the ELF
entry point, resolved by name ``_start`` first, else by the binary's e_entry via the
``object`` crate is *not* available here -- ``@entry`` falls back to the name path).
"""
from __future__ import annotations

import os

from .. import config
from .base import ReferenceDecompiler, ReferenceResult
from ... import decompile as kdecompile
from ... import paths


class KunaReference(ReferenceDecompiler):
    """A kuna decompiler engine, used as a comparison reference.

    ``engine`` is ``"cpp"`` or ``"rust"``; ``name`` is the registry key
    (``kuna-cpp`` / ``kuna-rust``).
    """

    def __init__(self, engine="cpp", timeout=None):
        if engine not in ("cpp", "rust"):
            raise ValueError("kuna engine must be 'cpp' or 'rust', not %r" % engine)
        self.engine = engine
        self.name = "kuna-%s" % engine
        self.timeout = timeout or config.KUNA_TIMEOUT

    # -- ReferenceDecompiler protocol ---------------------------------------

    def _binary_path(self):
        return paths.binary("decomp_dbg", "KUNA_DECOMP_DBG", engine=self.engine)

    def available(self) -> bool:
        return os.path.exists(str(self._binary_path()))

    def version(self):
        # No semantic version for the in-tree build; pin the engine tag + binary path
        # so a differential is reproducible against a known artifact.
        return "kuna-%s (%s)" % (self.engine, self._binary_path())

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
            res.error = ("kuna %s engine not built at %s -- "
                         "`make binaries` (cpp) / "
                         "`cargo build --release -p kuna-console` (rust)"
                         % (self.engine, self._binary_path()))
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
                binary, sel, by_address=by_address, engine=self.engine,
                timeout=timeout or self.timeout,
            )
        except kdecompile.DecompileError as e:
            res.error = "kuna-%s: %s" % (self.engine, e)
            return res

        res.text = text
        res.func_name = sel if not by_address else None
        res.func_addr = sel if by_address else None
        res.meta = {"engine": self.engine, "binary_path": str(self._binary_path())}
        return res


class KunaCppReference(KunaReference):
    name = "kuna-cpp"

    def __init__(self, timeout=None):
        super().__init__(engine="cpp", timeout=timeout)


class KunaRustReference(KunaReference):
    name = "kuna-rust"

    def __init__(self, timeout=None):
        super().__init__(engine="rust", timeout=timeout)
