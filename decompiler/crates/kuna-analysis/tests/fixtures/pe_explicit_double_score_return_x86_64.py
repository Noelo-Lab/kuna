#!/usr/bin/env python3
"""Generate the refined-XMM PE for `explicit-double-score-prototype`.

This selects the faithful refinement variant of the shared PolyMLP score
fixture: the returned double crosses the MSVC cookie checker as adjacent
XMM0_Da/XMM0_Db cells, and `score` is exported for the promoted CLI probe.
"""
import os

import pe_double_score_return_x86_64 as base


if __name__ == "__main__":
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "pe_explicit_double_score_return_x86_64.exe")
    with open(out, "wb") as f:
        f.write(base.build(refined=True))
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
