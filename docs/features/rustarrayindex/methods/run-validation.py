from common import *
from concurrent.futures import ThreadPoolExecutor
assert json.loads((w/'build.json').read_text())['stage_assertions']==[3794,3794]
assert json.loads((w/'native-record.json').read_text())['status']=='PASS'
with ThreadPoolExecutor(max_workers=2)as pool:
 jobs=[pool.submit(run,r,w,['python3',str(w/script)],script.removesuffix('.py')+'-coordinator.log')for script in ['validate.py','breadth.py']]
 for job in jobs:job.result()
print('Array gates and breadth complete; structural review pending',flush=True)
