#!/bin/bash
# rounds H+I sweep (pinned metric): final main + round H build + round G control + round C control
set -u
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
export DECBENCH_NO_CACHE=1
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export KUNA_TREE=/home/mahaloz/kwt/_final-main
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
export PYTHONPATH=/home/mahaloz/kwt/_final-i/tools
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow --opt O0 --opt O2 --opt O2-noinline"
cd /home/mahaloz/kwt/_final-main
I=/home/mahaloz/kwt/_final-i
KUNA_BIN=$I/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out $I/sweep-i > $I/sweep-i.log 2>&1 &
KUNA_BIN=$I/bh/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out $I/sweep-h > $I/sweep-h.log 2>&1 &
KUNA_BIN=/home/mahaloz/kwt/_final-g/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out $I/sweep-gctl > $I/sweep-gctl.log 2>&1 &
KUNA_BIN=/home/mahaloz/kwt/_final-c/kuna ~/.virtualenvs/decbench/bin/python -m finalsweep kuna $P --workers 12 --out $I/sweep-cctl > $I/sweep-cctl.log 2>&1 &
wait
echo ALLSWEEPS_DONE
