#!/bin/bash
# rounds J+K+L sweep (pinned metric): final main (L) + round-I control + round-K and round-J builds,
# plus the round-L ladder as option arms on the final build (landing order #718, #729, #743, #744, #731)
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
L=/home/mahaloz/kwt/_final-l
PY=~/.virtualenvs/decbench/bin/python
CB=/home/mahaloz/kwt/castbench
arm() { # arm <name> <kuna> [SWEEP_OPTS]
  SWEEP_OPTS="${3:-}" KUNA_BIN=$2 $PY -m finalsweepopt kuna $P --workers 12 --out $L/sweep-$1 > $L/sweep-$1.log 2>&1
}
arm l $L/kuna &
arm ictl /home/mahaloz/kwt/_final-i/kuna &
arm k $CB/bin-0096e984d/kuna &
arm l0 $L/kuna "callbacktype off callrettype off castobject off castwiden off elemptr off" &
wait
echo WAVE1_DONE
arm j $CB/bin-c960fb18d/kuna &
arm l1 $L/kuna "callrettype off castobject off castwiden off elemptr off" &
arm l2 $L/kuna "castobject off castwiden off elemptr off" &
arm l3 $L/kuna "castwiden off elemptr off" &
arm l4 $L/kuna "elemptr off" &
wait
echo ALLSWEEPS_DONE
