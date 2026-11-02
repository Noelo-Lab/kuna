from common import *
from concurrent.futures import ThreadPoolExecutor
assert (w/'build.json').exists()
with ThreadPoolExecutor(max_workers=5)as pool:
 fs=[pool.submit(run,r,w,['python3',str(w/script)],log)for script,log in [('validate.py','validation-coordinator.log'),('breadth-off.py','breadth-off-coordinator.log'),('breadth-on.py','breadth-on-coordinator.log'),('native-proof.py','native-coordinator.log'),('combined-native.py','combined-native-coordinator.log')]]
 for f in fs:f.result()
run(r,w,['python3',str(w/'language-native.py')],'language-native-coordinator.log')
print('HIGH-SLICE OPT-IN CORRECTNESS COMPLETE;FULL REVIEW AND DEFAULT-OFF SPEED PENDING',flush=True)
