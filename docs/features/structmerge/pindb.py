"""Pin the `decbench` package to the campaign's checkout (decisions.md #11).

The venv installs decbench editable through a PEP-660 meta-path finder, which
outranks sys.path, so the pin rewrites that finder's MAPPING entry instead.
"""
import os
import sys


def pin():
    p = os.environ.get("DECBENCH_PIN")
    if not p:
        return
    assert "decbench" not in sys.modules, "pin before importing decbench"
    import __editable___decbench_0_1_0_finder as F
    F.MAPPING["decbench"] = os.path.join(p, "decbench")
    sys.path.insert(0, p)
