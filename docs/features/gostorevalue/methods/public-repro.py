from common import *

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
fixture = w / 'public-repro'
run(r, w, ['cc', '-O2', '-g', '-fno-inline', '-fno-pie', '-no-pie', str(w / 'public-repro.c'), '-o', str(fixture)], 'public-repro-build.log')
records = []
for arm, root in [('parent', Path('/home/mahaloz/kwt/angr-go-pair-results')), ('candidate', r)]:
    command = [str(root / 'decompiler/target/release/kuna'), 'decompile', str(fixture), 'put', '--language', 'go', '--assert', 'prototype put void put(int8 *p,uint8 a,uint8 b)', '--option', 'ctypes', 'off', '--option', 'realtypes', 'off']
    output = subprocess.check_output(command, cwd=root, env=env | {'KUNA_SPECS': str(root / 'specs')}, text=True)
    (w / ('public-repro-' + arm + '.go')).write_text(output)
    assert ('*p = a + b' in output) if arm == 'parent' else ('*p = int64(a + b)' in output)
    records.append({'arm': arm, 'command': command, 'cli_sha256': sha(root / 'decompiler/target/release/kuna'), 'output': output})
(w / 'public-repro.json').write_text(json.dumps({'fixture_sha256': sha(fixture), 'records': records}, indent=2) + '\n')
print('Public Go STORE reproduction confirmed on exact parent/candidate', flush=True)
