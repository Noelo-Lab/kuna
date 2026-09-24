#!/bin/bash
# castbench arm on the final build with option flips: cbopt.sh <arm> NAME VALUE [NAME VALUE ...]
set -u
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
B=/home/mahaloz/kwt/castbench
arm=$1; shift
O=()
while [ $# -ge 2 ]; do O+=(--option "$1" "$2"); shift 2; done
python3 $B/castbench.py run --kuna /home/mahaloz/kwt/_final-i/kuna --out /home/mahaloz/kwt/_final-i/cb-$arm "${O[@]}" --workers 12 > /home/mahaloz/kwt/_final-i/cb-$arm.log 2>&1
echo "CB_RC=$?" >> /home/mahaloz/kwt/_final-i/cb-$arm.log
