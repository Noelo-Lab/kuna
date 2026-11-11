from pathlib import Path
import os, sys, json, time, threading, subprocess, runpy, statistics, shutil

w = Path('/tmp/kuna-rust-panic-d850')
cpu=int(os.environ.get('KUNA_TIMING_CPU','77'))
siblings=list(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
assert len(siblings)==2 and cpu in siblings
sibling=next(c for c in siblings if c!=cpu)
runpy.run_path(str(w/'audit-idle.py'), run_name='__main__')
attempt = w/'timing-attempts'/time.strftime('%Y%m%d-%H%M%S', time.gmtime())
attempt.mkdir(parents=True)
if (w/'baseline-commit.txt').exists():
    shutil.copy2(w/'baseline-commit.txt', attempt/'baseline-commit.txt')
shutil.copy2(w/'cpu-audit.json', attempt/'cpu-audit.json')
samples = []
done = threading.Event()

def sample():
    rows = {}
    for line in Path('/proc/stat').read_text().splitlines():
        parts = line.split()
        if parts and parts[0] in [f'cpu{cpu}', f'cpu{sibling}']:
            rows[parts[0]] = list(map(int, parts[1:]))
    return {'time': time.monotonic(), 'cpus': rows}

def monitor():
    while not done.wait(0.25):
        samples.append(sample())

samples.append(sample())
thread = threading.Thread(target=monitor)
thread.start()
try:
    with (attempt/'timing.log').open('w') as log:
        result = subprocess.run(['python3', str(w/'timing.py')]+sys.argv[1:], env=os.environ|{'KUNA_TIMING_OUT': str(attempt)}, stdout=log, stderr=subprocess.STDOUT)
finally:
    done.set()
    thread.join()
    samples.append(sample())
    (attempt/'cpu-samples.json').write_text(json.dumps(samples, indent=2)+'\n')
assert result.returncode == 0, (result.returncode, attempt)
a, b = samples[0]['cpus'][f'cpu{sibling}'], samples[-1]['cpus'][f'cpu{sibling}']
sibling_idle = 100*(b[3]-a[3])/sum(y-x for x, y in zip(a, b))
records = [json.loads(p.read_text()) for p in sorted(attempt.glob('timing-*.json'))]
cv = {r['fixture']: {arm: statistics.stdev(x['cpu_ms'] for x in rows)/statistics.mean(x['cpu_ms'] for x in rows) for arm, rows in r['raw'].items()} for r in records}
accepted = bool(records) and sibling_idle >= 95 and all(v <= 0.10 for arms in cv.values() for v in arms.values())
record = {'attempt': str(attempt), 'target_cpu': cpu, 'sibling_cpu': sibling, 'sibling_idle_pct': sibling_idle, 'cpu_coefficient_of_variation': cv, 'criteria': {'sibling_idle_min_pct': 95, 'cpu_coefficient_of_variation_max': 0.10}, 'accepted_for_speed_review': accepted}
(attempt/'monitor.json').write_text(json.dumps(record, indent=2)+'\n')
print(json.dumps(record, indent=2), flush=True)
assert accepted, 'Measurement contention remains; samples retained without replacing the canonical timings.'
for p in attempt.glob('timing-*.json'):
    shutil.copy2(p, w/p.name)
shutil.copy2(attempt/'monitor.json', w/'timing-monitor.json')
