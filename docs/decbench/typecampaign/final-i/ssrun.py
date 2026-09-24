"""structscore under the campaign's pinned decbench checkout (decisions.md #11)."""
import runpy
import sys

sys.path.insert(0, "/home/mahaloz/kwt/_final-c/tools")
sys.path.insert(0, "/home/mahaloz/kwt/_final-main")

import pindb  # noqa: E402

pindb.pin()

sys.argv = ["structscore"] + sys.argv[1:]
runpy.run_module("scripts.decbench.structscore", run_name="__main__")
