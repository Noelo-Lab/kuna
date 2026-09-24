#!/bin/bash
set -u
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
B=/home/mahaloz/kwt/castbench
arm=$1; bin=$2
python3 $B/castbench.py run --kuna $bin --out /home/mahaloz/kwt/_final-i/cb-$arm --workers 12 > /home/mahaloz/kwt/_final-i/cb-$arm.log 2>&1
echo "CB_RC=$?" >> /home/mahaloz/kwt/_final-i/cb-$arm.log
