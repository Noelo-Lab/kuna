#!/bin/bash
# structmerge siblings (the round-H opt-in) on the final build: typesweep + castbench
set -u
I=/home/mahaloz/kwt/_final-i
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
export DECBENCH_NO_CACHE=1
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export KUNA_TREE=/home/mahaloz/kwt/_final-main
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow --opt O0 --opt O2 --opt O2-noinline"
( cd /home/mahaloz/kwt/_final-main && PYTHONPATH=$I/tools KUNA_BIN=$I/kuna SWEEP_OPTS="structmerge siblings" \
  ~/.virtualenvs/decbench/bin/python -m finalsweepopt kuna $P --workers 12 --out $I/sweep-sm > $I/sweep-sm.log 2>&1 ) &
$I/cbopt.sh sm structmerge siblings &
wait
echo SM_DONE
