from common import *
import datetime,gzip,lzma,re,shutil
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());native=json.loads((w/'native-record.json').read_text());review=json.loads((w/'breadth/structural-review.json').read_text());speed=json.loads((w/'timing-result.json').read_text());parent=json.loads((w/'rust-pointer-merged.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and speed['all_default_comparisons_passed'];assert native['status']=='PASS'and native['complete_bodies']==45 and native['native_function_comparisons']==23112 and not native['body_rewrites'];assert native['candidate_cli_sha256']==build['new_cli_sha256']==gates['release_sha256']==speed['binary_sha256']['candidate']==sha(r/'decompiler/target/release/kuna');assert build['parent_commit']==parent['published_commit']and build['parent_tree']==parent['merged_tree'];assert build['parent_cli_sha256']==parent['candidate_cli_sha256'];assert not subprocess.check_output(['git','diff',build['parent_commit'],'--','docs/baseline.json'],cwd=r)
sources=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==digest for p,digest in sources.items());out=r/'docs/features/rustarrayindex';out.mkdir(exist_ok=False)
def copy(p,name=None):
 p=Path(p);dest=out/(name or p.name);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dest)
def compressed(p,name):
 dest=out/name;dest.parent.mkdir(parents=True,exist_ok=True)
 with Path(p).open('rb')as src,dest.open('wb')as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0)as dst:shutil.copyfileobj(src,dst)
def evidence(p,name):
 p=Path(p)
 if p.stat().st_size>40000 or p.suffix in ['.log','.jsonl','.stdout','.stderr']:compressed(p,name+'.gz')
 else:copy(p,name)
for name in ['build.json','gates-passed.json','native-record.json','native-ast-review.json','native-ast-binding.json','public-repro.c','public-repro.json','public-repro-parent.rs','public-repro-candidate.rs','timing-plan.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','production-source-before-validation.json','parent-source-before-timing.json','rust-pointer-merged.json','setup-record.json','plan.json']:
 evidence(w/name,name)
for p in sorted(w.glob('*.py')):copy(p,'methods/'+p.name)
for p in sorted(w.glob('*.log')):compressed(p,'logs/'+p.name+'.gz')
for folder in ['validated-native','prototype-proof']:
 for p in sorted((w/folder).rglob('*')):
  if p.is_file()and p.suffix in ['.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.kuna']:evidence(p,folder+'/'+str(p.relative_to(w/folder)))
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
address=Path('/tmp/kuna-rust-array-address');gap=json.loads((address/'record.json').read_text());assert gap['parent_cli_sha256']==build['new_cli_sha256']and not gap['body_rewrites'];assert gap['records'][-1]['index']==9 and gap['records'][-1]['candidate']['exit']==101
for p in sorted(address.iterdir()):
 if p.is_file()and p.suffix in ['.json','.log','.stdout','.stderr','.rs','.c']:evidence(p,'remaining-array-address/'+p.name)
