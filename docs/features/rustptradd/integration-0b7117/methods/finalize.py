from common import *
import datetime,gzip,lzma,re,shutil
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());review=json.loads((w/'breadth/structural-review.json').read_text());native=json.loads((w/'native-record.json').read_text());speed=json.loads((w/'timing-result.json').read_text());parent=json.loads((w/'parent-main.json').read_text());integration=json.loads((w/'integration.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and speed['all_default_comparisons_passed']and native['status']=='PASS';assert build['new_cli_sha256']==gates['release_sha256']==native['candidate_cli_sha256']==speed['binary_sha256']['candidate']==sha(r/'decompiler/target/release/kuna');assert build['parent_tree']==parent['merged_tree']=='5a6e71ae41be880ad7e757155c39762377d64d17';assert native['complete_bodies']==57 and native['native_function_comparisons']==29544 and not native['body_rewrites']
sources=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==h for p,h in sources.items());assert not subprocess.check_output(['git','diff',build['parent_commit'],'--','docs/baseline.json'],cwd=r)
out=r/'docs/features/rustptradd/integration-0b7117';out.mkdir(exist_ok=False)
def copy(p,name=None):
 p=Path(p);dest=out/(name or p.name);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dest)
def compressed(p,name):
 dest=out/name;dest.parent.mkdir(parents=True,exist_ok=True)
 with Path(p).open('rb')as src,dest.open('wb')as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0)as dst:shutil.copyfileobj(src,dst)
def evidence(p,name):
 p=Path(p)
 if p.stat().st_size>40000 or p.suffix in ['.log','.jsonl','.stdout','.stderr']:compressed(p,name+'.gz')
 else:copy(p,name)
for name in ['build.json','gates-passed.json','native-record.json','public-repro.c','public-repro.json','public-repro-parent.rs','public-repro-candidate.rs','timing-plan.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','production-source-before-validation.json','parent-source-before-timing.json','parent-main.json','integration.json','prior-proof.json','plan.json']:
 copy(w/name)
for p in sorted(w.glob('*.py')):copy(p,'methods/'+p.name)
for p in sorted(w.glob('*.log')):compressed(p,'logs/'+p.name+'.gz')
for p in sorted((w/'native').iterdir()):
 if p.is_file()and p.suffix in ['.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.kuna']:evidence(p,'native/'+p.name)
for folder,count in [('pointer-cast-native',40),('pointer-cast-memory',1620)]:
 extra=json.loads((w/folder/'record.json').read_text());assert extra['status']=='PASS'and extra['comparisons']==count and not extra['registered']and not extra['body_rewrites'];assert all(row['cli_sha256']==build['new_cli_sha256']for row in extra['records']if row.get('arm')=='candidate')
 for p in sorted((w/folder).iterdir()):
  if p.is_file()and p.suffix in ['.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.kuna']:evidence(p,'supplementary/'+folder+'/'+p.name)
for p in sorted(w.glob('controls*')):
 if p.is_file()and p.suffix in ['.c','.rs','.json','.jsonl','.kuna']:evidence(p,'controls/'+p.name)
for p in sorted(w.glob('*-samples.json')):copy(p,'speed/'+p.name)
(out/'breadth').mkdir();src=w/'breadth/record.json';archive=out/'breadth/complete-changes.json.xz'
with src.open('rb')as f,lzma.open(archive,'wb',preset=6)as dst:shutil.copyfileobj(f,dst)
raw=hashlib.sha256()
with lzma.open(archive,'rb')as f:
 while block:=f.read(1024*1024):raw.update(block)
