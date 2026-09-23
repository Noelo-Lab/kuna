#!/bin/bash
set -u
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
export DECBENCH_NO_CACHE=1 SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export KUNA_TREE=/home/mahaloz/kwt/_final-main DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
export PYTHONPATH=/home/mahaloz/kwt/_final-f/tools KUNA_BIN=/home/mahaloz/kwt/_final-f/kuna
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow --opt O0 --opt O2 --opt O2-noinline"
cd /home/mahaloz/kwt/_final-main
SWEEP_OPTS="structsynth param" ~/.virtualenvs/decbench/bin/python -m finalsweepopt kuna $P --workers 12 --out /home/mahaloz/kwt/_final-f/sweep-ssparam \
  > /home/mahaloz/kwt/_final-f/sweep-ssparam.log 2>&1 &
SWEEP_OPTS="protoorder types" ~/.virtualenvs/decbench/bin/python -m finalsweepopt kuna $P --workers 12 --out /home/mahaloz/kwt/_final-f/sweep-protypes \
  > /home/mahaloz/kwt/_final-f/sweep-protypes.log 2>&1 &
wait
echo ARM2_DONE
