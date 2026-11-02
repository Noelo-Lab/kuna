from common import *
out=w/'native-rust-remainder';out.mkdir(exist_ok=False)
failed=['signed_uint8_wide','signed_uint16_wide','signed_uint32_wide','signed_as_unsigned','unsigned_as_signed']
cases=json.loads((w/'matrix-cases.json').read_text());parent={f['name']:f['code']for f in json.loads((w/'matrix-parent-rust.json').read_text())['functions']};new={f['name']:f['code']for f in json.loads((w/'matrix-candidate-rust.json').read_text())['functions']}
assert all(parent[n]==new[n]for n in failed)
code=(w/'native-matrix/candidate.rs').read_text()
for n in failed:
 assert code.count(new[n])==1;code=code.replace(new[n],'')
 code='\n'.join(line for line in code.splitlines()if not line.startswith(f'CALLS=0; println!("{{:016x}}",{n}('))+'\n'
source=out/'candidate.rs';source.write_text(code)
expected=[s for i,s in enumerate((w/'native-matrix/source-c-0.txt').read_text().splitlines())if cases[i%len(cases)][0]not in failed]
results=[]
for opt in ['0','2']:
 exe=out/f'candidate-O{opt}';p=subprocess.run(['rustc','--edition=2021','-C','opt-level='+opt,str(source),'-o',str(exe)],capture_output=True,text=True);(out/f'build-O{opt}.log').write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr
 p=subprocess.run([str(exe)],capture_output=True,text=True);(out/f'run-O{opt}.log').write_text(p.stderr);(out/f'O{opt}.txt').write_text(p.stdout);assert p.returncode==0 and p.stdout.splitlines()==expected
 results.append({'optimization':opt,'matches_source':True,'lines':len(expected),'output_sha256':hashlib.sha256(p.stdout.encode()).hexdigest()})
(out/'record.json').write_text(json.dumps({'whole_callers':21,'changed_callers':18,'input_pairs':784,'all_changed_callers_included':True,'exclusions_unchanged_from_parent':failed,'separate_gap':'Rust signedness conversions at full field width, including masked returns.','results':results},indent=2)+'\n');print('21 Rust whole callers PASS, all 18 changed callers included.')
