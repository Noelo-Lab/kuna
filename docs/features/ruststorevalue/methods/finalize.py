from common import *
import datetime,gzip,lzma,re,shutil
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());review=json.loads((w/'breadth/structural-review.json').read_text());native=json.loads((w/'native-record.json').read_text());speed=json.loads((w/'timing-result.json').read_text());merged=json.loads((w/'rust-integers-merged.json').read_text())
extra_root=Path('/tmp/kuna-rust-store-research/width-native');extra=json.loads((extra_root/'record.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and speed['all_default_comparisons_passed']and native['status']=='PASS'
assert build['new_cli_sha256']==gates['release_sha256']==native['candidate_cli_sha256']==speed['binary_sha256']['candidate']==sha(r/'decompiler/target/release/kuna')
assert merged['merged_tree']==build['parent_tree']=='08cd977cf6425295c85385bd13701e038ac04243'
assert native['complete_bodies']==33 and native['native_function_comparisons']==21384 and not native['body_rewrites']
assert extra['candidate_function_comparisons']==1728 and not extra['body_rewrites']
for row in extra['records']:
 if row['arm']=='candidate':
  assert row['cli_sha256']==build['new_cli_sha256']and all(outcome['compile_exit']==0 and outcome['matches_reference']for outcome in row['outcomes'])
sources=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==digest for p,digest in sources.items())
assert not subprocess.check_output(['git','diff',build['parent_commit'],'--','docs/baseline.json'],cwd=r)
out=r/'docs/features/ruststorevalue';assert not out.exists();out.mkdir(parents=True)
def copy(p,name=None):
 p=Path(p);q=out/(name or p.name);q.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,q)
def compressed(p,name):
 q=out/name;q.parent.mkdir(parents=True,exist_ok=True)
 with Path(p).open('rb')as src,q.open('wb')as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0)as dst:shutil.copyfileobj(src,dst)
def evidence(p,name):
 p=Path(p)
 if p.stat().st_size>40000 or p.suffix in {'.log','.jsonl','.stdout','.stderr'}:compressed(p,name+'.gz')
 else:copy(p,name)
for name in ['build.json','gates-passed.json','native-record.json','parent-record.json','public-repro.c','public-repro.json','public-repro-parent.rs','public-repro-candidate.rs','timing-plan.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','production-source-before-validation.json','parent-source-before-timing.json','rust-integers-merged.json','plan.json','audit-selftest-input.json','audit-selftest-output.json']:
 copy(w/name)
for name in ['common.py','native.py','check-parent.py','prepare-harness.py','prepare.py','validate.py','run-validation.py','breadth.py','review.py','prepare-audit.py','audit-selftest.py','timing.py','bind-timing.py','public-repro.py','finalize.py']:
 copy(w/name,'methods/'+name)
for name in ['original.log','stages.log','tools.log','lint.log','spec.log','catalog.log','counters.log','workspace-final.log','final-build.log','harness-build.log','timing.log','initial-missing-harness-validation-coordinator.log','initial-missing-harness-prepare-coordinator.log','initial-missing-harness-stage-focused.log','audit-selftest-final-coordinator.log','assignment-review-v2.log','structural-review-v2.log']:
 compressed(w/name,'logs/'+name+'.gz')
for p in w.iterdir():
 if p.is_file()and (p.name.startswith(('parent-','candidate-','reference-'))or p.name in ['candidate.rs','parent.rs','harness.rs','harness.c','cases.json']):
  if p.suffix in {'.json','.jsonl','.log','.stdout','.stderr','.rs','.c'}:evidence(p,'native/'+p.name)
for p in extra_root.iterdir():
 if p.is_file()and p.suffix in {'.json','.rs','.c','.kuna','.stdout','.stderr'}:evidence(p,'supplementary-width/'+p.name)
