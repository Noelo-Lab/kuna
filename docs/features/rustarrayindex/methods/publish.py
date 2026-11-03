from common import *
import datetime

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
build = json.loads((w / 'build.json').read_text())
merged = json.loads((w / 'rust-pointer-merged.json').read_text())
record = json.loads((r / 'docs/features/rustarrayindex/record.json').read_text())
assert record['gates']['all_four_gates_passed']
assert record['breadth']['pending_records'] == 0
assert sha(r / 'decompiler/target/release/kuna') == build['new_cli_sha256']
sources = json.loads((w / 'production-source-before-validation.json').read_text())
assert all(sha(r / p) == digest for p, digest in sources.items())
manifest = json.loads((r / 'docs/features/rustarrayindex/manifest.json').read_text())['files']
assert all(sha(r / 'docs/features/rustarrayindex' / p) == digest for p, digest in manifest.items())
remote = subprocess.check_output(['git', 'ls-remote', 'origin', 'refs/heads/main'], cwd=r, text=True).split()[0]
assert remote == merged['merged_commit'], ('main advanced; inspect and integrate before publishing', remote)
subprocess.run(['git', 'fetch', 'origin', 'main'], cwd=r, check=True)
assert subprocess.check_output(['git', 'rev-parse', 'origin/main^{tree}'], cwd=r, text=True).strip() == merged['merged_tree']
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=r, text=True).strip() == build['parent_commit']
assert not subprocess.check_output(['git', 'diff', '--name-only', '--diff-filter=U'], cwd=r)
subprocess.run(['git', 'diff', '--check'], cwd=r, check=True)
changed = subprocess.check_output(['git', 'diff', '--name-only', build['parent_commit']], cwd=r, text=True).splitlines()
untracked = subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard'], cwd=r, text=True).splitlines()
paths = sorted(set(changed + untracked))
assert all(p.startswith(('decompiler/crates/', 'docs/features/rustarrayindex/', 'docs/spec/', 'tests/stages/')) or p == 'docs/baseline-stages.json' for p in paths), paths
assert not any('/target/' in p or p.endswith('.sla') for p in paths)
subprocess.run(['git', 'add', '--', *paths], cwd=r, check=True)
subprocess.run(['git', 'commit', '-m', '[AUTOMATED] Convert Rust integer array indices to usize'], cwd=r, check=True)
head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=r, text=True).strip()
tree = subprocess.check_output(['git', 'rev-parse', 'HEAD^{tree}'], cwd=r, text=True).strip()
assert not subprocess.check_output(['git', 'status', '--porcelain'], cwd=r)
(w / 'commit.json').write_text(json.dumps({'head': head, 'tree': tree, 'parent_main': remote, 'published_parent': build['parent_commit'], 'at': datetime.datetime.now(datetime.timezone.utc).isoformat()}, indent=2) + '\n')
subprocess.run(['git', 'push', '-u', 'origin', 'fix/angr-rust-array-index'], cwd=r, check=True)
body = (w / 'pr-body-template.txt').read_text().replace('@HEAD@', head).replace('@MEDIAN@', f"{record['max_cpu_or_wall_median_delta_pct']:+.3f}").replace('@UPPER@', f"{record['max_95pct_upper_mean_delta_pct']:+.3f}")
(w / 'pr-body.txt').write_text(body)
output = subprocess.check_output(['gh', 'pr', 'create', '--repo', 'Noelo-Lab/kuna', '--base', 'main', '--head', 'fix/angr-rust-array-index', '--title', '[AUTOMATED] Convert Rust integer array indices to usize', '--body-file', str(w / 'pr-body.txt'), '--label', 'full-ci'], cwd=r, text=True)
(w / 'published.txt').write_text(output)
print(output, flush=True)
