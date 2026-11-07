from common import *
import datetime,gzip,lzma,re,shutil
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
read=lambda name:json.loads((w/name).read_text())
build=read('build.json');gates=read('gates-passed.json');native=read('native-record.json');wide=read('wide-native-record.json');review=read('breadth/structural-review.json');speed=read('timing-result.json');parent=read('rust-array-decay-merged.json')
assert native['default_off_all_records_unchanged'] and review['default_off_all_records_unchanged']
assert gates['all_four_gates_passed'] and gates['additional_tools_lint_catalog_counters_passed'] and review['complete'] and speed['all_default_comparisons_passed']
assert native['status']=='PASS' and native['complete_bodies']==13 and native['native_function_comparisons']==416 and not native['body_rewrites']
assert wide['status']=='PASS' and wide['complete_bodies']==6 and wide['native_function_comparisons']==2304 and not wide['body_rewrites']
assert native['candidate_cli_sha256']==wide['candidate_cli_sha256']==build['new_cli_sha256']==gates['release_sha256']==speed['binary_sha256']['candidate']==sha(r/'decompiler/target/release/kuna')
assert build['parent_commit']==parent['published_commit'] and build['parent_tree']==parent['merged_tree'] and build['parent_cli_sha256']==parent['candidate_cli_sha256']
assert not subprocess.check_output(['git','diff',build['parent_commit'],'--','docs/baseline.json'],cwd=r)
binding=read('native-ast-binding.json');audit=read('native-ast-review.json')
assert binding['candidate_cli_sha256']==build['new_cli_sha256'] and binding['native_record_sha256']==sha(w/'native-record.json') and binding['wide_native_record_sha256']==sha(w/'wide-native-record.json')
assert binding['input_sha256']==sha(w/'native-ast-input.json') and binding['review_sha256']==sha(w/'native-ast-review.json') and binding['pending']==audit['pending']==0 and binding['changed_records']==audit['changed_records']==8 and audit['self_checks']==20
assert review['changed_rust_records']==14 and review['array_zero_stores']==28 and review['pending_records']==review['variable_provenance_records']==review['existing_rust_parse_failures']==review['new_rust_parse_errors']==0 and review['c_go_records_unchanged']
log=(w/'workspace-final.log').read_text();rows=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',log)
workspace=dict(zip(['passed','failed','ignored'],[sum(int(row[i])for row in rows)for i in range(3)]));workspace['suites']=len(rows)
assert workspace=={'passed':10375,'failed':0,'ignored':39,'suites':72},workspace
assert 'test rust_array_constant::complete_rust_zero_array_stores_match_native_memory ... ok' in log
assert not(r/'decompiler/target/debug').exists()
sources=read('production-source-before-validation.json');assert all(sha(r/p)==digest for p,digest in sources.items())
out=r/'docs/features/rustarrayzero';out.mkdir(exist_ok=False)
def copy(p,name=None):
 p=Path(p);dest=out/(name or p.name);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dest)
def compressed(p,name):
 dest=out/name;dest.parent.mkdir(parents=True,exist_ok=True)
 with Path(p).open('rb')as src,dest.open('wb')as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0)as dst:shutil.copyfileobj(src,dst)
def evidence(p,name):
 p=Path(p)
 if p.suffix=='.gz':copy(p,name)
 elif p.stat().st_size>40000 or p.suffix in ['.log','.jsonl','.stdout','.stderr']:compressed(p,name+'.gz')
 else:copy(p,name)
for name in ['build.json','gates-passed.json','native-record.json','wide-native-record.json','native-ast-review.json','native-ast-input.json','native-ast-binding.json','parent-record.json','public-repro.c','public-repro.json','public-repro-parent.rs','public-repro-candidate.rs','timing-plan.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','production-source-before-validation.json','parent-source-before-timing.json','rust-array-decay-merged.json','setup-record.json','plan.json','parent-commit.json','prototype-build.json']:
 evidence(w/name,name)
for p in sorted(w.glob('*.py')):copy(p,'methods/'+p.name)
for p in sorted(w.glob('*.log')):compressed(p,'logs/'+p.name+'.gz')
for p in sorted(w.glob('public-native-*')):
 if p.suffix in ['.stdout','.stderr','.rs','.jsonl']:evidence(p,'public-native/'+p.name)
for name in ['probe.c','helpers.c','harness.c','harness.rs','bodies.rs','parent-full.rs','candidate-full.rs','capture.json']:
 evidence(w/name,'native/'+name)