assert raw.hexdigest()==sha(src)
for name in ['pointer-review-v4.json','metadata-review.json','structural-review.json']:compressed(w/'breadth'/name,'breadth/'+name+'.gz')
for name in ['Cargo.toml','Cargo.lock','src/main.rs']:copy(Path('/tmp/kuna-rust-pointer-index/ast-audit-v4')/name,'methods/ast-audit-v4/'+name)
copy(Path('/tmp/kuna-rust-io-proposal/built-specs.json'),'compiled-specs.json')
log=(w/'workspace-final.log').read_text();rows=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',log);workspace=dict(zip(['passed','failed','ignored'],[sum(int(row[i])for row in rows)for i in range(3)]));workspace['suites']=len(rows);assert workspace['failed']==0 and workspace['passed']>=10363 and workspace['suites']>=72;assert 'test rust_pointer_index::complete_rust_pointer_indices_match_native_memory_addresses_and_calls ... ok'in log
max_median=max(v for row in speed['records']for v in row['median_delta_pct'].values());max_upper=max(v for row in speed['records']for v in row['confidence_upper_mean_delta_pct'].values())
summary={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'feature':'Rust raw-pointer PTRADD indexing integrated with main0b7117 persistcalls','parent_commit':build['parent_commit'],'parent_tree':parent['merged_tree'],'merged_parent_commit':parent['merged_commit'],'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'gates':gates,'workspace':workspace,'registered_native_complete_bodies':57,'registered_native_function_comparisons':29544,'supplementary_native_complete_bodies':5,'supplementary_native_function_comparisons':1660,'breadth':review,'max_cpu_or_wall_median_delta_pct':max_median,'max_95pct_upper_mean_delta_pct':max_upper,'upper_bound_is_acceptance_gate':False,'whole_runtime_native_equivalence_claimed':False,'complete_changes_raw_sha256':raw.hexdigest(),'prior_milestone':integration['milestone_head'],'array_exclusions':json.loads((w/'plan.json').read_text())['remaining_array_gaps']};(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n')
table='\n'.join('| '+row['case']+' | '+' | '.join(f"{row[key][metric]:+.3f}%"for key in ['median_delta_pct','confidence_upper_mean_delta_pct']for metric in ['cpu_ms','wall_ms'])+' |'for row in speed['records'])
(out/'analysis.md').write_text(f'''# Rust raw-pointer indexing on the integrated main tree

The public repro changes `return p[i];` for a raw pointer into `return *p.wrapping_byte_offset((i as isize).wrapping_mul(4));`. The emitter preserves PTRADD's byte scale and modular address arithmetic, uses one dereference for a load or store, and leaves an address as a raw pointer. Literal-dependent indices retain their integer width. Base and index expressions are evaluated once; differing output pointee types receive a pointer cast. Matching 32/64-bit raw pointer/index widths and byte-addressed pointers are required. Constant bases, relative pointers, implied PTRSUB/COPY bases and array-valued sources keep their prior paths, as does cast suppression.

Main advanced to `{build['parent_commit']}` with the persistcalls dataflow change before this fix was published. The earlier completed proof is retained in [the parent archive](../analysis.md). This archive validates both freshly built binaries on the new parent. Only the corpus counter and stage baseline conflicted; the count includes both new stage files and the baseline was regenerated. The pointer implementation itself is unchanged.

The registered test executes 57 complete emitted Rust bodies at O0/O2 against C, making 29,544 function/input comparisons. It checks all destination and guard bytes, signed and unsigned element widths 1/2/4/8, valid positive/negative indices, record fields, modular address bits, callback order and all 64 shift counts. Five supplementary complete bodies add 1,660 comparisons for pointer-result casts and scaled byte memory accesses. They are archived separately and are not registered cargo tests. Rust uses `overflow-checks=off`; C uses `-fwrapv`. No emitted body is rewritten. A fresh nine-body compile survey retains two unresolved array-valued index failures: an array field and the inner index of a nested array.

Both arms were captured fresh across {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records. All {review['changed_rust_records']} changed Rust records pass the AST audit; C/Go records are unchanged. The audit restores original indexing/address syntax while preserving the base, index, dereference, existing casts and surrounding AST. Newly explicit pointer-result casts are recorded. Instruction scales are recorded, but the old AST alone cannot independently establish them. Twenty-six controls cover preserved and added casts in the same and different statements, changed operands, lost dereferences, nonwrapping operations and duplicated calls.

Variable names, types and storage remain identical. All {review['variable_provenance_records']} changed variable provenance records have addresses independently checked as the exact union of their source-line mappings. There are no new decompiler or parser errors. The {review['existing_rust_parse_failures']} inherited parser failures retain their raw diagnostics and use identifier normalization only for structural review. The upstream `probestack` fixture was decompiled statically and never executed. Structural comparison does not establish native equivalence or full type correctness for runtime images.

All four repository checks and tooling/lint/catalog/counter checks pass: {workspace['passed']} workspace tests, all original 675 assertions, and {gates['stage_assertions']} stage assertions. Eight stage assertions were added relative to the integrated main baseline without losing prior assertions. The original baseline remains unchanged.

The fresh speed plan uses 64 balanced pairs across eight cases, with three warmups per arm. Every CPU and wall median delta is within 5%; the largest is {max_median:.3f}%. The largest 95% upper mean bound is {max_upper:.3f}%; bounds describe uncertainty and are not acceptance thresholds. Every raw sample and CPU telemetry record is archived. Sources, binaries and all 154 compiled specs were checked before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` binds this proof to the exact current-main parent tree. The complete changed records are compressed losslessly and round-trip hash-checked. `manifest.json` hashes this archive. Array indices, multiarchitecture native proof, Go aggregate ABI and Rust enum recovery remain separate work.
''')
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()};(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n');assert all(sha(r/p)==digest for p,digest in sources.items());print('Integrated evidence complete',len(manifest),'files',workspace,flush=True)
