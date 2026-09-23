#!/bin/bash
# structscore --all (TRex + layout + nesting) for round E and round F binaries; instrument byte-identical E->F
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
mkdir -p /home/mahaloz/kwt/_final-f/ss
cd /home/mahaloz/kwt/_final-main
for arm in f e; do
  if [ $arm = f ]; then export KUNA_BIN=/home/mahaloz/kwt/_final-f/kuna; else export KUNA_BIN=/home/mahaloz/kwt/_final-e/kuna; fi
  for opt in O0 O2; do for b in fmt ls sort du; do
    ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-f/tools/ssrun.py "$R/$opt/coreutils/stripped/$b" --all \
      --out /home/mahaloz/kwt/_final-f/ss/$arm-$opt-$b.json > /home/mahaloz/kwt/_final-f/ss/$arm-$opt-$b.log 2>&1 &
  done; done
  wait
  echo "done arm $arm"
done
echo SS_DONE
