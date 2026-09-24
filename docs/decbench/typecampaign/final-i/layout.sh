#!/bin/bash
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
cd /home/mahaloz/kwt/_final-main
echo "=== FINAL (ROUND I) DEFAULT"
KUNA_BIN=/home/mahaloz/kwt/_final-i/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/layoutrun.py
echo "RC_I=$?"
echo "=== ROUND G DEFAULT (control, this tree's instrument)"
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/layoutrun.py
echo "RC_G=$?"
echo "=== ROUND H (dbe854ba3) DEFAULT"
KUNA_BIN=/home/mahaloz/kwt/_final-i/bh/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/layoutrun.py
echo "RC_H=$?"
echo "=== FINAL with structmerge siblings (the round-H opt-in)"
LAYOUTSCORE_OPTIONS="structmerge siblings" KUNA_BIN=/home/mahaloz/kwt/_final-i/kuna ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/layoutrun.py
echo "RC_SM=$?"
echo "=== NESTING (final build, 8 builds, default vs nest)"
KUNA_BIN=/home/mahaloz/kwt/_final-i/kuna W=8 BINS="coreutils:fmt:O0,coreutils:fmt:O2,coreutils:ls:O0,coreutils:ls:O2,coreutils:sort:O0,coreutils:sort:O2,coreutils:du:O0,coreutils:du:O2" \
  PINDB_DIR=/home/mahaloz/kwt/_final-i/tools ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-main/docs/features/structnest/nestscore.py
echo "RC_NEST=$?"
echo LAYOUT_DONE
