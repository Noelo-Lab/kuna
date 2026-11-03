from common import *
from concurrent.futures import ThreadPoolExecutor
assert(w/'build.json').exists()
with ThreadPoolExecutor(max_workers=2)as pool:
 fs=[pool.submit(run,r,w,['python3',str(w/'validate.py')],'validation-coordinator.log'),pool.submit(run,r,w,['python3',str(w/'breadth.py')],'breadth-coordinator.log')]
 for f in fs:f.result()
run(r,w,['python3',str(w/'review.py')],'review-coordinator.log')
print('CURRENT MAIN FULL GATES AND BREADTH REVIEW COMPLETE',flush=True)
