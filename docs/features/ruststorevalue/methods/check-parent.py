from pathlib import Path
import json,subprocess,hashlib
w=Path('/tmp/kuna-rust-store-conversions');r=Path('/home/mahaloz/kwt/angr-rust-integer-conversions');fixtures=r/'decompiler/crates/kuna-analysis/tests/fixtures'
capture=json.loads(Path('/tmp/kuna-rust-store-research/fields-f846/capture.json').read_text());functions={f['name']:f for f in capture['functions']};names=json.loads((w/'cases.json').read_text());assert len(names)==33
for name in names:assert not functions[name].get('error')
code=(w/'harness.rs').read_text().replace('/* EMITTED_BODIES */','\n'.join(functions[name]['code']for name in names));(w/'parent.rs').write_text(code)
records=[]
for opt in ['0','2']:
 cmd=['cc','-O'+opt,'-fwrapv','-I',str(fixtures),str(w/'harness.c'),str(fixtures/'go_store_value_helpers.c'),'-o',str(w/('reference-'+opt))]
 result=subprocess.run(cmd,capture_output=True);(w/('reference-'+opt+'-build.log')).write_bytes(result.stdout+result.stderr);assert result.returncode==0
 output=subprocess.check_output([str(w/('reference-'+opt))]);(w/('reference-'+opt+'.stdout')).write_bytes(output);assert len(output.splitlines())==324*33
 records.append({'kind':'C reference','optimization':opt,'command':cmd,'output_sha256':hashlib.sha256(output).hexdigest()})
 cmd=['rustc','--edition=2021','--error-format=json','-C','opt-level='+opt,'-C','overflow-checks=off',str(w/'parent.rs'),'-o',str(w/('parent-'+opt))]
 result=subprocess.run(cmd,capture_output=True);(w/('parent-'+opt+'-build.jsonl')).write_bytes(result.stdout+result.stderr)
 errors=[json.loads(line)for line in result.stderr.splitlines()if line.startswith(b'{')];errors=[e for e in errors if e.get('level')=='error' and e.get('code')]
 assert result.returncode!=0 and errors and all(e['code']['code']=='E0308'for e in errors),[(e.get('code'),e.get('message'))for e in errors]
 records.append({'kind':'parent Rust','optimization':opt,'command':cmd,'compile_exit':result.returncode,'error_count':len(errors),'error_codes':sorted({e['code']['code']for e in errors})})
assert (w/'reference-0.stdout').read_bytes()==(w/'reference-2.stdout').read_bytes()
(w/'parent-record.json').write_text(json.dumps({'status':'Parent failure reproduced; C references agree','parent_cli_sha256':'f846b2b4f01492a8c26ae22eac3b8563368d756996290ee42cfa76ef29c2f352','complete_bodies':33,'input_pairs':324,'body_rewrites':False,'candidate_native_equivalence_proven':False,'records':records},indent=2)+'\n')
print('Parent Rust fails only E0308; C guard-byte references agree at O0/O2',flush=True)
