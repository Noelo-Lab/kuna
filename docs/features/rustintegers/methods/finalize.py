from common import *
import datetime, gzip, lzma, re, shutil
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
build = json.loads((w / 'build.json').read_text())
gates = json.loads((w / 'gates-passed.json').read_text())
review = json.loads((w / 'breadth/structural-review.json').read_text())
speed = json.loads((w / 'timing-result.json').read_text())
merged = json.loads((w / 'go-store-merged.json').read_text())
native = [json.loads((w / directory / 'record.json').read_text()) for directory in ['native-context', 'native-conversions']]
extra = json.loads((w.parent / 'shift-signedness-controls/record.json').read_text())
assert merged['merged_tree'] == build['parent_tree'] == '4b1229a6935ad2b5cd9ef8d0568b7a51d4177473'
assert gates['all_four_gates_passed'] and review['complete'] and speed['all_default_comparisons_passed']
assert gates['release_sha256'] == build['new_cli_sha256'] == speed['binary_sha256']['candidate']
assert all(record['status'] == 'PASS' and not record['body_rewrites'] and record['candidate_cli_sha256'] == build['new_cli_sha256'] for record in native)
assert sum(record['function_comparisons'] for record in native) == 70152
assert extra['status'] == 'PASS' and not extra['body_rewrites'] and extra['candidate_cli_sha256'] == build['new_cli_sha256']
assert extra['complete_functions'] == 4 and extra['candidate_function_comparisons'] == 512
assert sha(r / 'decompiler/target/release/kuna') == build['new_cli_sha256']
sources = json.loads((w / 'production-source-before-validation.json').read_text())
assert all(sha(r / p) == digest for p, digest in sources.items())
assert not subprocess.check_output(['git', 'diff', build['parent_commit'], '--', 'docs/baseline.json'], cwd=r)
out = r / 'docs/features/rustintegers'
assert not out.exists()
out.mkdir(parents=True)
def copy(source, destination=None):
    source = Path(source); destination = out / (destination or source.name)
    destination.parent.mkdir(parents=True, exist_ok=True); shutil.copy2(source, destination)
def compressed(source, destination):
    destination = out / destination; destination.parent.mkdir(parents=True, exist_ok=True)
    with Path(source).open('rb') as src, destination.open('wb') as raw, gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0) as sink:
        shutil.copyfileobj(src, sink)
def archive_text_directory(source, destination):
    for p in sorted(Path(source).iterdir()):
        if not p.is_file() or p.suffix not in {'.json', '.c', '.rs', '.kuna', '.log', '.stdout', '.stderr', '.py'}: continue
        if p.stat().st_size > 40000 or p.suffix in {'.log', '.stdout', '.stderr'}:
            compressed(p, destination + '/' + p.name + '.gz')
        else: copy(p, destination + '/' + p.name)
for name in ['build.json', 'gates-passed.json', 'public-repro.c', 'public-repro.json', 'public-repro-parent.rs', 'public-repro-candidate.rs', 'timing-plan.json', 'timing-result.json', 'timing-cpus.json', 'timing-cpu-samples.json.gz', 'production-source-before-validation.json', 'parent-source-before-timing.json', 'go-store-merged.json', 'audit-selftest-input.json', 'audit-selftest-output.json']:
    copy(w / name)
for name in ['native.py', 'native-context-run.py', 'native-conversions-run.py', 'breadth.py', 'review.py', 'validate.py', 'prepare.py', 'run-validation.py', 'timing.py', 'bind-timing.py', 'public-repro.py', 'common.py', 'finalize.py', 'audit-selftest.py']:
    copy(w / name, 'methods/' + name)
for name in ['original.log', 'stages.log', 'tools.log', 'lint.log', 'spec.log', 'catalog.log', 'counters.log', 'workspace-final.log', 'build.log', 'ast-review-v3-build.log', 'ast-review-v3.log', 'audit-selftest.log', 'timing.log']:
    compressed(w / name, 'logs/' + name + '.gz')
for directory in ['native-context', 'native-conversions']:
    archive_text_directory(w / directory, directory)
archive_text_directory(w.parent / 'shift-signedness-controls', 'supplementary-shift-controls')
for p in w.glob('*-samples.json'): copy(p, 'speed/' + p.name)
with (w / 'breadth/record.json').open('rb') as src, lzma.open(out / 'breadth/complete-changes.json.xz', 'wb', preset=6) as dst:
    shutil.copyfileobj(src, dst)
for name in ['structural-review.json', 'ast-algebra-review.json', 'ast-algebra-review-v2.json', 'ast-algebra-review-v3.json']:
    compressed(w / 'breadth' / name, 'breadth/' + name + '.gz')
for name in ['Cargo.toml', 'Cargo.lock', 'src/main.rs', 'pre-full-review-version.rs', 'pre-additive-review.rs']:
    copy(w.parent / 'ast-audit' / name, 'methods/ast-audit/' + name)
copy(Path('/tmp/kuna-rust-io-proposal/built-specs.json'), 'compiled-specs.json')
copy(w.parent / 'plan.json', 'diagnostics/development-plan.json')
for directory in ['shift-context', 'context-expanded', 'angle-subtraction-before']:
    archive_text_directory(w.parent / directory, 'diagnostics/' + directory)
for name in ['record.json', 'ast-review.json', 'ast-review-diagnostics.json']:
    compressed(w.parent / 'context-preflight' / name, 'diagnostics/context-preflight/' + name + '.gz')
