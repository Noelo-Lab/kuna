#!/bin/bash
# after the main 4-arm run finishes, re-run the cases that exceeded +5% at a quieter moment
until grep -q SPEED_DONE /home/mahaloz/kwt/_final-f/speed.log; do sleep 30; done
sleep 60
~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-f/tools/speed6.py 15 /home/mahaloz/kwt/_final-f/speed-quiet.json fmt,ls
echo QUIET_DONE
