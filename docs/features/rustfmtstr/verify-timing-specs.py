from pathlib import Path
import hashlib,json,datetime
reference=Path('/tmp/kuna-rust-io-proposal/built-specs.json')
expected=json.loads(reference.read_text())
roots=[Path('/home/mahaloz/kwt')/name for name in ['angr-rust-panic-main','angr-rust-fmt-pieces','angr-rust-fmt-str','angr-rust-fmt-write']]
rows=[]
for root in roots:
 mismatches=[]
 for relative,digest in expected.items():
  path=root/'specs'/relative
  actual=hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
  if actual!=digest:mismatches.append({'path':relative,'expected':digest,'actual':actual})
 rows.append({'root':str(root),'checked':len(expected),'mismatches':mismatches})
record={'scope':'Post-study compiled-spec identity check against the validated parent-derived manifest recorded before sampling. This check was not part of the frozen timing runner or its predeclared protocol.','recorded_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'reference_manifest':str(reference),'reference_sha256':hashlib.sha256(reference.read_bytes()).hexdigest(),'roots':rows,'all_identical':all(not row['mismatches']for row in rows)}
p=Path('/tmp/kuna-angr-reference/timing-specs-post-study.json');p.write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record,indent=2))
assert record['all_identical']
