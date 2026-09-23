#!/bin/bash
# round-G sweep (pinned metric): round G default + round F control + round C control
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
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out /home/mahaloz/kwt/_final-g/sweep-g \
  > /home/mahaloz/kwt/_final-g/sweep-g.log 2>&1 &
KUNA_BIN=/home/mahaloz/kwt/_final-f/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out /home/mahaloz/kwt/_final-g/sweep-fctl \
  > /home/mahaloz/kwt/_final-g/sweep-fctl.log 2>&1 &
KUNA_BIN=/home/mahaloz/kwt/_final-c/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out /home/mahaloz/kwt/_final-g/sweep-cctl \
  > /home/mahaloz/kwt/_final-g/sweep-cctl.log 2>&1 &
wait
echo ALLSWEEPS_DONE
