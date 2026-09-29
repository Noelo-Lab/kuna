#!/bin/bash
# per-parameter layout P/R: final (round L), round-I control, final with structheadless closed; nesting on the final build
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
cd /home/mahaloz/kwt/_final-main
echo "=== FINAL (ROUND L) DEFAULT"
KUNA_BIN=/home/mahaloz/kwt/_final-l/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/layoutrun.py
echo "RC_L=$?"
echo "=== ROUND I DEFAULT (control, this tree's instrument)"
KUNA_BIN=/home/mahaloz/kwt/_final-i/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/layoutrun.py
echo "RC_I=$?"
echo "=== FINAL with structheadless closed (the round-K opt-in)"
LAYOUTSCORE_OPTIONS="structheadless closed" KUNA_BIN=/home/mahaloz/kwt/_final-l/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/layoutrun.py
echo "RC_SHL=$?"
echo "=== NESTING (final build, 8 builds)"
KUNA_BIN=/home/mahaloz/kwt/_final-l/kuna W=8 BINS="coreutils:fmt:O0,coreutils:fmt:O2,coreutils:ls:O0,coreutils:ls:O2,coreutils:sort:O0,coreutils:sort:O2,coreutils:du:O0,coreutils:du:O2" \
  PINDB_DIR=/home/mahaloz/kwt/_final-i/tools ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-main/docs/features/structnest/nestscore.py
echo "RC_NEST=$?"
echo LAYOUT_DONE
