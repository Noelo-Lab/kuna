#!/bin/bash
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
cd /home/mahaloz/kwt/_final-main
echo "=== ROUND E DEFAULT"
KUNA_BIN=/home/mahaloz/kwt/_final-e/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-e/tools/layoutrun.py
echo "RC_E=$?"
echo "=== ROUND D DEFAULT (control, this tree's instrument)"
KUNA_BIN=/home/mahaloz/kwt/_final-d/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-e/tools/layoutrun.py
echo "RC_D=$?"
echo "=== NEST param vs nest (round E build, 8 builds)"
KUNA_BIN=/home/mahaloz/kwt/_final-e/kuna W=8 BINS="coreutils:fmt:O0,coreutils:fmt:O2,coreutils:ls:O0,coreutils:ls:O2,coreutils:sort:O0,coreutils:sort:O2,coreutils:du:O0,coreutils:du:O2" \
  PINDB_DIR=/home/mahaloz/kwt/_final-e/tools ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-main/docs/features/structnest/nestscore.py
echo "RC_NEST=$?"
echo LAYOUT_DONE