for directory in ['before-context-fix', 'before-angle-context-fix']:
    for p in (w.parent / directory).glob('*.rs'):
        compressed(p, 'diagnostics/' + directory + '/' + p.name + '.gz')
rows = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w / 'workspace-final.log').read_text())
workspace = dict(zip(['passed', 'failed', 'ignored'], [sum(int(row[i]) for row in rows) for i in range(3)])); workspace['suites'] = len(rows)
assert workspace == {'passed': 10361, 'failed': 0, 'ignored': 39, 'suites': 72}
max_median = max(value for row in speed['records'] for value in row['median_delta_pct'].values())
max_upper = max(value for row in speed['records'] for value in row['confidence_upper_mean_delta_pct'].values())
summary = {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'feature': 'Rust primitive integer conversions and expression context', 'parent_commit': build['parent_commit'], 'parent_tree': merged['merged_tree'], 'merged_parent_commit': merged['merged_commit'], 'candidate_cli_sha256': build['new_cli_sha256'], 'parent_cli_sha256': build['parent_cli_sha256'], 'gates': gates, 'workspace': workspace, 'registered_native_complete_bodies': 64, 'registered_native_function_comparisons': 70152, 'supplementary_native_complete_bodies': 4, 'supplementary_native_function_comparisons': 512, 'breadth': review, 'max_cpu_or_wall_median_delta_pct': max_median, 'max_95pct_upper_mean_delta_pct': max_upper, 'upper_bound_is_acceptance_gate': False, 'whole_runtime_native_equivalence_claimed': False}
(out / 'record.json').write_text(json.dumps(summary, indent=2) + '\n')
table = '\n'.join('| ' + row['case'] + ' | ' + ' | '.join(f"{row[key][metric]:+.3f}%" for key in ['median_delta_pct', 'confidence_upper_mean_delta_pct'] for metric in ['cpu_ms', 'wall_ms']) + ' |' for row in speed['records'])
(out / 'analysis.md').write_text(f'''# Rust primitive integer conversions

Rust returns, copies and modular binary operations now convert primitive integers when the rendered value and its destination have the same 1-, 2-, 4- or 8-byte width but different signedness. The public reproduction changes `return value;` to `return value as u64;` for an `i64` argument and `u64` result. Noninteger values, enums and unsupported widths retain their existing handling. The internal cast-suppression setting remains honored.

Context-dependent literals retain their source integer width through an explicit `::core::convert::identity::<T>` expression before conversion. This also covers existing same-width casts around shifts. Casts at the end of nested expressions receive parentheses where a following comparison or shift token would otherwise be parsed as generic arguments. Compound assignment expands only when the emitter's existing safe-place rules permit it; call values are evaluated once.

The registered native tests compare 64 complete emitted bodies against C at O0/O2: 70,152 function/input comparisons. Rust uses `overflow-checks=off` and C uses `-fwrapv` to model machine wrapping arithmetic. The bodies are not rewritten. Coverage includes signed/unsigned widths, high bits, constants, operators, nested shifts, comparison and shift grammar boundaries, and exact callback counts in loops. Four supplementary complete right-shift bodies add 512 comparisons over all 64 shift counts; these are archived separately and are not claimed as registered cargo tests.

The static comparison covers {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records. All {review['changed_rust_records']} changed Rust records are classified; C and Go output is unchanged. No new decompiler or Rust parser failures were found. Twenty-eight previously invalid records now parse, while 66 retain existing syntax failures. The corpus includes static-only inspection of the upstream `probestack` image; it was never executed.

The structural audit strips primitive casts and exact integer identity calls, expands safe local compound assignments, and canonicalizes modular arithmetic while preserving operand order. For the 94 records with a raw parser failure in either version, review-only lexical cast erasure and reserved-identifier renaming permit comparison; original output and parse status remain archived. Thirteen audit controls exercise accepted transformations and rejection of changed call order, duplicated calls, changed literals/operators/precedence and repeated pointer evaluation. This comparison does not establish native equivalence, type correctness, or preservation of existing casts for whole runtime images. Those claims remain bounded by the source guards and targeted native tests.

All four repository checks pass, including {workspace['passed']} workspace tests, all original 675 assertions and {gates['stage_assertions']} stage assertions. The original baseline is unchanged. The stage total grows by 12: eleven new cases and one exact Rust call-result conversion check. Catalog metadata is unchanged.

The speed plan was declared before the integrated build or sampling: 64 balanced pairs per case, three warmups per arm, CPU and wall median deltas at most 5%. All cases pass. The 95% upper bounds on mean deltas report uncertainty and are not acceptance thresholds. The maximum measured median delta is {max_median:.3f}%; the largest reported upper bound is {max_upper:.3f}%. Raw samples and CPU telemetry are retained.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

Earlier prototypes exposed a literal shift-width regression, an inherited explicit-cast context bug, and nested cast parsing errors. Their failed probes and outputs remain in `diagnostics/`; passing evidence applies only to the exact candidate identified in `record.json`. Both executables, production sources and all 154 compiled specs were verified before and after timing. `manifest.json` hashes the archive. STORE conversion/width handling, raw-pointer indexing, callback types and Rust enum recovery remain separate work.
''')
manifest = {str(p.relative_to(out)): sha(p) for p in sorted(out.rglob('*')) if p.is_file()}
(out / 'manifest.json').write_text(json.dumps({'files': manifest}, indent=2) + '\n')
assert all(sha(r / p) == digest for p, digest in sources.items())
print('Rust evidence finalized', len(manifest), 'files', flush=True)
