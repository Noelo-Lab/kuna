from pathlib import Path
import ast
import hashlib
import json
import re
import shutil
import subprocess

w = Path('/tmp/kuna-rust-fmt-write-optin')
old = Path('/tmp/kuna-rust-fmt-write')
r = Path('/home/mahaloz/kwt/angr-rust-fmt-write')
base = Path('/home/mahaloz/kwt/angr-rust-fmt-str')
out = r / 'docs/features/rustfmtwrite'


def read(root, name):
    return json.loads((root / name).read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


gates = read(w, 'gates-passed.json')
assert gates['all_four_gates_passed'] and gates['additional_tools_lint_catalog_counters_passed']
assert not (r / 'decompiler/target/debug').exists()
rows = re.findall(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w / 'workspace-final.log').read_text())
assert rows and all(row[0] == 'ok' and row[2] == '0' for row in rows)
tests = dict(passed=sum(int(x[1]) for x in rows), failed=sum(int(x[2]) for x in rows), ignored=sum(int(x[3]) for x in rows), suites=len(rows))
assert tests == dict(passed=10109, failed=0, ignored=39, suites=71), tests
(w / 'workspace-summary.json').write_text(json.dumps(tests, indent=2) + '\n')
assert all('PARITY OK' in (w / f'{name}.log').read_text() for name in ['original', 'stages'])
assert read(r, 'docs/baseline-stages.json')['data_footer'] == [3541, 3541]
subprocess.run(['git', 'diff', '--exit-code', '--', 'docs/baseline.json'], cwd=r, check=True)
source = read(w, 'production-source-before-validation.json')
assert all(sha(r / p) == h for p, h in source.items())
digest = sha(r / 'decompiler/target/release/kuna')
assert digest == gates['release_sha256'] == read(w, 'build.json')['cli_sha256']
parent = read(old, 'parent-source-before-commit.json')
parent_head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=base, text=True).strip()
assert not subprocess.check_output(['git', 'status', '--porcelain'], cwd=base)
assert all(sha(base / p) == h for p, h in parent['files'].items())
assert sha(base / 'decompiler/target/release/kuna') == parent['binary_sha256']
reanchor = read(old, 'reanchor-proof.json')
assert reanchor['new_baseline_commit'] == parent_head == (old / 'baseline-commit.txt').read_text().strip()
assert reanchor['release_binary_unchanged'] and reanchor['release_binary_sha256'] == digest

equivalence = read(w, 'breadth/record.json')
assert equivalence['images'] == 27 and equivalence['all_complete_function_records_identical']
assert equivalence['new_kuna_sha256'] == digest
assert equivalence['baseline_commit'] == parent_head and not equivalence['baseline_has_pending_feature']
for fixture in equivalence['fixtures']:
    for capture in fixture['captures']:
        assert capture['all_complete_function_records_identical']
        for prefix in ['fresh', 'prior']:
            assert sha(Path(capture[prefix + '_file'])) == capture[prefix + '_sha256']

breadth = read(old, 'breadth/record.json')
summary = read(old, 'breadth-summary.json')
macros = read(old, 'all-macros.json')
native = read(old, 'native-execution.json')
context = read(old, 'reference-context-review.json')
assert equivalence['previous_enabled_kuna_sha256'] == breadth['patched_kuna_sha256']
assert not breadth['semantic_evidence_pending'] and not breadth['baseline_has_pending_feature']
assert breadth['baseline_commit'] == parent_head
assert summary['images'] == 27 and summary['changed_functions'] == macros['changed_functions'] == macros['recovered_calls'] == 8
assert macros['all_template_bytes_and_argument_bindings_verified'] and macros['all_writer_vtables_and_result_bytes_verified'] and not macros['source_mapping_classification_pending']
assert native['total_comparisons'] == 21168 and native['texts'] == 156
assert all(x['original_string_bytes_and_result_bytes_match'] and x['calls'] == 7056 for x in native['results'])
assert context['context_difference_preexists_feature'] and context['total_comparisons'] == 7128
assert all(x['original_string_bytes_and_result_bytes_match'] for x in context['executions'])

attempt = old / 'timing-randomized-attempts/20261102-005536'
enabled_monitor = read(attempt, 'monitor.json')
assert not enabled_monitor['accepted_for_speed_review']
failed = [name for name, result in enabled_monitor['results'].items() if not result['speed_budget_passed']]
assert failed == ['scalar-c'] and len(enabled_monitor['results']) == 10
for name in enabled_monitor['results']:
    row = read(attempt, f'timing-{name}.json')
    assert row['binaries_sha256'] == {'baseline': parent['binary_sha256'], 'patched': breadth['patched_kuna_sha256']}
    assert row['samples'] == 768 and row['warmups'] == 3 and row['plan']['maximum_looks'] == 1
