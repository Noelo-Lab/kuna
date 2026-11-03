from common import *
for script in ['native-go.py','native-c.py','native-extra.py','native-extra-run.py','native-raw.py','public-repro.py']:
 run(r,w,['python3',str(w/script)],script.removesuffix('.py')+'-coordinator.log')
print('CURRENT MAIN NATIVE PROOFS COMPLETE',flush=True)
