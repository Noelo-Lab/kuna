from common import *
import datetime, gzip, re, shutil

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
build = json.loads((w / 'build.json').read_text())
gates = json.loads((w / 'gates-passed.json').read_text())
review = json.loads((w / 'breadth/structural-review.json').read_text())
breadth = json.loads((w / 'breadth/record.json').read_text())
speed = json.loads((w / 'timing-result.json').read_text())
native = json.loads((w / 'native/record.json').read_text())
merged = json.loads((w / 'go-pair-merged.json').read_text())
assert merged['merged_tree'] == 'b42d6c0ed0c62d229db647d241fba45b1a8a0159'
assert gates['all_four_gates_passed'] and review['complete'] and speed['all_default_comparisons_passed']
assert gates['release_sha256'] == build['new_cli_sha256'] == native['candidate_cli_sha256'] == speed['binary_sha256']['candidate']
assert sha(r / 'decompiler/target/release/kuna') == build['new_cli_sha256']
sources = json.loads((w / 'production-source-before-validation.json').read_text())
assert all(sha(r / p) == digest for p, digest in sources.items())
assert not subprocess.check_output(['git', 'diff', build['parent_commit'], '--', 'docs/baseline.json'], cwd=r)
out = r / 'docs/features/gostorevalue'
assert not out.exists()
out.mkdir(parents=True)

def copy(source, destination=None):
    source = Path(source)
    destination = out / (destination or source.name)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, destination)

def compressed(source, destination):
    destination = out / destination
    destination.parent.mkdir(parents=True, exist_ok=True)
    with destination.open('wb') as raw, gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0) as sink:
        sink.write(Path(source).read_bytes())

for name in ['build.json', 'gates-passed.json', 'probe-capture.json', 'public-repro.c', 'public-repro.json', 'public-repro-parent.go', 'public-repro-candidate.go', 'timing-plan.json', 'timing-result.json', 'timing-cpus.json', 'timing-cpu-samples.json.gz', 'production-source-before-validation.json', 'parent-source-before-timing.json', 'go-pair-merged.json']:
    copy(w / name)
for name in ['capture.py', 'native.py', 'breadth.py', 'review.py', 'validate.py', 'timing.py', 'bind-timing.py', 'public-repro.py', 'common.py', 'finalize.py']:
    copy(w / name, 'methods/' + name)
for name in ['original.log', 'stages.log', 'tools.log', 'lint.log', 'spec.log', 'catalog.log', 'counters.log', 'workspace-final.log']:
    compressed(w / name, 'logs/' + name + '.gz')
for p in (w / 'native').iterdir():
    if p.suffix in ['.json', '.c', '.go', '.log']:
        copy(p, 'native/' + p.name)
    elif p.suffix == '.stdout':
        compressed(p, 'native/' + p.name + '.gz')
for arm in ['parent', 'candidate']:
    for lang in ['c', 'go', 'rust']:
        compressed(w / (arm + '-' + lang + '.json'), 'probes/' + arm + '-' + lang + '.json.gz')
for p in w.glob('*-samples.json'):
    copy(p, 'speed/' + p.name)
compressed(w / 'breadth/record.json', 'breadth/complete-changes.json.gz')
compressed(w / 'breadth/structural-review.json', 'breadth/structural-review.json.gz')
copy(Path('/tmp/kuna-rust-io-proposal/built-specs.json'), 'compiled-specs.json')

