from common import *
import datetime,gzip,lzma,re,shutil
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());native=json.loads((w/'native-record.json').read_text());review=json.loads((w/'breadth/structural-review.json').read_text());speed=json.loads((w/'timing-result.json').read_text());parent=json.loads((w/'rust-array-address-merged.json').read_text());extras=json.loads((w/'extras-record.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and speed['all_default_comparisons_passed'];assert native['status']=='PASS'and native['complete_bodies']==97 and native['native_function_comparisons']==776 and not native['body_rewrites'];assert native['candidate_cli_sha256']==build['new_cli_sha256']==gates['release_sha256']==speed['binary_sha256']['candidate']==sha(r/'decompiler/target/release/kuna')==extras['candidate_cli_sha256'];assert build['parent_commit']==parent['published_commit']and build['parent_tree']==parent['merged_tree'];assert build['parent_cli_sha256']==parent['candidate_cli_sha256'];assert not subprocess.check_output(['git','diff',build['parent_commit'],'--','docs/baseline.json'],cwd=r)
binding=json.loads((w/'native-ast-binding.json').read_text());assert binding['candidate_cli_sha256']==build['new_cli_sha256'];assert binding['native_record_sha256']==sha(w/'native-record.json');assert binding['review_sha256']==sha(w/'native-ast-review.json')and binding['pending']==0
sources=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==digest for p,digest in sources.items());out=r/'docs/features/rustarraydecay';out.mkdir(exist_ok=False)
def copy(p,name=None):
 p=Path(p);dest=out/(name or p.name);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dest)
def compressed(p,name):
 dest=out/name;dest.parent.mkdir(parents=True,exist_ok=True)
 with Path(p).open('rb')as src,dest.open('wb')as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0)as dst:shutil.copyfileobj(src,dst)
def evidence(p,name):
 p=Path(p)
 if p.stat().st_size>40000 or p.suffix in ['.log','.jsonl','.stdout','.stderr']:compressed(p,name+'.gz')
 else:copy(p,name)
for name in ['build.json','gates-passed.json','final-validation-completion.json','native-record.json','native-ast-review.json','native-ast-binding.json','parent-record.json','parent-record-73.json','extras-record.json','public-repro.c','public-repro.json','public-repro-parent.rs','public-repro-candidate.rs','timing-plan.json','timing-plan-73.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','production-source-before-validation.json','parent-source-before-timing.json','rust-array-address-merged.json','setup-record.json','plan.json']:
 evidence(w/name,name)
for p in sorted(w.glob('*.py')):copy(p,'methods/'+p.name)
for p in sorted(w.glob('*.log')):compressed(p,'logs/'+p.name+'.gz')
for p in sorted(w.glob('public-native-*')):
 if p.suffix in ['.stdout','.stderr','.rs','.jsonl']:evidence(p,'public-native/'+p.name)