for pattern in ['parent-*.json','candidate-*.json','parent-*.jsonl','candidate-*.jsonl','candidate-*.stdout','candidate-*.stderr','reference-*.stdout']:
 for p in sorted(w.glob(pattern)):evidence(p,'native/'+p.name)
for folder in ['wide-native','prototype-pass','initial-declared-buffer-reference','failed-default-on']:
 for p in sorted((w/folder).rglob('*')):
  if not p.is_file():continue
  if folder=='failed-default-on' and 'breadth' in p.relative_to(w/folder).parts and p.name not in ['record.json','array-review.json','metadata-review.json','structural-review.json']:continue
  if p.suffix in ['.json','.jsonl','.gz','.log','.stdout','.stderr','.rs','.c','.kuna','.patch','.py','.toml','.xml','.md']:evidence(p,folder+'/'+str(p.relative_to(w/folder)))
for p in sorted(w.glob('*-samples.json')):copy(p,'speed/'+p.name)
(out/'breadth').mkdir();src=w/'breadth/record.json';archive=out/'breadth/complete-changes.json.xz'
with src.open('rb')as f,lzma.open(archive,'wb',preset=6)as dst:shutil.copyfileobj(f,dst)
raw=hashlib.sha256()
with lzma.open(archive,'rb')as f:
 while block:=f.read(1024*1024):raw.update(block)
