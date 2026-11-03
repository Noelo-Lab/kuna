from pathlib import Path
import subprocess, json, hashlib, os

w = Path(__file__).parent
root = Path('/home/mahaloz/kwt/angr-rust-integer-conversions')
parent = Path('/home/mahaloz/kwt/angr-go-store-conversions')
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
digest = sha(root / 'decompiler/target/release/kuna')
assert digest == 'f846b2b4f01492a8c26ae22eac3b8563368d756996290ee42cfa76ef29c2f352'
fixture = w / 'fixture'
subprocess.run(['cc', '-O2', '-g', '-fno-inline', '-fno-pie', '-no-pie', str(w / 'probe.c'), '-o', str(fixture)], check=True)
expected = None
for opt in ['0', '2']:
    exe = w / ('reference-' + opt)
    subprocess.run(['cc', '-O' + opt, '-DKUNA_NATIVE_MAIN', str(w / 'probe.c'), '-o', str(exe)], check=True)
    output = subprocess.check_output([str(exe)])
    (w / ('reference-' + opt + '.stdout')).write_bytes(output)
    if expected is not None: assert output == expected
    expected = output
assert len(expected.splitlines()) == 64
records = []; failures = []
for arm, repo in [('parent', parent), ('candidate', root)]:
    command = [str(repo / 'decompiler/target/release/kuna'), 'decompile-all', str(fixture), '--language', 'rust', '--json', '--functions', 'logical_min,logical_max,arithmetic_min,arithmetic_negative', '--assert', '@' + str(w / 'probe.kuna')]
    raw = subprocess.check_output(command, cwd=repo, env=os.environ | {'KUNA_SPECS': str(repo / 'specs')})
    (w / (arm + '.json')).write_bytes(raw)
    data = json.loads(raw); assert len(data['functions']) == 4 and all(not f.get('error') for f in data['functions'])
    assert all(a['status'] == 'applied' for a in data['assertions'])
    text = '\n'.join(f['code'] for f in data['functions']) + '\nfn main(){for n in 0..64u64 {unsafe {println!("{:016x} {:016x} {:016x} {:016x}",logical_min(n) as u64,logical_max(n) as u64,arithmetic_min(n),arithmetic_negative(n));}}}\n'
    source = w / (arm + '.rs'); source.write_text(text)
    for opt in ['0', '2']:
        exe = w / (arm + '-' + opt)
        p = subprocess.run(['rustc', '--edition=2021', '-C', 'opt-level=' + opt, '-C', 'overflow-checks=off', str(source), '-o', str(exe)], capture_output=True, text=True)
        (w / (arm + '-' + opt + '-build.log')).write_text(p.stdout + p.stderr)
        record = {'arm': arm, 'optimization': opt, 'compile_exit': p.returncode}
        if p.returncode == 0:
            output = subprocess.check_output([str(exe)]); (w / (arm + '-' + opt + '.stdout')).write_bytes(output)
            record['matches_reference'] = output == expected
        if arm == 'candidate' and (p.returncode or not record.get('matches_reference')): failures.append(record)
        records.append(record)
assert sha(root / 'decompiler/target/release/kuna') == digest
record = {'status': 'PASS' if not failures else 'FAIL', 'candidate_cli_sha256': digest, 'parent_cli_sha256': sha(parent / 'decompiler/target/release/kuna'), 'complete_functions': 4, 'shift_counts_per_run': 64, 'body_rewrites': False, 'candidate_function_comparisons': 512 if not failures else None, 'records': records, 'failures': failures}
(w / 'record.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record), flush=True)
assert not failures
