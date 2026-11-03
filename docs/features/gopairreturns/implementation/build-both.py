from common import *
from concurrent.futures import ThreadPoolExecutor
base=Path('/home/mahaloz/kwt/angr-go-pair-parent-460')
with ThreadPoolExecutor(max_workers=2)as pool:
 fs=[pool.submit(run,r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'candidate-build.log',r/'decompiler'),pool.submit(run,base,w,['cargo','build','--release','-p','kuna-cli'],'parent-build.log',base/'decompiler')]
 for f in fs:f.result()
print('BOTH A324 RELEASE BUILDS PASSED',flush=True)
