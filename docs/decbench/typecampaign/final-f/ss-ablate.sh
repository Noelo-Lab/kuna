#!/bin/bash
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625 KUNA_BIN=/home/mahaloz/kwt/_final-f/kuna
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
mkdir -p /home/mahaloz/kwt/_final-f/ss
cd /home/mahaloz/kwt/_final-main
for opt in O0 O2; do for b in fmt ls sort du; do
  ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-f/tools/ssrun.py "$R/$opt/coreutils/stripped/$b" --all --option slotptr off \
    --out /home/mahaloz/kwt/_final-f/ss/nosp-$opt-$b.json > /home/mahaloz/kwt/_final-f/ss/nosp-$opt-$b.log 2>&1 &
done; done
wait
echo SSABL_DONE
