from common import *

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
fixture = w / 'public-repro'
run(r, w, ['cc', '-O2', '-g', '-fno-inline', '-fno-pie', '-no-pie', str(w / 'public-repro.c'), '-o', str(fixture)], 'public-repro-build.log')
build = json.loads((w / 'build.json').read_text())
records = []
for arm, root in [('parent', Path('/home/mahaloz/kwt/angr-go-store-conversions')), ('candidate', r)]:
    assert sha(root / 'decompiler/target/release/kuna') == build['parent_cli_sha256' if arm == 'parent' else 'new_cli_sha256']
    command = [str(root / 'decompiler/target/release/kuna'), 'decompile', str(fixture), 'flip', '--language', 'rust', '--assert', 'prototype flip uint8 flip(int8 value)']
    output = subprocess.check_output(command, cwd=root, env=env | {'KUNA_SPECS': str(root / 'specs')}, text=True)
    (w / ('public-repro-' + arm + '.rs')).write_text(output)
    expected = 'return value;' if arm == 'parent' else 'return value as u64;'
    assert expected in output
    records.append({'arm': arm, 'command': command, 'cli_sha256': sha(root / 'decompiler/target/release/kuna'), 'output': output})
(w / 'public-repro.json').write_text(json.dumps({'fixture_sha256': sha(fixture), 'records': records}, indent=2) + '\n')
print('Public Rust return conversion reproduced on exact integrated binaries', flush=True)
