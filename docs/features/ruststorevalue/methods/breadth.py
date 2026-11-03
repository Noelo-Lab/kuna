from common import *
from concurrent.futures import ThreadPoolExecutor, as_completed
import difflib

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
base = Path('/home/mahaloz/kwt/angr-rust-integer-conversions')
prior = json.loads(Path('/tmp/kuna-rust-integer-conversions/integration-store/breadth/record.json').read_text())
build = json.loads((w / 'build.json').read_text())
assert prior['candidate_cli_sha256'] == build['parent_cli_sha256']
out = w / 'breadth'; out.mkdir(exist_ok=False)
assert sha(r / 'decompiler/target/release/kuna') == build['new_cli_sha256']
assert sha(base / 'decompiler/target/release/kuna') == build['parent_cli_sha256']

def functions(path):
    d = json.loads(Path(path).read_text()); assert not d.get('error')
    result = {f['address']: f for f in d['functions']}
    assert len(result) == len(d['functions']) and result
    return result

def capture(root, command, path):
    command = [str(root / 'decompiler/target/release/kuna')] + command[1:]
    with path.open('w') as stdout, path.with_suffix('.log').open('w') as stderr:
        process = subprocess.Popen(command, cwd=root, env=env | {'KUNA_SPECS': str(root / 'specs')}, stdout=stdout, stderr=stderr, start_new_session=True)
        while process.poll() is None:
            memory = {line.split(':')[0]: int(line.split()[1]) for line in Path('/proc/meminfo').read_text().splitlines()}
            if 1 - memory['MemAvailable'] / memory['MemTotal'] >= .9:
                os.killpg(process.pid, signal.SIGTERM); process.wait()
                (w / 'memory-limit-hit.json').write_text(json.dumps(memory))
                raise RuntimeError('90% RAM; own capture stopped; user help required')
            time.sleep(1)
        assert process.returncode == 0, (root, path, process.returncode)
    return command

def row_job(row):
    name = row['fixture']; binary = Path(row['fixture_path'])
    assert sha(binary) == row['fixture_sha256']
    records = []; changes = []
    items = list(row['records'])
    def language_job(item):
        language = item['language']; pair = item['gopairreturns']
        command = [arg.replace(str(base), str(r)) for arg in item['command']]
        command[command.index('gopairreturns') + 1] = pair
        if item.get('fresh_parent'):
            before = out / f'{name}-{language}-{pair}-parent.json'
            capture(base, command, before)
        else:
            before = Path(item['candidate_file'])
            assert sha(before) == item['candidate_sha256']
        old = functions(before)
        path = out / f'{name}-{language}-{pair}-candidate.json'
        command = capture(r, command, path)
        new = functions(path); assert old.keys() == new.keys()
        errors = [hex(address) for address in old if new[address].get('error') and not old[address].get('error')]
        assert not errors, (name, language, pair, errors)
        local = []
        for address in old:
            if old[address] == new[address]: continue
            fields = sorted(key for key in set(old[address]) | set(new[address]) if old[address].get(key) != new[address].get(key))
            assert language == 'rust' and set(fields) <= {'code', 'line_mappings'}, (name, language, pair, hex(address), fields)
            local.append({'language': language, 'gopairreturns': pair, 'address': hex(address), 'name': new[address]['name'], 'fields': fields, 'parent': old[address], 'candidate': new[address], 'diff': list(difflib.unified_diff(old[address].get('code', '').splitlines(), new[address].get('code', '').splitlines(), fromfile='parent', tofile='candidate', lineterm='')), 'review_pending': True})
        return {'language': language, 'gopairreturns': pair, 'functions': len(old), 'parent_file': str(before), 'parent_sha256': sha(before), 'candidate_file': str(path), 'candidate_sha256': sha(path), 'command': command, 'new_errors': errors, 'fresh_parent_capture': bool(item.get('fresh_parent'))}, local

    with ThreadPoolExecutor(max_workers=len(items)) as pool:
        for future in as_completed([pool.submit(language_job, item) for item in items]):
            record, local = future.result(); records.append(record); changes.extend(local)
    assert sha(binary) == row['fixture_sha256']
    result = {'fixture': name, 'fixture_path': str(binary), 'fixture_sha256': row['fixture_sha256'], 'records': records, 'changes': changes}
    (out / (name + '-record.json')).write_text(json.dumps(result, indent=2) + '\n')
    print(name, 'complete', len(changes), 'changed Rust records', flush=True)
    return result

with ThreadPoolExecutor(max_workers=4) as pool:
    records = [future.result() for future in as_completed([pool.submit(row_job, row) for row in prior['records']])]
assert sha(r / 'decompiler/target/release/kuna') == build['new_cli_sha256']
assert sha(base / 'decompiler/target/release/kuna') == build['parent_cli_sha256']
record = {'status': 'captured; semantic review pending', 'scope': '72-image complete function records, with Go pairs disabled/enabled in Go and Rust; C unchanged. Upstream probestack is parsed statically and never executed.', 'images': len(records), 'distinct_functions': sum(row['records'][0]['functions'] for row in records), 'language_function_records': sum(item['functions'] for row in records for item in row['records']), 'changed_rust_records': sum(len(row['changes']) for row in records), 'parent_cli_sha256': build['parent_cli_sha256'], 'candidate_cli_sha256': build['new_cli_sha256'], 'records': records}
(out / 'record.json').write_text(json.dumps(record, indent=2) + '\n')
print('Complete Rust STORE breadth captured', flush=True)