log=(w/'workspace-final.log').read_text();rows=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',log);workspace=dict(zip(['passed','failed','ignored'],[sum(int(row[i])for row in rows)for i in range(3)]));workspace['suites']=len(rows);assert workspace['failed']==0 and workspace['passed']>=10372 and workspace['suites']>=72;assert 'test rust_array_index::complete_rust_array_indices_match_native_memory_addresses_and_calls ... ok'in log
max_median=max(v for row in speed['records']for v in row['median_delta_pct'].values());max_upper=max(v for row in speed['records']for v in row['confidence_upper_mean_delta_pct'].values())
summary={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'feature':'Rust integer array indices as usize','parent_commit':build['parent_commit'],'parent_tree':build['parent_tree'],'merged_parent_commit':parent['merged_commit'],'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'gates':gates,'workspace':workspace,'registered_native_complete_bodies':45,'registered_native_function_comparisons':23112,'parent_native_compile_errors_per_profile':41,'breadth':review,'max_cpu_or_wall_median_delta_pct':max_median,'max_95pct_upper_mean_delta_pct':max_upper,'upper_bound_is_acceptance_gate':False,'whole_runtime_native_equivalence_claimed':False,'complete_changes_raw_sha256':raw.hexdigest(),'remaining_array_address_gap_reproduced':True};(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n')
table='\n'.join('| '+row['case']+' | '+' | '.join(f"{row[key][metric]:+.3f}%"for key in ['median_delta_pct','confidence_upper_mean_delta_pct']for metric in ['cpu_ms','wall_ms'])+' |'for row in speed['records'])
(out/'analysis.md').write_text(f'''# Rust integer array indices

The public repro changes `return (*p).values[i];` with an `i64` index into `return (*p).values[i as usize];`. Rust requires a target-sized array index. The helper preserves the complete index expression, evaluates it once and gives literal-dependent arithmetic its original integer width before conversion. A bounded context query refuses unknown contexts. Primitive integer/unknown widths 1/2/4/8 are accepted when the varnode size agrees; constants, enums and noninteger types keep their prior path. Cast suppression and plain pointer-addition notation retain their existing behavior. Raw-pointer wrapping is a separate path.

The registered native test uses 45 complete emitted Rust bodies without rewriting them. At O0/O2, 23,112 function/input comparisons agree with C, checking signed/unsigned indices and element widths, valid array addresses, nested arrays, every store and guard byte, callback order and all 64 shift counts over boundary-valued memory. Both already-valid constant-index controls remain identical. The parent bundle has 41 E0277 errors at each optimization level. Rust uses overflow-checks=off and C uses -fwrapv on x86_64. The first successful prototype is retained separately: rebuilding after the stage/corpus counter changed the executable, so native captures were repeated on the final binary before validation.

The candidate was captured fresh across {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records. Its exact frozen parent captures come from published pointer PR1463 and are hash-verified against parent binary `{build['parent_cli_sha256']}`. All {review['changed_rust_records']} changed Rust records are classified, with {review['new_index_casts']} index casts and {review['literal_width_contexts']} literal-width contexts. The AST audit removes only a new usize cast whose operand matches the original index, optionally with the matching primitive integer identity wrapper. Existing indices/casts, operands, dereferences, calls, assignments, address expressions and the rest of the AST must remain identical. Eighteen positive/negative controls cover wrong operands, narrowed casts, changed bases, lost conversions, nested indices and duplicated calls.

C/Go records remain identical. Variable names, types and storage remain unchanged; all {review['variable_provenance_records']} changed provenance records match the exact union of their source-line mappings. There are no new engine or parser errors. The {review['existing_rust_parse_failures']} inherited raw Rust parser failures retain their diagnostics, with identifier normalization only for structural review. The upstream probestack binary is inspected statically and never executed. Structural review does not establish whole-runtime native equivalence or complete runtime type correctness.

The address semantics remain a separate gap. A complete emitted array-address function matches C for valid indices 0 and 8, but valid C one-past index 9 returns an address in C and panics in Rust. Its unrevised body and raw outputs are archived in `remaining-array-address/`. This index conversion does not repair that reference/bounds behavior, raw-pointer fallback syntax, global byte-scale recovery, uninitialized local arrays or arbitrary nested-subpiece integer contexts. No multiarchitecture native parity is claimed.

All four repository checks plus tooling/lint/catalog/counters pass: {workspace['passed']} workspace tests, all original 675 assertions, and {gates['stage_assertions']} stage assertions. Ten new stage assertions cover loads, stores, valid addresses, nested arrays, callbacks, literal-width contexts, constants and diagnostic cast suppression. No prior assertion was lost, and the original baseline is unchanged.

The fixed speed plan uses 64 balanced pairs across eight cases, three warmups per arm, and a 5% CPU/wall median budget. Every case passes; the largest median delta is {max_median:.3f}%. The largest 95% upper mean bound is {max_upper:.3f}%, reported as uncertainty rather than an acceptance threshold. Every sample and CPU telemetry record is retained. Source manifests, binaries, fixtures and all 154 compiled specs were verified before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` binds this proof to the exact published parent tree and its actual merge. The complete changed records are losslessly compressed and round-trip hash-checked. `manifest.json` hashes the evidence files.
''')
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()};(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n');assert all(sha(r/p)==digest for p,digest in sources.items());print('Array evidence complete',len(manifest),'files',workspace,flush=True)
