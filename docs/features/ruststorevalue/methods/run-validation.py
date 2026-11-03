from common import *
from concurrent.futures import ThreadPoolExecutor

run(r, w, ['python3', str(w / 'prepare.py')], 'prepare-coordinator.log')
run(r, w, ['python3', str(w / 'native.py')], 'native-coordinator.log')
with ThreadPoolExecutor(max_workers=2) as pool:
    futures = [pool.submit(run, r, w, ['python3', str(w / script)], script.removesuffix('.py') + '-coordinator.log') for script in ['validate.py', 'breadth.py']]
    for future in futures: future.result()
print("Rust STORE gates, native and breadth finished; structural review pending", flush=True)
