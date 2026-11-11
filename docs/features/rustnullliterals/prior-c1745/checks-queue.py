from common import *
from concurrent.futures import ThreadPoolExecutor
assert (w/'build.json').exists()
with ThreadPoolExecutor(max_workers=3)as pool:
 fs=[pool.submit(run,r,w,['python3',str(w/script)],log)for script,log in [('validate.py','validation-coordinator.log'),('breadth.py','breadth-coordinator.log'),('native-proof.py','native-coordinator.log')]]
 for f in fs:f.result()
print('RUST NULL POINTER CORRECTNESS COMPLETE; FULL REVIEW AND SPEED PENDING',flush=True)
