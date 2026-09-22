#!/bin/bash
# usage: sweep.sh <arm> [<arm>...]  -> .scratch/sweep-<arm>/rows.json
set -u
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
export DECBENCH_NO_CACHE=1
export SLEIGHHOME=/home/mahaloz/kwt/slotptr/specs
export KUNA_SPECS=/home/mahaloz/kwt/slotptr/specs
export KUNA_TREE=/home/mahaloz/kwt/slotptr
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
export PYTHONPATH=/home/mahaloz/kwt/_final-e/tools
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow --opt O0 --opt O2 --opt O2-noinline"
cd /home/mahaloz/kwt/slotptr
for arm in "$@"; do
  KUNA_BIN=/home/mahaloz/kwt/slotptr/.scratch/bin/kuna-$arm ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out .scratch/sweep-$arm > .scratch/sweep-$arm.log 2>&1 &
done
wait
echo ALLSWEEPS_DONE
