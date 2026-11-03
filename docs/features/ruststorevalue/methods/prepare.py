from common import *
import datetime

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
base = Path('/home/mahaloz/kwt/angr-rust-integer-conversions')
parent = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=r, text=True).strip()
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=base, text=True).strip() == parent
assert not subprocess.check_output(['git', 'status', '--porcelain'], cwd=base)
assert sha(base / 'decompiler/target/release/kuna') == 'f846b2b4f01492a8c26ae22eac3b8563368d756996290ee42cfa76ef29c2f352'
kuna = r / 'decompiler/target/release/kuna'
rows = json.loads(subprocess.check_output([str(kuna), 'catalog', '--json'], text=True))
assert rows == json.loads(subprocess.check_output([str(base / 'decompiler/target/release/kuna'), 'catalog', '--json'], text=True))
run(r, w, [str(kuna), 'catalog', '--check'], 'catalog.log')
run(r, w, ['python3', '-m', 'scripts.repipe.counters', '--check'], 'counters.log')
command = [str(kuna), 'test', '--datatests', '--datatests-dir', 'tests/stages']
for name in ['ghangr-rust-store-value', 'ghangr-rust-integer-conversions', 'ghangr-go-store-value']:
    command += ['--name', name]
run(r, w, command, 'stage-focused.log')
run(r, w, [str(kuna), 'test', '--datatests', '--datatests-dir', 'tests/stages', '--save-baseline', 'docs/baseline-stages.json'], 'record-stages.log')
old = json.loads(subprocess.check_output(['git', 'show', parent + ':docs/baseline-stages.json'], cwd=r, text=True))
new = json.loads((r / 'docs/baseline-stages.json').read_text())
before = set(old['passing']); after = set(new['passing'])
assert not before - after
assert len(after - before) == 5
assert new['data_footer'] == [old['data_footer'][0] + 5] * 2
assert not subprocess.check_output(['git', 'diff', 'HEAD', '--', 'docs/baseline.json'], cwd=r)
corpus = sum(1 for folder in ['tests/datatests', 'tests/stages'] for _ in (r / folder).glob('*.xml'))
record = {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'parent_commit': parent, 'parent_worktree_head': parent, 'parent_tree': subprocess.check_output(['git', 'rev-parse', parent + '^{tree}'], cwd=r, text=True).strip(), 'parent_cli_sha256': sha(base / 'decompiler/target/release/kuna'), 'new_cli_sha256': sha(kuna), 'catalog_count': len(rows), 'corpus_files': corpus, 'stage_assertions': new['data_footer'], 'stage_keys_added': sorted(after - before)}
(w / 'build.json').write_text(json.dumps(record, indent=2) + '\n')
print('Rust STORE prepared', json.dumps(record), flush=True)