rows = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w / 'workspace-final.log').read_text())
workspace = dict(zip(['passed', 'failed', 'ignored'], [sum(int(row[i]) for row in rows) for i in range(3)]))
workspace['suites'] = len(rows)
assert workspace['failed'] == 0
max_median = max(value for row in speed['records'] for value in row['median_delta_pct'].values())
max_upper = max(value for row in speed['records'] for value in row['confidence_upper_mean_delta_pct'].values())
summary = {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'feature': 'Go same-width integer STORE signedness conversion', 'parent_commit': build['parent_commit'], 'parent_tree': merged['merged_tree'], 'merged_parent_commit': merged['merged_commit'], 'candidate_cli_sha256': build['new_cli_sha256'], 'parent_cli_sha256': build['parent_cli_sha256'], 'gates': gates, 'workspace': workspace, 'native_complete_bodies': native['complete_bodies'], 'native_input_comparisons': native['native_go_input_comparisons'], 'native_function_comparisons': native['function_comparisons'], 'breadth_images': breadth['images'], 'breadth_distinct_functions': breadth['distinct_functions'], 'breadth_language_function_records': breadth['language_function_records'], 'changed_go_records': breadth['changed_go_records'], 'structural_review_pending': review['pending_functions'], 'c_rust_records_unchanged': True, 'new_decompilation_errors': 0, 'new_go_parse_errors': 0, 'max_cpu_or_wall_median_delta_pct': max_median, 'max_95pct_upper_mean_delta_pct': max_upper, 'upper_bound_is_acceptance_gate': False, 'whole_runtime_native_equivalence_claimed': False, 'limits': native['exclusions']}
(out / 'record.json').write_text(json.dumps(summary, indent=2) + '\n')
table = '\n'.join('| ' + row['case'] + ' | ' + ' | '.join(f"{row[key][metric]:+.3f}%" for key in ['median_delta_pct', 'confidence_upper_mean_delta_pct'] for metric in ['cpu_ms', 'wall_ms']) + ' |' for row in speed['records'])
(out / 'analysis.md').write_text(f'''# Go integer stores

The emitter now converts an integer STORE value to the pointed-to integer type when both have the same 1-, 2-, 4- or 8-byte width and opposite signedness. `*p = a + b` becomes `*p = int64(a + b)` when `p` points to `int64` and the operands are `uint64`. This retains the stored bits and evaluates the pointer and value once. Noninteger values, unknown carriers and mismatched widths retain their existing output.

The public reproduction is in `public-repro.c`; exact parent and candidate output is retained beside it. The registered native test covers 33 complete Go bodies on Go 1.22.5 and 1.27.3, compared with C at O0/O2 using wrapping signed arithmetic: {native['function_comparisons']} function/input comparisons, including exact pointer/value callback counts. The unchanged `nested_store` fixture still has an array-pointer rendering error and is explicitly excluded from native compilation.

The {breadth['images']}-image static comparison covers {breadth['distinct_functions']} distinct functions, with Go paired results disabled and enabled. All {breadth['changed_go_records']} changed Go records differ only by integer conversions around complete assignment right-hand sides and corresponding line mappings. C and Rust records are unchanged; no new decompiler or Go parser errors occur. This is structural coverage, not native equivalence for entire runtimes. Complete changed function pairs, capture commands, hashes and the structural review are retained in `breadth/`.

All four repository checks pass, including {workspace['passed']} workspace tests and all original 675 assertions. The five new stage assertions bring this parent’s stage total to {gates['stage_assertions']}. Catalog metadata is unchanged.

The speed plan was declared before the integrated build or sampling: 64 balanced paired rounds per case, three warmups per arm, CPU and wall median deltas at most 5%. The 95% upper bounds on mean deltas report uncertainty and are not acceptance thresholds. Every sample and CPU telemetry is retained. The maximum measured median delta is {max_median:.3f}%; the largest reported upper bound is {max_upper:.3f}%.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` identifies the binaries and parent tree; the parent’s published commit and actual merge commit have identical trees. `manifest.json` hashes every archived evidence file. Sources, both executables and all 154 compiled processor specs were checked before and after timing.
''')
manifest = {str(p.relative_to(out)): sha(p) for p in sorted(out.rglob('*')) if p.is_file()}
(out / 'manifest.json').write_text(json.dumps({'files': manifest}, indent=2) + '\n')
assert all(sha(r / p) == digest for p, digest in sources.items())
print('STORE evidence finalized', len(manifest), 'files', flush=True)
