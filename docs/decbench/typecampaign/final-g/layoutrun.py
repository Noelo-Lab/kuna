"""layoutscore.py under the campaign's pinned decbench checkout."""
import runpy
import sys

sys.path.insert(0, "/home/mahaloz/kwt/_final-g/tools")
sys.path.insert(0, "/home/mahaloz/kwt/_final-main")

import pindb  # noqa: E402

pindb.pin()

runpy.run_path("/home/mahaloz/kwt/_final-main/docs/features/structsynth/layoutscore.py",
               run_name="__main__")
