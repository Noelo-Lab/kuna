#!/bin/bash
# structscore --all for the campaign baseline binary on this round's instrument
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
cd /home/mahaloz/kwt/_final-main
export KUNA_BIN=/home/mahaloz/kwt/_baseline/kuna
for opt in O0 O2; do for b in fmt ls sort du; do
  ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-i/tools/ssrun.py "$R/$opt/coreutils/stripped/$b" --all \
    --out /home/mahaloz/kwt/_final-i/ss/base-$opt-$b.json > /home/mahaloz/kwt/_final-i/ss/base-$opt-$b.log 2>&1 &
done; done
wait
echo SSBASE_DONE
