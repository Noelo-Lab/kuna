#!/bin/bash
# every castbench arm of rounds J-L: final, the round-L ladder and ablations on the final build,
# the round-K ladder on the round-K build, a round-K control, and the opt-in structheadless arm
set -u
L=/home/mahaloz/kwt/_final-l
K=/home/mahaloz/kwt/castbench/bin-0096e984d/kuna
cd $L
./cb.sh l $L/kuna
./cb.sh l0 $L/kuna callbacktype off callrettype off castobject off castwiden off elemptr off
./cb.sh l1 $L/kuna callrettype off castobject off castwiden off elemptr off
./cb.sh l2 $L/kuna castobject off castwiden off elemptr off
./cb.sh l3 $L/kuna castwiden off elemptr off
./cb.sh l4 $L/kuna elemptr off
for o in callbacktype callrettype castobject castwiden elemptr; do ./cb.sh no$o $L/kuna $o off; done
./cb.sh kctl $K
./cb.sh k0 $K castindex off castternary off callpush off
./cb.sh k1 $K castternary off callpush off
./cb.sh k2 $K callpush off
./cb.sh shl $L/kuna structheadless closed
./cb.sh castwidenon $L/kuna castwiden on
echo CBALL_DONE
