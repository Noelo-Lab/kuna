from common import *
from concurrent.futures import ThreadPoolExecutor
assert json.loads((w/'build.json').read_text())['stage_assertions']==[3811,3811]
assert json.loads((w/'native-record.json').read_text())['status']=='PASS'
with ThreadPoolExecutor(max_workers=3)as pool:
 jobs=[pool.submit(run,r,w,['python3',str(w/script)],script.removesuffix('.py')+'-coordinator.log')for script in ['validate.py','breadth.py','audit-live.py']]
 for job in jobs:job.result()
run(r,w,['python3',str(w/'review.py')],'structural-review.log')
print('Array decay gates and breadth/review complete',flush=True)
