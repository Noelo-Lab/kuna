#!/bin/bash
set -u
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
export DECBENCH_NO_CACHE=1
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export KUNA_TREE=/home/mahaloz/kwt/_final-main
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
export PYTHONPATH=/home/mahaloz/kwt/_final-g/tools
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow --opt O0 --opt O2 --opt O2-noinline"
cd /home/mahaloz/kwt/_final-main
# option ablations on the round-G build
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna SWEEP_OPTS="calleevote off" ~/.virtualenvs/decbench/bin/python -m finalsweepopt kuna $P --workers 12 --out /home/mahaloz/kwt/_final-g/sweep-nocv \
  > /home/mahaloz/kwt/_final-g/sweep-nocv.log 2>&1 &
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna SWEEP_OPTS="passthrough off" ~/.virtualenvs/decbench/bin/python -m finalsweepopt kuna $P --workers 12 --out /home/mahaloz/kwt/_final-g/sweep-nopt \
  > /home/mahaloz/kwt/_final-g/sweep-nopt.log 2>&1 &
# per-commit steps: round F + #711, then + #714
KUNA_BIN=/home/mahaloz/kwt/_final-g/b711/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out /home/mahaloz/kwt/_final-g/sweep-s711 \
  > /home/mahaloz/kwt/_final-g/sweep-s711.log 2>&1 &
KUNA_BIN=/home/mahaloz/kwt/_final-g/b714/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out /home/mahaloz/kwt/_final-g/sweep-s714 \
  > /home/mahaloz/kwt/_final-g/sweep-s714.log 2>&1 &
wait
echo ARMSWEEPS_DONE
