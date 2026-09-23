#!/bin/bash
# After the main speed run: the calleevote layout ablation, then the quiet re-run of the two
# binaries that crossed +5%.
until grep -q ALLSPEED_DONE /home/mahaloz/kwt/_final-g/speed-all.log 2>/dev/null; do sleep 30; done
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
cd /home/mahaloz/kwt/_final-main
echo "=== LAYOUT with calleevote off"
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna LAYOUTSCORE_OPTIONS="calleevote off" \
  ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-g/tools/layoutrun.py
echo "LAYOUT_ABL_DONE"
sleep 90
~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-g/tools/speedextra.py 15 \
  /home/mahaloz/kwt/_final-g/speed-quiet.json kmod-O2ni,dpkg-divert-O2ni
echo FOLLOWUP_DONE