assert raw.hexdigest()==sha(src)
for name in ['array-review.json','metadata-review.json','structural-review.json']:compressed(w/'breadth'/name,'breadth/'+name+'.gz')
for name in ['Cargo.toml','Cargo.lock','src/main.rs']:copy(w/'ast-audit'/name,'methods/ast-audit/'+name)
copy(Path('/tmp/kuna-rust-io-proposal/built-specs.json'),'compiled-specs.json')
max_default_median=max(v for row in speed['records']if row['case']in speed['plan']['required_cases']for v in row['median_delta_pct'].values());max_enabled_median=max(v for row in speed['records']if row['case']in speed['plan']['enabled_cases']for v in row['median_delta_pct'].values());max_median=max(v for row in speed['records']for v in row['median_delta_pct'].values());max_upper=max(v for row in speed['records']for v in row['confidence_upper_mean_delta_pct'].values())
summary={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'feature':'Opt-in exact-width zero values for Rust integer byte-array stores','option':'rustarrayzero','default_on':False,'speed_forced_off':True,'initial_default_on_speed_failure':read('failed-default-on/failure.json'),'max_default_off_median_delta_pct':max_default_median,'max_final_enabled_median_delta_pct':max_enabled_median,'parent_commit':build['parent_commit'],'parent_tree':build['parent_tree'],'merged_parent_commit':parent['merged_commit'],'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'gates':gates,'workspace':workspace,'registered_native_complete_bodies':13,'registered_native_function_comparisons':416,'supplementary_wide_complete_bodies':6,'supplementary_wide_native_comparisons':2304,'parent_native_compile_errors_per_profile':7,'breadth':review,'max_cpu_or_wall_median_delta_pct':max_median,'max_95pct_upper_mean_delta_pct':max_upper,'upper_bound_is_acceptance_gate':False,'whole_runtime_native_equivalence_claimed':False,'complete_changes_raw_sha256':raw.hexdigest(),'superseded_reference_harness_retained':True}
(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n')
table='\n'.join('| '+row['case']+' | '+' | '.join(f"{row[key][metric]:+.3f}%"for key in ['median_delta_pct','confidence_upper_mean_delta_pct']for metric in ['cpu_ms','wall_ms'])+' |'for row in speed['records'])
(out/'analysis.md').write_text(f"""# Opt-in Rust zero values for byte-array stores

A wide zero store can recover a byte-array destination and emit `*p = 0;`, which Rust rejects with E0308. The candidate emits `*p = [0; 16];` for the public repro. Complete parent and candidate bodies, compiler diagnostics and native output are retained without body rewrites.

The STORE value helper accepts only a constant zero whose varnode width exactly matches a nonempty array destination. Elements must be one-byte signed, unsigned or unknown integers with alignment one, excluding enums, and the element count must equal the destination size. It emits a contextually typed `[0; N]`. Nonzero values, nonconstants, other element types and inconsistent widths retain their previous handling. The destination expression, evaluation count and alignment remain unchanged. This avoids inventing a more strongly aligned scalar pointer. Existing diagnostic cast suppression remains effective. The correction requires `--option rustarrayzero on`. The option is off in every preset after the initial default-on build failed the predeclared speed budget.

The registered regression contains 13 complete functions: seven zero stores and six unchanged scalar/copy controls. All 416 C/Rust comparisons pass at O0/O2 over eight source/input patterns and two storage locations. Each comparison checks all 128 bytes, including guards, and callback counts. Scalar 128 storage is aligned; the byte-array stores deliberately use unaligned offsets 17 and 33. The C reference uses malloc storage and memcpy from a genuine 128-bit source. Its initial declared-byte-buffer harness is preserved under `initial-declared-buffer-reference/` as superseded evidence, not the accepted aliasing proof. Rust record padding, size and alignment are asserted to match C. The parent full bundle fails with seven E0308 errors per profile. Six control functions and all C/Go records remain identical.

The previously retained six-function wide-store bundle now also compiles in full and passes 2304 C/Rust comparisons at O0/O2. It covers 64 low/high input combinations, two disjoint destinations and exact source/destination aliasing, comparing all memory/guard bytes. Carry boundaries include complete 128-bit wrap. Only `wide_zero` changes from the parent; the other five bodies remain identical. Prior failures remain in the parent feature's evidence. This establishes these complete bodies, not arbitrary scalar STORE-width or type-recovery correctness.

Current proof binds to final CLI `{build['new_cli_sha256']}`. Native and structural checks were repeated after the stage/corpus rebuild. The successful earlier prototype is preserved separately. Fresh candidate captures cover {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records against hash-verified frozen parent captures from merged PR1471. Every default-off function record in all three languages is identical to the parent. The following changes refer to explicit `rustarrayzero on`. All {review['changed_rust_records']} changed Rust records are classified, comprising {review['array_zero_stores']} zero-array assignments. C/Go records and variable identity, type, storage and provenance are unchanged. No new engine errors occur; neither side has parser failures among the changed Rust records.

The recursive paired AST audit permits only assignment RHS literal zero to become a positive-length repeat of literal zero. Assignment destinations, casts, calls, ordering and surrounding AST stay identical. Twenty positive/negative controls reject changed targets or casts, nonzero/nonconstant expressions, reordered or duplicated effects, boolean/float/suffixed literals, dynamic/zero lengths and changed return expressions. The same audit classifies all eight changed native records across both bundles with no pending cases. Counts are recorded, but old scalar syntax alone cannot prove the storage width; the production IR predicates and native memory comparison provide that separate evidence.

Probestack is inspected statically and never executed. Arbitrary invalid pointers, other array element types, nonzero values, inconsistent scalar widths and multiarchitecture native equivalence remain outside this fix. Native execution uses x86_64, Rust overflow-checks=off and C -fwrapv; the small public zero-store repro also passes with Rust's default overflow checks.

All four repository checks plus tooling/lint/catalog/counters pass: {workspace['passed']} workspace tests, 675 original assertions and {gates['stage_assertions']} stage assertions. Eight stage assertions cover off/on behavior and diagnostic cast suppression. The catalog total assertion was renamed for 378 entries; no existing passing behavior was lost and the original 675 baseline is unchanged.

The initial default-on build failed the fixed 64-pair plan: the small C control measured +5.075% CPU median against the 5% budget, with -0.119% wall median. That path does not call the new Rust helper, so no causal explanation or noise dismissal is asserted. Every raw sample, source snapshot, native proof and broad comparison is retained under `failed-default-on/`. The source was then changed to a named default-off option; the unchanged default-on implementation was not retimed.\n\nThe final gated build has a separately predeclared plan: 64 balanced pairs for each of 16 comparisons, three warmups per arm. Eight compare default-off against the frozen parent and must meet the 5% CPU/wall median budget; their maximum delta is {max_default_median:.3f}%. Eight measure explicit on versus off on the same final binary; their maximum median delta is {max_enabled_median:.3f}%. The option remains off regardless of those later enabled measurements. The largest 95% upper mean bound across all final comparisons is {max_upper:.3f}%, reported as uncertainty rather than an acceptance threshold. All samples and CPU telemetry are retained. Sources, binaries, fixtures and all 154 compiled specs were verified before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` binds the proof to the parent tree and actual merge. Complete changed records are losslessly compressed and round-trip hash-checked; `manifest.json` hashes the evidence files.
""")
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()};(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n')
assert all(sha(r/p)==digest for p,digest in sources.items())
print('Array zero evidence complete',len(manifest),'files',workspace,flush=True)