timing = read(w, 'timing-default-result.json')
assert timing['all_default_comparisons_passed'] and not timing['enabled_default_on_accepted']
assert timing['binary_sha256'] == {'parent': parent['binary_sha256'], 'default_off': digest}
assert timing['parent_commit'] == parent_head
assert timing['plan']['samples_per_arm'] == 192 and timing['plan']['maximum_looks'] == 1
assert all(row['speed_budget_passed'] for row in timing['records'])
maxima = {metric: max(row['median_delta_pct'][metric] for row in timing['records']) for metric in ['cpu_ms', 'wall_ms']}

out.mkdir(parents=True, exist_ok=True)
record = {'option': 'rustfmtwrite', 'default': 'off', 'enabled_in_any_preset': False,
          'baseline_commit': parent_head, 'integrated_main_commit': '69fad3ce4cbdbb3232680938252bd75c2bec5a68',
          'kuna_sha256': digest, 'previous_enabled_kuna_sha256': breadth['patched_kuna_sha256'],
          'reference_angr_commit': '6afa0c79a83d9f677a46b0275cdb63bf16c1979e',
          'reference_binaries_commit': '76ad9e296dde8f82b8ede93ce06a578cd1933639',
          'datatests': {'passed': 675, 'total': 675, 'baseline_unchanged': True},
          'stages': {'passed': 3541, 'total': 3541, 'new_case_before': 0, 'new_case_enabled': 10, 'new_case_assertions': 10},
          'workspace': tests, 'tools': {'passed': 114, 'skipped': 1}, 'lint': True, 'spec': True,
          'catalog': {'rows': 367, 'live_fields': 173, 'corpus_files': 646, 'check': True},
          'breadth': summary, 'fresh_complete_record_equivalence': equivalence,
          'native': native, 'reference_context_native': context, 'macro_review': macros,
          'native_reuse_basis': 'Previous native execution is reused only for freshly captured byte-identical enabled expressions; native execution was not repeated for this default-only rebuild.',
          'default_off_speed': timing, 'enabled_default_on_accepted': False,
          'failed_enabled_speed_study': enabled_monitor, 'default_decision': read(w, 'decision.json'),
          'limitations': ['Only exact stable Rust 1.93.0/1.93.1 x86-64 little-endian ELF String writer identities and layouts qualify.',
                          'Only supported literal, primitive integer and string descriptors are recovered.',
                          'Unchanged emitted expressions execute with explicit typed inputs; complete-body recompilation is not claimed.',
                          'I/O writers, dynamic formatting, derived callbacks, older writer layouts and full Go/Rust parity remain separate work.']}
(out / 'record.json').write_text(json.dumps(record, indent=2) + '\n')

# Preserve the earlier proof artifacts under their original binary identity.
prior = out / 'enabled-proof'
prior.mkdir(exist_ok=True)
tree = ast.parse((old / 'finalize.py').read_text())
old_files = next(ast.literal_eval(node.value) for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'files' for t in node.targets))
for name in old_files:
    if (old / name).is_file():
        shutil.copy2(old / name, prior / name)
for name in ['review-reference-context.py', 'reference-context-review.json', 'reference-context-review.log', 'reference-context-native.rs', 'reference-context-execution.rs', 'postreview-initial-context-failure.log', 'breadth-69fa-initial-context-failure.log', 'complete-context-review.py', 'complete-context-review.log', 'breadth-before-parent-commit.json', 'parent-source-before-commit.json']:
    shutil.copy2(old / name, prior / name)
shutil.copy2(old / 'breadth/record.json', prior / 'breadth.json')
for name in ['overlap-capture-plan.json', 'capture-ready.json']:
    shutil.copy2(old / 'breadth' / name, prior / name)
for pattern in ['*.diff', '*-record.json', '*-descriptor.jsonl']:
    for p in (old / 'breadth').glob(pattern):
        shutil.copy2(p, prior / p.name)
for pattern in ['*.compile.log', '*.stdout', 'result-abi*', 'initial-integer-writer-binding-*.rs', 'initial-integer-writer-binding-*.log']:
    for p in old.glob(pattern):
        if p.is_file() and p.suffix in ['.rs', '.json', '.log', '.stdout']:
            shutil.copy2(p, prior / p.name)
