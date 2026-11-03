from common import *
import datetime

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
plan = json.loads((w / 'timing-plan.json').read_text())
build = json.loads((w / 'build.json').read_text())
gates = json.loads((w / 'gates-passed.json').read_text())
review = json.loads((w / 'breadth/structural-review.json').read_text())
assert gates['all_four_gates_passed'] and review['complete']
assert review['native_complete_bodies'] == 97 and review['native_function_comparisons'] == 776
assert gates['release_sha256'] == build['new_cli_sha256']
assert not (w / 'timing-result.json').exists() and not list(w.glob('*-samples.json'))
base = Path(plan['base'])
assert not subprocess.check_output(['git', 'status', '--porcelain'], cwd=base)
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=base, text=True).strip() == build['parent_worktree_head']
assert sha(base / 'decompiler/target/release/kuna') == build['parent_cli_sha256']
assert sha(r / 'decompiler/target/release/kuna') == build['new_cli_sha256']
for p, digest in plan['fixture_sha256'].items(): assert sha(p) == digest, p
files = subprocess.check_output(['git', 'ls-files', 'decompiler', 'tests', 'specs', 'VERSION', 'Makefile', 'docs/spec', 'docs/options.md'], cwd=base, text=True).splitlines()
(w / 'parent-source-before-timing.json').write_text(json.dumps({'commit': build['parent_worktree_head'], 'files': {p: sha(base / p) for p in files if (base / p).is_file()}}, indent=2) + '\n')
current = json.loads((w / 'production-source-before-validation.json').read_text())
assert all(sha(r / p) == digest for p, digest in current.items())
plan.update(build=build, candidate_cli_sha256=build['new_cli_sha256'], bound_at=datetime.datetime.now(datetime.timezone.utc).isoformat())
(w / 'timing-plan.json').write_text(json.dumps(plan, indent=2) + '\n')
print('Rust array decay speed inputs bound to validated sources, binaries and fixtures', flush=True)
