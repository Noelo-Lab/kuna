from common import *
from concurrent.futures import ThreadPoolExecutor
base=Path('/home/mahaloz/kwt/angr-base-c1745')
with ThreadPoolExecutor(max_workers=2)as pool:
 jobs=[pool.submit(run,base,w,['python3',str(w/'prepare-parent.py')],'parent-coordinator.log'),pool.submit(run,r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'candidate-build-initial.log',r/'decompiler')]
 for job in jobs:job.result()
run(r,w,['python3',str(w/'prepare.py')],'prepare-coordinator.log')