shutil.copytree(old / 'verifier/src', prior / 'verifier', dirs_exist_ok=True)
for name in ['Cargo.toml', 'Cargo.lock']:
    shutil.copy2(old / 'verifier' / name, prior / 'verifier' / name)
shutil.copytree(old / 'timing-randomized-attempts', out / 'enabled-timing-attempts', dirs_exist_ok=True)
for name in ['finalize.py', 'validate.py', 'validate.log', 'breadth.py', 'breadth.log', 'decision.json', 'build.json', 'preset-policy-correction.json', 'production-source-before-validation.json', 'gates-passed.json', 'workspace-summary.json', 'workspace-final.log', 'original.log', 'stages.log', 'tools.log', 'lint.log', 'spec.log', 'catalog.log', 'counters.log', 'timing-default.py', 'timing-default-plan.json', 'timing-default-result.json', 'timing-default-cpus.json', 'timing-default-cpu-samples.json.gz', 'timing-default.log']:
    shutil.copy2(w / name, out / name)
shutil.copy2(w / 'breadth/record.json', out / 'fresh-breadth.json')
shutil.copy2(old / 'reanchor-proof.json', out / 'reanchor-proof.json')
shutil.copy2(w / 'timing-default-plan-before-reservation.json', out / 'timing-default-plan-before-reservation.json')
shutil.copy2(Path(timing['plan']['compiled_spec_verification']['manifest']), out / 'built-specs.json')
shutil.copytree(w / 'initial-preset-policy-failure', out / 'initial-preset-policy-failure', dirs_exist_ok=True)

old_text = (old / 'finalize.py').read_text()
analysis = old_text.split("(out/'analysis.md').write_text(f'''", 1)[1].split('## Repository checks and speed', 1)[0]
for key, value in summary.items():
    analysis = analysis.replace("{summary['" + key + "']:,}", f'{value:,}').replace("{summary['" + key + "']}", str(value))
analysis = analysis.replace('# Verified Rust String writes', '# Opt-in verified Rust String writes')
analysis = analysis.replace('`rustfmtwrite` recovers', '`rustfmtwrite`, off in every preset, recovers')
analysis += f'''## Default decision and fresh validation

The enabled implementation did not meet its predeclared speed criterion. In the
768-round three-arm study, nine cases passed but the small C control had upper
mean wall-time bounds of +5.603% against the parent and +6.125% against the
disabled option, above the 5% budget. Its median wall deltas were +0.084% and
+0.151%. This is an unresolved confidence bound, not a measured 6% median
slowdown. The failed study is retained and was not repeated unchanged.

The option consequently ships off in every preset. A fresh release build and
all four required checks pass: 675 original assertions, 3,541 stage assertions,
10,109 workspace tests with zero failures and 39 ignored across 71 suites.
Tooling passes 114 tests with one optional skip. Lint, spec, catalog, generated
documentation and shared counts pass. The initial preset-policy test failure is
retained with the test-only correction documenting the intentional exclusion.

Fresh captures of all 27 images compare every complete function record. Default
and explicit-off Rust output equal the previously reviewed parent output;
explicit-on Rust and default C/Go equal their previously reviewed output. This
binds the native execution evidence above to byte-identical enabled expressions
from the new binary. Native execution was not repeated for this default-only
rebuild. `fresh-breadth.json` records both generations of capture hashes.

A separately declared default-off comparison uses four cases, 192 balanced
two-arm rounds, three warmups and one final 20,000-resample block bootstrap,
with family alpha 0.05 across eight CPU/wall comparisons. Every median ratio
and upper mean-ratio bound meets 5%. Maximum parent-to-default median CPU/wall
deltas are {maxima['cpu_ms']:+.3f}%/{maxima['wall_ms']:+.3f}%. This verifies the
configuration being shipped and cannot promote recovery to default-on. Both
studies retain observations, CPU assignments, telemetry and binary identities.

The enabled proof artifacts name their original binary and capture state. The
parent source manifest and reanchor receipt bind that state to committed parent
`{parent_head}` without changing its executable. The fresh binary hash is
`{digest}`. Full Go/Rust parity remains separate work.
'''
assert '{summary[' not in analysis
(out / 'analysis.md').write_text(analysis)
print(json.dumps({'workspace': tests, 'breadth': summary, 'default': 'off', 'max_default_median_delta_pct': maxima}, indent=2))
