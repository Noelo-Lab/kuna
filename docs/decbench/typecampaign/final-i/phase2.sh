#!/bin/bash
set -u
I=/home/mahaloz/kwt/_final-i
cd $I
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
export DECBENCH_NO_CACHE=1
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
export KUNA_TREE=/home/mahaloz/kwt/_final-main
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow --opt O0 --opt O2 --opt O2-noinline"
( cd /home/mahaloz/kwt/_final-main && PYTHONPATH=$I/tools KUNA_BIN=$I/kuna SWEEP_OPTS="castarith off castimplied off castsign off" \
  ~/.virtualenvs/decbench/bin/python -m finalsweepopt kuna $P --workers 12 --out $I/sweep-castoff > $I/sweep-castoff.log 2>&1 ) &
bash $I/ss.sh > $I/ss.log 2>&1 &
~/.virtualenvs/decbench/bin/python $I/tools/goal2i.py > $I/goal2.log 2>&1 &
( ./cbopt.sh castoff castarith off castimplied off castsign off; ./cbopt.sh noarith castarith off; ./cbopt.sh noimplied castimplied off; ./cbopt.sh nosign castsign off ) &
wait
bash $I/layout.sh > $I/layout.log 2>&1
echo PHASE2_DONE
