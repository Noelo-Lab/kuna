#!/bin/bash
# goal 2 (varcensus), structscore, then layout: round-I control vs final (round L)
set -u
L=/home/mahaloz/kwt/_final-l
cd $L
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
~/.virtualenvs/decbench/bin/python $L/goal2l.py > $L/goal2.log 2>&1 &
bash $L/ss.sh > $L/ss.log 2>&1 &
wait
bash $L/layout.sh > $L/layout.log 2>&1
echo PHASE2_DONE
