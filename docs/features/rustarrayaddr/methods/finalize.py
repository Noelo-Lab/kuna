from common import *
import datetime,gzip,lzma,re,shutil
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());native=json.loads((w/'native-record.json').read_text());review=json.loads((w/'breadth/structural-review.json').read_text());speed=json.loads((w/'timing-result.json').read_text());parent=json.loads((w/'rust-array-merged.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and speed['all_default_comparisons_passed'];assert native['status']=='PASS'and native['complete_bodies']==33 and native['native_function_comparisons']==2224 and not native['body_rewrites'];assert native['candidate_cli_sha256']==build['new_cli_sha256']==gates['release_sha256']==speed['binary_sha256']['candidate']==sha(r/'decompiler/target/release/kuna');assert build['parent_commit']==parent['published_commit']and build['parent_tree']==parent['merged_tree'];assert build['parent_cli_sha256']==parent['candidate_cli_sha256'];assert not subprocess.check_output(['git','diff',build['parent_commit'],'--','docs/baseline.json'],cwd=r)
sources=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==digest for p,digest in sources.items());out=r/'docs/features/rustarrayaddr';out.mkdir(exist_ok=False)
def copy(p,name=None):
 p=Path(p);dest=out/(name or p.name);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dest)
def compressed(p,name):
 dest=out/name;dest.parent.mkdir(parents=True,exist_ok=True)
 with Path(p).open('rb')as src,dest.open('wb')as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0)as dst:shutil.copyfileobj(src,dst)
def evidence(p,name):
 p=Path(p)
 if p.stat().st_size>40000 or p.suffix in ['.log','.jsonl','.stdout','.stderr']:compressed(p,name+'.gz')
 else:copy(p,name)
for name in ['build.json','gates-passed.json','native-record.json','native-ast-review.json','native-ast-binding.json','public-repro.c','public-repro.json','public-repro-parent.rs','public-repro-candidate.rs','timing-plan.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','production-source-before-validation.json','parent-source-before-timing.json','rust-array-merged.json','setup-record.json','plan.json']:
 evidence(w/name,name)
for p in sorted(w.glob('*.py')):copy(p,'methods/'+p.name)
for p in sorted(w.glob('*.log')):compressed(p,'logs/'+p.name+'.gz')
for p in sorted(w.glob('public-native-*')):
 if p.suffix in ['.stdout','.stderr','.rs']:evidence(p,'public-native/'+p.name)
for folder in ['native','initial-prototype-0421','prototype-cast-pass']:
 for p in sorted((w/folder).rglob('*')):
  if p.is_file()and p.suffix in ['.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.kuna','.patch']:evidence(p,folder+'/'+str(p.relative_to(w/folder)))