copy(Path('/tmp/kuna-rust-store-research/width-native.py'),'supplementary-width/driver.py')
for p in w.glob('*-samples.json'):copy(p,'speed/'+p.name)
(out/'breadth').mkdir(exist_ok=True)
with (w/'breadth/record.json').open('rb')as src,lzma.open(out/'breadth/complete-changes.json.xz','wb',preset=6)as dst:shutil.copyfileobj(src,dst)
for name in ['assignment-review.json','assignment-review-v2.json','structural-review.json']:compressed(w/'breadth'/name,'breadth/'+name+'.gz')
for name in ['Cargo.toml','Cargo.lock','src/main.rs','before-keyword-rhs-fix.rs']:copy(w/'ast-audit'/name,'methods/ast-audit/'+name)
copy(Path('/tmp/kuna-rust-io-proposal/built-specs.json'),'compiled-specs.json')
rows=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',(w/'workspace-final.log').read_text());workspace=dict(zip(['passed','failed','ignored'],[sum(int(row[i])for row in rows)for i in range(3)]));workspace['suites']=len(rows);assert workspace=={'passed':10362,'failed':0,'ignored':39,'suites':72}
max_median=max(value for row in speed['records']for value in row['median_delta_pct'].values());max_upper=max(value for row in speed['records']for value in row['confidence_upper_mean_delta_pct'].values())
summary={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'feature':'Rust same-width integer STORE conversion','parent_commit':build['parent_commit'],'parent_tree':merged['merged_tree'],'merged_parent_commit':merged['merged_commit'],'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'gates':gates,'workspace':workspace,'registered_native_complete_bodies':33,'registered_native_function_comparisons':21384,'supplementary_native_complete_bodies':48,'supplementary_native_function_comparisons':1728,'breadth':review,'max_cpu_or_wall_median_delta_pct':max_median,'max_95pct_upper_mean_delta_pct':max_upper,'upper_bound_is_acceptance_gate':False,'whole_runtime_native_equivalence_claimed':False,'exclusions':native['exclusions']}
(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n')
table='\n'.join('| '+row['case']+' | '+' | '.join(f"{row[key][metric]:+.3f}%"for key in ['median_delta_pct','confidence_upper_mean_delta_pct']for metric in ['cpu_ms','wall_ms'])+' |'for row in speed['records'])
(out/'analysis.md').write_text(f'''# Rust integer stores

The emitter converts a STORE value to the rendered pointer's integer type when both primitive widths match but signedness differs. The public example changes `*p = value;` to `*p = value as i64;` for an `i64` pointer and a `u64` value. Supported widths are 1, 2, 4 and 8 bytes. The shared integer conversion helper preserves literal-width context and declines enums, noninteger values and unsupported widths. Cast suppression remains honored. The pointer and complete value expression are each emitted once.

The registered test compiles 33 complete emitted Rust bodies at O0/O2 and compares them with C: 21,384 function/input comparisons across 324 boundary pairs. It checks every destination and guard byte, sibling fields, nested fields, and exact pointer/value callback counts. Rust uses `overflow-checks=off`; C uses `-fwrapv`. No emitted body is rewritten. The inherited `array_store` raw-pointer indexing error remains excluded from native compilation, with both versions and compiler diagnostics retained.

Another 48 complete widening/narrowing bodies cover all distinct 1/2/4/8-byte width pairs and both signedness choices. Their 1,728 supplementary comparisons also match C and preserve guard bytes. These probes are archived separately and are not registered cargo tests. They demonstrate ordinary compiler-generated stores, not arbitrary inconsistent inferred types, unknown 128-bit values or an independent repair of memory-operation width.

The static comparison covers {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records. All {review['changed_rust_records']} changed Rust records pass the strict assignment audit: removing newly added primitive casts or exact integer identity wrappers around complete assignment right-hand sides restores the parent syntax tree, including every existing cast, destination and other expression. C and Go records are unchanged. No new decompiler or parser failures appear. The {review['existing_rust_parse_failures']} inherited parser failures use identifier-only normalization for structural review; original bodies and errors remain retained. The upstream `probestack` image was inspected statically and never executed.

Thirteen audit controls cover whole values, retained casts, literal context, keyword fields, changed destinations, partial casts, lost casts and reordered/duplicated calls. The initial report left four `.fn as i64` records pending because the review helper mistook `fn` before `as` for a declaration. The corrected helper and its new control classify those records without a production change. This structural comparison does not establish native equivalence or type correctness for whole runtime images.

All four repository checks pass: {workspace['passed']} workspace tests, all original 675 assertions and {gates['stage_assertions']} stage assertions. Five stage assertions were added without losing prior ones; the original baseline and catalog metadata are unchanged. Production sources stayed fixed throughout validation and timing.

The declared speed plan uses 64 balanced pairs per case and three warmups per arm. Every CPU and wall median delta is within the 5% budget. The largest measured median delta is {max_median:.3f}%; the largest 95% upper mean bound is {max_upper:.3f}%. Bounds describe uncertainty and are not acceptance thresholds. Every raw sample and CPU telemetry record is retained.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` identifies the candidate and the exact parent tree, which matches its actual merge. Both binaries and all 154 compiled processor specs were checked before and after timing. `manifest.json` hashes the evidence archive. Raw-pointer indexing, arbitrary store-width/type inconsistencies, and Rust enum recovery remain separate work.
''')
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()};(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n');assert all(sha(r/p)==digest for p,digest in sources.items());print('Rust STORE evidence finalized',len(manifest),'files',flush=True)
