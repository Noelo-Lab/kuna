from pathlib import Path
import json, subprocess, re

w = Path('/tmp/kuna-go-byte-extract-b848-review')
base = w/'baseline-breadth'
out = w/'diagnostic-probes'
out.mkdir(exist_ok=True)
records = []
for row in json.loads((w/'typecheck-all/record.json').read_text()):
    if not row['functions']:
        continue
    original = {hex(f['address']):f for f in json.loads((base/(row['fixture']+'.json')).read_text())['functions']}
    probes = []
    for finding in row['functions']:
        p = original[finding['address']].copy()
        p['code'] += '\nfunc SUB168(value [16]byte, offset int) int64 { return 0 }\n'
        probes.append(p)
    path = out/(row['fixture']+'.json')
    path.write_text(json.dumps({'functions':probes}))
    p = subprocess.run([str(w/'typecheck'),str(path)],capture_output=True,text=True,check=True)
    (out/(row['fixture']+'-diagnostics.json')).write_text(p.stdout)
    old = {hex(f['address']):f['errors'] for f in json.loads(p.stdout)}
    for finding in row['functions']:
        expected = [re.sub(r'\*\(\*int64\)\(unsafe.Add\(unsafe.Pointer\(&(\w+)\), (\d+)\)\)', r'SUB168(\1, \2)', e) for e in finding['errors']]
        assert all(e in old[finding['address']] for e in expected), (row['fixture'],expected,old)
        records.append({'fixture':row['fixture'],'function':finding['name'],'address':finding['address'],'new_diagnostics':finding['errors'],'reproduced_by_original_body_with_typed_SUB168_declaration':expected,'classification':'existing arithmetic width mismatch' if 'refill' in finding['name'] else 'existing recursive prototype mismatch'})
(w/'diagnostic-classification.json').write_text(json.dumps({'note':'Partial Go type check; unresolved external symbols omitted. Probe adds only an int64-returning declaration for the original eight-byte SUB168 helper. Every remaining after-only diagnostic is reproduced in that original body, with extraction expressions normalized to the old helper. No source expression or prototype is repaired.','records':records},indent=2)+'\n')
print('Remaining diagnostic records reproduced from original bodies:',len(records))
