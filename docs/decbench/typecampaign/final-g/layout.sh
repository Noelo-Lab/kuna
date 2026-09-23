#!/bin/bash
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
cd /home/mahaloz/kwt/_final-main
echo "=== ROUND G DEFAULT"
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-g/tools/layoutrun.py
echo "RC_G=$?"
echo "=== ROUND F DEFAULT (control, this tree's instrument)"
KUNA_BIN=/home/mahaloz/kwt/_final-f/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-g/tools/layoutrun.py
echo "RC_F=$?"
echo "=== NESTING (round G build, 8 builds, default vs nest)"
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna W=8 BINS="coreutils:fmt:O0,coreutils:fmt:O2,coreutils:ls:O0,coreutils:ls:O2,coreutils:sort:O0,coreutils:sort:O2,coreutils:du:O0,coreutils:du:O2" \
  PINDB_DIR=/home/mahaloz/kwt/_final-g/tools ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-main/docs/features/structnest/nestscore.py
echo "RC_NEST=$?"
echo LAYOUT_DONE
