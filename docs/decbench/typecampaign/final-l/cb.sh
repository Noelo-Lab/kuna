#!/bin/bash
# castbench full for one binary, options optional: cb.sh <arm> <kuna> [NAME VALUE ...]
set -u
export SLEIGHHOME=/home/mahaloz/kwt/_final-main/specs
export KUNA_SPECS=/home/mahaloz/kwt/_final-main/specs
B=/home/mahaloz/kwt/castbench
L=/home/mahaloz/kwt/_final-l
arm=$1; bin=$2; shift 2
O=()
while [ $# -ge 2 ]; do O+=(--option "$1" "$2"); shift 2; done
python3 $B/castbench.py run --kuna $bin --out $L/cb-$arm "${O[@]}" --workers 12 > $L/cb-$arm.log 2>&1
echo "CB_RC=$?" >> $L/cb-$arm.log
