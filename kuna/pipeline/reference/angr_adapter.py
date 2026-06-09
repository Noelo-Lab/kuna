"""angr reference-decompiler adapter.

Bridges the two-interpreter gap: the kuna venv has no angr, so this runs
``_angr_runner.py`` under $KUNA_PIPELINE_ANGR_PYTHON as a subprocess and parses its
JSON. Pins the angr version into every ReferenceResult for reproducibility.
"""
from __future__ import annotations

import json
import os
import subprocess

from .. import config
from .base import ReferenceDecompiler, ReferenceResult

_RUNNER = os.path.join(os.path.dirname(__file__), "_angr_runner.py")


def _extract_last_json(text):
    """Best-effort: pull the trailing JSON object out of mixed stdout."""
    end = text.rfind("}")
    if end == -1:
        return None
    depth = 0
    for i in range(end, -1, -1):
        ch = text[i]
        if ch == "}":
            depth += 1
        elif ch == "{":
            depth -= 1
            if depth == 0:
                try:
                    return json.loads(text[i:end + 1])
                except json.JSONDecodeError:
                    return None
    return None


class AngrReference(ReferenceDecompiler):
    name = "angr"

    def __init__(self, python=None, runner=None, timeout=None):
        self.python = python or config.angr_python()
        self.runner = runner or _RUNNER
        self.timeout = timeout or config.ANGR_TIMEOUT
        self._version = None

    def available(self) -> bool:
        try:
            r = subprocess.run(
                [self.python, "-c", "import angr"],
                capture_output=True, text=True, timeout=120,
            )
            return r.returncode == 0
        except Exception:
            return False

    def version(self):
        if self._version is None:
            try:
                r = subprocess.run(
                    [self.python, "-c", "import angr;print(angr.__version__)"],
                    capture_output=True, text=True, timeout=120,
                )
                self._version = (r.stdout or "").strip() or None
            except Exception:
                self._version = None
        return self._version

    def decompile(self, binary, selector, *, arch=None, load_debug=False,
                  structurer=None, timeout=None) -> ReferenceResult:
        binary = os.path.abspath(str(binary))
        selector = str(selector)
        res = ReferenceResult(source="angr", binary=binary, selector=selector)

        if not os.path.exists(binary):
            res.error = "binary not found: %s" % binary
            return res

        cmd = [self.python, self.runner, binary, selector]
        if arch:
            cmd += ["--arch", str(arch)]
        if load_debug:
            cmd += ["--load-debug"]
        if structurer:
            cmd += ["--structurer", str(structurer)]

        try:
            proc = subprocess.run(
                cmd, capture_output=True, text=True,
                timeout=timeout or self.timeout,
            )
        except subprocess.TimeoutExpired:
            res.error = "angr timed out after %ss" % (timeout or self.timeout)
            return res

        out = (proc.stdout or "").strip()
        if not out:
            res.error = "angr runner produced no stdout; stderr tail: %s" % (
                (proc.stderr or "")[-600:]
            )
            return res

        data = None
        try:
            data = json.loads(out)
        except json.JSONDecodeError:
            data = _extract_last_json(out)
        if data is None:
            res.error = "could not parse angr runner JSON: %s" % out[:600]
            return res

        res.text = data.get("text")
        res.version = data.get("version")
        res.func_name = data.get("func_name")
        res.func_addr = data.get("func_addr")
        res.error = data.get("error")
        res.meta = data
        return res