for folder in [w/'constant-probe',Path('/tmp/kuna-rust-wide-store-research')]:
 for p in sorted(folder.iterdir()):
  if p.is_file() and p.suffix in ['.json','.jsonl','.log','.rs','.c','.kuna','.py']:evidence(p,'remaining-array-decay/'+folder.name+'/'+p.name)
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
log=(w/'workspace-final.log').read_text();rows=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',log);workspace=dict(zip(['passed','failed','ignored'],[sum(int(row[i])for row in rows)for i in range(3)]));workspace['suites']=len(rows);assert workspace['failed']==0 and workspace['passed']>=10373 and workspace['suites']>=72;assert 'test rust_array_address::complete_rust_array_addresses_match_native_without_memory_reads ... ok'in log
max_median=max(v for row in speed['records']for v in row['median_delta_pct'].values());max_upper=max(v for row in speed['records']for v in row['confidence_upper_mean_delta_pct'].values())
summary={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'feature':'Raw Rust addresses and integer casts of proven array places','parent_commit':build['parent_commit'],'parent_tree':build['parent_tree'],'merged_parent_commit':parent['merged_commit'],'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'gates':gates,'workspace':workspace,'registered_native_complete_bodies':33,'registered_native_function_comparisons':2224,'parent_native_compile_errors_per_profile':3,'parent_one_past_panic_exit':101,'breadth':review,'max_cpu_or_wall_median_delta_pct':max_median,'max_95pct_upper_mean_delta_pct':max_upper,'upper_bound_is_acceptance_gate':False,'whole_runtime_native_equivalence_claimed':False,'complete_changes_raw_sha256':raw.hexdigest(),'initial_native33_failure_retained':True};(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n')
table='\n'.join('| '+row['case']+' | '+' | '.join(f"{row[key][metric]:+.3f}%"for key in ['median_delta_pct','confidence_upper_mean_delta_pct']for metric in ['cpu_ms','wall_ms'])+' |'for row in speed['records'])
(out/'analysis.md').write_text(f'''# Raw Rust array addresses

The public repro computes a pointer to element 9 of a 9-element array. The parent emits `&mut (*p).values[i as usize]`, whose bounds check panics at this valid one-past address. The candidate takes `&raw mut` of the whole array and applies the instruction's wrapping byte offset, returning byte offset 36 without forming a reference or loading the array. Same-width primitive integer casts of a proven array place likewise take its raw address before conversion.

The proof accepts an implied PTRSUB at the exact start of a nonempty struct array field, or offset zero through a pointer to a nonempty array. It excludes unresolved fields, unions, relative pointers, non-byte-addressed pointers and other array-place forms. PTRADD requires matching 32/64-bit pointer and primitive index widths, a constant byte scale and a bounded integer-context query. Index arithmetic keeps its width before `isize`; each operand is evaluated once. Cast suppression remains diagnostic; array notation off keeps the prior PTRADD path. Array field projection still needs a valid containing object; arbitrary null/dangling field bases are not established.

The registered native test uses 33 complete emitted Rust bodies without rewriting them. At O0/O2, 2224 function/input comparisons agree with C, covering signed/unsigned indices, twelve integer/floating/pointer/boolean element types, valid one-past addresses, nested arrays, callback order, modular integer address arithmetic and 64 shift counts. Every case runs in readable and PROT_NONE memory without reading pointed-to values. Rust uses overflow-checks=off and C uses -fwrapv on x86_64. The parent bundle has two E0606 errors and one E0605 error per profile; the separately compiled public body panics at index 9. The initial prototype fixed 32 forms but retained the left-shift case's array-to-integer cast. Its failed complete 33-body bundle is preserved in `initial-prototype-0421/`; adding raw array decay at integer casts made all 33 pass. Native captures were repeated after the final stage/corpus rebuild, and all current proof binds to CLI `{build['new_cli_sha256']}`.

The candidate was captured fresh across {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records. Exact frozen parent captures from merged array-index PR1464 are hash-verified. All {review['changed_rust_records']} changed Rust records are classified: {review['raw_address_transformations']} raw-address transformations and {review['array_integer_casts']} integer casts. The audit reconstructs only these forms and requires places, indices, integer contexts, existing casts, calls and all surrounding AST to remain identical. Its 22 positive/negative controls reject changed operands, narrowed indices, lost casts, references substituted for raw pointers, nonwrapping arithmetic and duplicated calls. New byte scales and pointer targets are recorded; they come from IR and cannot be inferred from the old AST alone.

C/Go records remain identical. Variable identity, types and storage remain unchanged; all {review['variable_provenance_records']} changed provenance records match the exact union of source-line mappings. There are no new engine or Rust parser errors; {review['existing_rust_parse_failures']} inherited raw parser failures retain diagnostics, with identifier-only normalization for structural review. The upstream probestack binary is inspected statically and never executed. Structural comparison does not establish whole-runtime equivalence. Unresolved global/local array forms, array-to-pointer cast paths, arbitrary field-base validity, scalar STORE-width repair and multiarchitecture native parity remain outside this change. The retained constant-address probe compiles all four bodies and still fails on two unchanged compile-time one-past forms: their IR advances a pointer to a whole array and then needs implicit array-to-pointer decay. Its two constant-index 8 transformations are structurally classified, but no native pass is claimed for the failed bundle. The separate six-body 128-bit-store probe also still fails array-to-pointer casts and a byte-array/scalar assignment; all five changed address portions are structurally classified. Raw captures, rejected initial prototype directives and full compiler diagnostics are preserved under `remaining-array-decay/`.

All four repository checks plus tooling/lint/catalog/counters pass: {workspace['passed']} workspace tests, 675 original assertions and {gates['stage_assertions']} stage assertions. Eight assertions were added; the prior valid array-address assertion now expects raw byte arithmetic under the same passing key. No prior assertion was lost, and the original baseline is unchanged.

The fixed speed plan uses 64 balanced pairs across eight cases, three warmups per arm and a 5% CPU/wall median budget. Every case passes; the largest median delta is {max_median:.3f}%. The largest 95% upper mean bound is {max_upper:.3f}%, reported as uncertainty rather than an acceptance threshold. Every sample and CPU telemetry record is retained. Sources, binaries, fixtures and all 154 compiled specs were verified before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` binds the proof to the exact parent tree and its actual merge. Complete changed records are losslessly compressed and round-trip hash-checked. `manifest.json` hashes the evidence files.
''')
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()};(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n');assert all(sha(r/p)==digest for p,digest in sources.items());print('Array address evidence complete',len(manifest),'files',workspace,flush=True)
