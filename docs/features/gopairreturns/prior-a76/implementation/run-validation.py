from common import *
from concurrent.futures import ThreadPoolExecutor
run(r,w,['python3',str(w/'build-focused.py')],'build-coordinator.log')
run(r,w,['python3',str(w/'prepare.py')],'prepare-coordinator.log')
def natives():
 for script in ['native-go.py','native-c.py','native-extra.py','native-extra-run.py','native-raw.py']:
  run(r,w,['python3',str(w/script)],script.removesuffix('.py')+'-coordinator.log')
with ThreadPoolExecutor(max_workers=3)as pool:
 fs=[pool.submit(run,r,w,['python3',str(w/'validate.py')],'validation-coordinator.log'),pool.submit(run,r,w,['python3',str(w/'breadth.py')],'breadth-coordinator.log'),pool.submit(natives)]
 for f in fs:f.result()
run(r,w,['python3',str(w/'review.py')],'review-coordinator.log')
print('ROUND3 VALIDATION COMPLETE; timing not started',flush=True)
