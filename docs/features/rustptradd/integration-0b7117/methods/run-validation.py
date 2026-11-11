from common import *
from concurrent.futures import ThreadPoolExecutor
for script in ['prepare.py','native.py','extra-casts.py','extra-memory.py','controls.py','public-repro.py']:
 run(r,w,['python3',str(w/script)],script.removesuffix('.py')+'-coordinator.log')
with ThreadPoolExecutor(max_workers=2)as pool:
 jobs=[pool.submit(run,r,w,['python3',str(w/script)],script.removesuffix('.py')+'-coordinator.log')for script in ['validate.py','breadth.py']]
 for job in jobs:job.result()
print('Integrated native, gates and breadth complete; structural review pending',flush=True)
