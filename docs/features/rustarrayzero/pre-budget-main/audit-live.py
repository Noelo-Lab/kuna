from common import *
out=w/'breadth';audit=w/'ast-audit/target/release/kuna-rust-array-zero-audit';seen={};reports=[]
while True:
 for path in sorted(out.glob('*-record.json')):
  if path.name in seen:continue
  data=json.loads(path.read_text());input_path=out/(data['fixture']+'-array-audit-input.json');report=out/(data['fixture']+'-array-audit.json');input_path.write_text(json.dumps({'records':[data]})+'\n')
  p=subprocess.run([str(audit),str(input_path),str(report)],capture_output=True);assert p.returncode==0,(path,p.stderr.decode());result=json.loads(report.read_text());seen[path.name]=hashlib.sha256(path.read_bytes()).hexdigest();reports+=result['records'];input_path.unlink();print(data['fixture'],result['changed_records'],'changes',result['pending'],'pending',flush=True)
 if(out/'record.json').exists():
  full=json.loads((out/'record.json').read_text())
  if len(seen)==full['images']:break
 time.sleep(2)
assert len(reports)==full['changed_rust_records']
assert all(hashlib.sha256((out/name).read_bytes()).hexdigest()==digest for name,digest in seen.items())
summary={'scope':result['scope'],'self_checks':result['self_checks'],'changed_records':len(reports),'pending':sum(not x['classified']for x in reports),'records':reports,'per_image_record_sha256':seen};(out/'array-review.json').write_text(json.dumps(summary,indent=2)+'\n');print('FULL ARRAY REVIEW',len(reports),'changes',summary['pending'],'pending',flush=True)
