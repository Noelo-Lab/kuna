from common import *
from concurrent.futures import ThreadPoolExecutor
while not(w/'build.json').exists():time.sleep(2)
before=json.loads(subprocess.check_output(['git','show','c5ea0fa96b83ac92b140e83017f57cff2d298e4c:docs/baseline-stages.json'],cwd=r,text=True));after=json.loads((r/'docs/baseline-stages.json').read_text());assert set(before['passing'])<=set(after['passing']);assert after['data_footer'][0]==before['data_footer'][0]+14 and after['data_footer'][0]==after['data_footer'][1]
with ThreadPoolExecutor(max_workers=2)as pool:
 fs=[pool.submit(run,r,w,['python3',str(w/name)],log) for name,log in [('validate.py','validation-coordinator.log'),('breadth.py','breadth-coordinator.log')]]
 for f in fs:f.result()
run(r,w,['python3',str(w/'final-captures.py')],'final-captures.log')
run(r,w,['python3',str(w/'review.py')],'review.log')
print('INTEGRATION C5 VALIDATION AND BREADTH COMPLETE',flush=True)