for folder in ['native','native73','initial-prototype-a547','prototype-pass','extras','interrupted-test-name-validation']:
 for p in sorted((w/folder).rglob('*')):
  if p.is_file()and p.suffix in ['.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.kuna','.patch']:evidence(p,folder+'/'+str(p.relative_to(w/folder)))
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
log=(w/'workspace-final.log').read_text();rows=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',log);workspace=dict(zip(['passed','failed','ignored'],[sum(int(row[i])for row in rows)for i in range(3)]));workspace['suites']=len(rows);assert workspace['failed']==0 and workspace['passed']>=10374 and workspace['suites']>=72;assert 'test rust_array_decay::complete_rust_array_decay_matches_native_pointer_consumers ... ok'in log
max_median=max(v for row in speed['records']for v in row['median_delta_pct'].values());max_upper=max(v for row in speed['records']for v in row['confidence_upper_mean_delta_pct'].values())
summary={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'feature':'Rust array-to-pointer conversions at typed consumers','parent_commit':build['parent_commit'],'parent_tree':build['parent_tree'],'merged_parent_commit':parent['merged_commit'],'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'gates':gates,'workspace':workspace,'registered_native_complete_bodies':97,'registered_native_function_comparisons':776,'supplementary_constant_bodies':4,'supplementary_native_comparisons':8,'parent_native_compile_errors_per_profile':97,'breadth':review,'max_cpu_or_wall_median_delta_pct':max_median,'max_95pct_upper_mean_delta_pct':max_upper,'upper_bound_is_acceptance_gate':False,'whole_runtime_native_equivalence_claimed':False,'complete_changes_raw_sha256':raw.hexdigest(),'initial_native97_failure_retained':True,'wide_six_body_bundle_still_fails':True};(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n')
table='\n'.join('| '+row['case']+' | '+' | '.join(f"{row[key][metric]:+.3f}%"for key in ['median_delta_pct','confidence_upper_mean_delta_pct']for metric in ['cpu_ms','wall_ms'])+' |'for row in speed['records'])
assert review['variable_provenance_records']==0 and review['existing_rust_parse_failures']==0
(out/'analysis.md').write_text(f'''# Rust array-to-pointer conversions

The public repro returns a nine-element array through an integer pointer return type. The parent emits `return *p;`, rejected by Rust with E0308. The candidate emits `return &raw mut *p as *mut i32;`, which compiles and returns the same address as C. Complete parent and candidate bodies and compiler diagnostics are retained without body rewrites.

The conversion applies at typed pointer returns, assignments, STORE values, known call parameters and explicit casts. It shares the raw-address array-place proof: an implied PTRSUB at the exact start of a nonempty struct array field, or offset zero through a pointer to a nonempty array. It requires matching 32/64-bit byte-addressed nonrelative pointer types. A bounded search follows at most seven implied same-width pointer COPY/CAST operations, preserving the address while selecting the actual consumer type. It does not cross loads, calls, integer conversions or explicit variables. Taking the raw address evaluates the place once without creating a reference or loading array values. Unknown parameters and unproved places keep their previous paths; cast suppression remains diagnostic. Array projection requires valid containing storage, and arbitrary null/dangling bases are not covered.

The registered regression uses 97 complete emitted bodies: twelve element types across field returns, whole-array returns, constant one-past addresses, erased pointers, explicit byte-pointer casts, pointer-to-array results, guarded pointer stores and call arguments, plus a callback-order case. The candidate passes 776 C/Rust comparisons at O0/O2 over readable and PROT_NONE mappings and two base locations. Store guard words and callback counts agree exactly. The parent bundle reports 97 E0308/E0605 errors per profile. The initial candidate still failed twelve erased-pointer returns because hidden pointer casts obscured the declared consumer type; the complete failure is retained in `initial-prototype-a547/`. The bounded pointer-chain search fixes those returns. The original 73-case parent fixture is also preserved; it was expanded to 97 before implementation or timing to include explicit casts and pointer-to-array results.

Four previously retained constant-address bodies now compile and pass eight additional C/Rust comparisons across O0/O2. The separate full six-function wide-store bundle still fails at `*p = 0` for a recovered `[u8; 16]` destination. Its prior array-to-pointer errors are repaired, but no native equivalence is claimed for this still-failing six-body bundle. The previous raw-array-address feature's evidence retains its original failures; current full captures and diagnostics are under `extras/`.

All current proof binds to final CLI `{build['new_cli_sha256']}`. The native and structural checks were repeated after the stage/corpus rebuild. Fresh candidate captures cover {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records against hash-verified frozen parent captures from merged PR1467. All {review['changed_rust_records']} changed Rust records are classified, comprising {review['array_pointer_conversions']} array-to-pointer conversions. The paired AST audit allows a raw mutable address plus a pointer cast only at the corresponding original expression. At that location, up to seven pointer casts may be subsumed, with every old and new target recorded. Place expressions, integer casts, calls, evaluation order and surrounding AST remain identical. The 24 positive/negative controls reject changed places, integer cast removal, references, reordered or duplicated calls, unrelated pointer casts and changed memory operations. This establishes structural scope; old syntax alone does not establish the consumer pointer type or whole-runtime correctness.

C/Go records remain identical. Variable identity, type, storage and provenance remain unchanged. No new engine errors occur; neither side has a Rust parser failure among the changed records. Probestack is inspected statically and never executed. Unproved global/local arrays, wide scalar STORE repair and multiarchitecture native equivalence remain outside this fix. Native execution uses x86_64, Rust overflow-checks=off and C -fwrapv.

All four repository checks plus tooling/lint/catalog/counters pass: {workspace['passed']} workspace tests, 675 original assertions and {gates['stage_assertions']} stage assertions. Nine stage assertions were added; no existing passing key was lost and the original baseline is unchanged.

The predeclared speed plan uses 64 balanced pairs across eight cases, three warmups per arm and a 5% CPU/wall median budget. Every case passes; the largest median delta is {max_median:.3f}%. The largest 95% upper mean bound is {max_upper:.3f}%, reported as uncertainty rather than an acceptance threshold. All samples and CPU telemetry are retained. Sources, binaries, fixtures and all 154 compiled specs were verified before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` binds the proof to the parent tree and actual merge. Complete changed records are losslessly compressed and round-trip hash-checked; `manifest.json` hashes the evidence files.
''')
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()};(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n');assert all(sha(r/p)==digest for p,digest in sources.items());print('Array decay evidence complete',len(manifest),'files',workspace,flush=True)
