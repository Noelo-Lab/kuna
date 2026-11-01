from pathlib import Path
import subprocess,json,concurrent.futures,hashlib
w=Path('/tmp/kuna-rust-panic');out=w/'type-audit';out.mkdir(exist_ok=True)
breadth=json.loads((w/'breadth/record.json').read_text())
tasks=[]
for fixture in breadth['fixtures']:
 for lang in fixture['languages']:
  if lang['language']!='rust' or not lang['changed']:continue
  name=fixture['fixture'];prior=Path('/tmp/kuna-rust-signed-bytes/breadth')/f'{name}-rust.json'
  if not prior.exists():prior=w/'breadth'/f'{name}-rust-before.json'
  before={v['address']:v for v in json.loads(prior.read_text())['functions']};after={v['address']:v for v in json.loads((w/'breadth'/f'{name}-rust.json').read_text())['functions']}
  for item in lang['changed']:
   address=int(item['address'],16);tasks.append((name,item,before[address]['code'],after[address]['code']))
codes={'E0308','E0277','E0605','E0606','E0369','E0600','E0080','overflowing_literals','arithmetic_overflow'}
def run(row):
 name,item,before,after=row;record={'fixture':name,'address':item['address'],'name':item['name']}
 for side,text in [('before',before),('after',after)]:
  stem=f'{name}-{item["address"]}-{side}';source=out/(stem+'.rs');source.write_text(text)
  p=subprocess.run(['taskset','-c','0-78','rustc','--edition=2021','--crate-name','probe','--crate-type','lib','--emit=metadata','--error-format=json','-Awarnings',str(source),'-o',str(out/(stem+'.rmeta'))],capture_output=True,text=True)
  (out/(stem+'.log')).write_text(p.stderr)
  diagnostics=[]
  for line in p.stderr.splitlines():
   try:d=json.loads(line)
   except json.JSONDecodeError:continue
   if (d.get('code') and d['code']['code'] in codes) or (d.get('level')=='error' and ('format string' in d.get('message','') or 'argument never used' in d.get('message',''))):diagnostics.append({'code':d['code']['code'] if d.get('code') else 'format_error','message':d['message'],'spans':[{k:s[k] for k in ['line_start','line_end','column_start','column_end','label','text']} for s in d['spans']]})
  record[side]={'compiles':p.returncode==0,'diagnostics':diagnostics}
 return record
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:records=list(pool.map(run,tasks))
summary={side:{'functions_with_diagnostics':sum(bool(r[side]['diagnostics']) for r in records),'diagnostics':sum(len(r[side]['diagnostics']) for r in records),'compiles':sum(r[side]['compiles'] for r in records)} for side in ['before','after']}
summary['newly_diagnosed_functions']=[{k:r[k] for k in ['fixture','address','name']} for r in records if not r['before']['diagnostics'] and r['after']['diagnostics']]
summary['previously_compiling_now_failing']=[{k:r[k] for k in ['fixture','address','name']} for r in records if r['before']['compiles'] and not r['after']['compiles']]
(w/'type-audit.json').write_text(json.dumps({'summary':summary,'records':records},indent=2)+'\n');print(json.dumps(summary,indent=2))
