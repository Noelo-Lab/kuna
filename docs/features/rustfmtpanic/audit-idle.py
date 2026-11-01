from pathlib import Path
import os,json,time
w=Path('/tmp/kuna-rust-panic-d850')
owned=['/home/mahaloz/kwt/angr-rust-panic-main','/home/mahaloz/kwt/angr-rust-extensions','/home/mahaloz/kwt/angr-rust-base-c281','/home/mahaloz/kwt/angr-rust-userop','/home/mahaloz/kwt/angr-rust-panic-base-d850','/tmp/kuna-rust-extensions-c281-review','/tmp/kuna-rust-userop','/tmp/kuna-rust-panic-d850']
owned += ['/home/mahaloz/kwt/angr-rust-fmt-str','/tmp/kuna-rust-fmt-str','/tmp/kuna-rust-symbols','/home/mahaloz/kwt/angr-rust-fmt-pieces','/home/mahaloz/kwt/angr-rust-panic','/home/mahaloz/kwt/angr-rust-signed-bytes','/tmp/kuna-rust-fmt-pieces','/tmp/kuna-rust-panic']
bench_cpu=int(os.environ.get('KUNA_TIMING_CPU','77'))
bench_siblings=list(map(int,Path(f'/sys/devices/system/cpu/cpu{bench_cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
processes=[]
for p in Path('/proc').iterdir():
 if not p.name.isdigit() or int(p.name)==os.getpid():continue
 try:args=(p/'cmdline').read_bytes().decode(errors='replace').split('\0');exe=Path(args[0]).name
 except (OSError,IndexError):continue
 if exe in ['bash','timeout','rg'] or not any(x in ' '.join(args) for x in owned):continue
 if True:
  processes.append({'pid':int(p.name),'arguments':args})
(w/'owned-process-audit.json').write_text(json.dumps(processes,indent=2)+'\n');assert not processes,processes

def sample():
 return {int(parts[0][3:]):list(map(int,parts[1:])) for line in Path('/proc/stat').read_text().splitlines() if (parts:=line.split()) and parts[0] in [f'cpu{x}'for x in bench_siblings]}
a=sample();time.sleep(5);b=sample();result={}
for cpu in a:
 delta=[y-x for x,y in zip(a[cpu],b[cpu])];result[cpu]={'idle_pct':100*delta[3]/sum(delta),'deltas':delta}
(w/'cpu-audit.json').write_text(json.dumps(result,indent=2)+'\n');assert min(result[c]['idle_pct']for c in bench_siblings)>=float(os.environ.get('KUNA_TIMING_MIN_IDLE','95')),result
print(json.dumps(result))
