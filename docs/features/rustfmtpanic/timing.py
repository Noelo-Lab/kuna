from pathlib import Path
import subprocess,time,resource,statistics,json,os,hashlib,sys
root=Path('/home/mahaloz/kwt/angr-rust-panic');base=Path('/home/mahaloz/kwt/angr-rust-signed-bytes');refs=Path('/tmp/kuna-angr-reference/binaries');out=Path('/tmp/kuna-rust-panic')
fixture=root/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_panic_real_x86_64'
rows=[('scalar-c',root/'tests/fixtures/go-output/output-c','choose','c'),('panic-literal',fixture,'panic_literal','rust'),('panic-integers',fixture,'panic_reorder','rust'),('rust-large-control',refs/'rust-fmtdemo-1931','0x13df0','rust'),('go-control',Path('/tmp/kuna-go-dwarf-signatures/native1273'),'main.main','go')]
if len(sys.argv)>1:rows=[row for row in rows if row[0]in sys.argv[1:]]
for name,binary,selector,language in rows:
 raw={k:[]for k in ['baseline','off','patched']}
 def run(side):
  checkout=base if side=='baseline'else root
  options=['--option','godwarfsig','on'] if name.startswith('go-') else []
  if side=='off':options += ['--option','rustfmtpanic','off']
  cmd=['taskset','-c','77',str(checkout/'decompiler/target/release/kuna'),'decompile',str(binary),selector,'--language',language]+options
  r=resource.getrusage(resource.RUSAGE_CHILDREN);cpu=r.ru_utime+r.ru_stime;t=time.perf_counter();subprocess.run(cmd,cwd=root,env=os.environ|{'KUNA_SPECS':str(root/'specs')},stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=True);r=resource.getrusage(resource.RUSAGE_CHILDREN)
  return {'wall_ms':(time.perf_counter()-t)*1000,'cpu_ms':(r.ru_utime+r.ru_stime-cpu)*1000}
 for side in raw:
  for _ in range(3):run(side)
 sides=list(raw)
 for i in range(31):
  order=sides[i%3:]+sides[:i%3]
  for side in order:raw[side].append(run(side))
 med={s:{m:statistics.median(x[m]for x in xs)for m in ['wall_ms','cpu_ms']}for s,xs in raw.items()};deltas={a+'_to_'+b:{m:100*(med[b][m]/med[a][m]-1)for m in med[a]}for a,b in [('baseline','patched'),('off','patched')]}
 result={'baseline_commit':'4cb0ed51b00327b20c40b279a0fa2ba31d94860f','fixture':name,'sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'selector':selector,'language':language,'samples':31,'warmups':3,'affinity_cpu':77,'medians':med,'delta_pct':deltas,'paired_median_delta_pct':{m:statistics.median(100*(b[m]/a[m]-1)for a,b in zip(raw['baseline'],raw['patched']))for m in ['wall_ms','cpu_ms']},'raw':raw};(out/f'timing-{name}.json').write_text(json.dumps(result,indent=2)+'\n');print(name,med,deltas,flush=True)
