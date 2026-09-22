#!/bin/bash
set -u
export KUNA_DECBENCH_RESULTS=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
export DECBENCH_NO_CACHE=1
export SLEIGHHOME=/home/mahaloz/kwt/o2census/specs
export KUNA_SPECS=/home/mahaloz/kwt/o2census/specs
export KUNA_BIN=/home/mahaloz/kwt/o2census/decompiler/target/release/kuna
export KUNA_TREE=/home/mahaloz/kwt/o2census
export DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625
cd /home/mahaloz/kwt/o2census
P="--project coreutils --project grep --project gzip --project diffutils --project bzip2 --project findutils --project tar --project shadow"
PYTHONPATH=/home/mahaloz/kwt/o2census/.scratch/py ~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/o2census/.scratch/py/o2sweep.py $P "$@" --workers 12 --out ${OUT}
echo "SWEEP_RC=$?"
