from common import *
import datetime,gzip,lzma,re,shutil
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());review=json.loads((w/'breadth/structural-review.json').read_text());native=json.loads((w/'native-record.json').read_text());speed=json.loads((w/'timing-result.json').read_text());merged=json.loads((w/'rust-store-merged.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and speed['all_default_comparisons_passed']and native['status']=='PASS'
assert build['new_cli_sha256']==gates['release_sha256']==native['candidate_cli_sha256']==speed['binary_sha256']['candidate']==sha(r/'decompiler/target/release/kuna')
assert merged['merged_tree']==build['parent_tree']=='af1423797609972837da654e04480e4aa2752dee'
assert native['complete_bodies']==57 and native['native_function_comparisons']==29544 and not native['body_rewrites']
sources=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==digest for p,digest in sources.items())
assert not subprocess.check_output(['git','diff',build['parent_commit'],'--','docs/baseline.json'],cwd=r)
out=r/'docs/features/rustptradd';assert not out.exists();out.mkdir(parents=True)
def copy(p,name=None):
 p=Path(p);q=out/(name or p.name);q.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,q)
def compressed(p,name):
 q=out/name;q.parent.mkdir(parents=True,exist_ok=True)
 with Path(p).open('rb')as src,q.open('wb')as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0)as dst:shutil.copyfileobj(src,dst)
def evidence(p,name):
 p=Path(p)
 if p.stat().st_size>40000 or p.suffix in {'.log','.jsonl','.stdout','.stderr'}:compressed(p,name+'.gz')
 else:copy(p,name)
for name in ['build.json','gates-passed.json','native-record.json','parent-record.json','public-repro.c','public-repro.json','public-repro-parent.rs','public-repro-candidate.rs','timing-plan.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','production-source-before-validation.json','parent-source-before-timing.json','rust-store-merged.json','plan.json','prototype-build.json','native-review.json','complete-capture-manifest.json']:
 copy(w/name)
for p in sorted(w.glob('*.py')):copy(p,'methods/'+p.name)
for p in sorted(w.glob('*.log')):compressed(p,'logs/'+p.name+'.gz')
for p in sorted((w/'native').iterdir()):
 if p.is_file()and p.suffix in {'.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.kuna'}:evidence(p,'native/'+p.name)
for folder in ['pointer-cast-native','pointer-cast-memory']:
 extra=json.loads((w/folder/'record.json').read_text());assert extra['status']=='PASS'and not extra['registered']and not extra['body_rewrites']
 assert all(row['cli_sha256']==build['new_cli_sha256']for row in extra['records']if row.get('arm')=='candidate')
 for p in sorted((w/folder).iterdir()):
  if p.is_file()and p.suffix in {'.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.kuna'}:evidence(p,'supplementary/'+folder+'/'+p.name)
for p in w.iterdir():
 if p.is_file()and p.name.startswith(('controls','index-context'))and p.suffix in {'.c','.rs','.json','.jsonl','.kuna'}:evidence(p,'controls-and-prototypes/'+p.name)
for p in sorted((w/'before-index-context-fix').iterdir()):
 if p.is_file()and p.name!='kuna':evidence(p,'before-index-context-fix/'+p.name)
for p in w.glob('*-samples.json'):copy(p,'speed/'+p.name)
(out/'breadth').mkdir(exist_ok=True)
source=w/'breadth/record.json';archive=out/'breadth/complete-changes.json.xz'
with source.open('rb')as src,lzma.open(archive,'wb',preset=6)as dst:shutil.copyfileobj(src,dst)
raw=hashlib.sha256()
with lzma.open(archive,'rb')as src:
 while block:=src.read(1024*1024):raw.update(block)
assert raw.hexdigest()==sha(source)
for name in ['pointer-review.json','pointer-review-v2.json','pointer-review-v3.json','pointer-review-v4.json','metadata-review.json','structural-review.json']:compressed(w/'breadth'/name,'breadth/'+name+'.gz')
for folder in ['ast-audit','ast-audit-v2','ast-audit-v3','ast-audit-v4']:
 for name in ['Cargo.toml','Cargo.lock','src/main.rs']:copy(w/folder/name,'methods/'+folder+'/'+name)
copy(Path('/tmp/kuna-rust-io-proposal/built-specs.json'),'compiled-specs.json')
rows=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',(w/'workspace-final.log').read_text());workspace=dict(zip(['passed','failed','ignored'],[sum(int(row[i])for row in rows)for i in range(3)]));workspace['suites']=len(rows);assert workspace=={'passed':10363,'failed':0,'ignored':39,'suites':72}
max_median=max(value for row in speed['records']for value in row['median_delta_pct'].values());max_upper=max(value for row in speed['records']for value in row['confidence_upper_mean_delta_pct'].values())
summary={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'feature':'Rust raw-pointer PTRADD indexing','parent_commit':build['parent_commit'],'parent_tree':merged['merged_tree'],'merged_parent_commit':merged['merged_commit'],'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'gates':gates,'workspace':workspace,'registered_native_complete_bodies':57,'registered_native_function_comparisons':29544,'supplementary_native_complete_bodies':5,'supplementary_native_function_comparisons':1660,'breadth':review,'max_cpu_or_wall_median_delta_pct':max_median,'max_95pct_upper_mean_delta_pct':max_upper,'upper_bound_is_acceptance_gate':False,'whole_runtime_native_equivalence_claimed':False,'complete_changes_raw_sha256':raw.hexdigest(),'array_exclusions':json.loads((w/'plan.json').read_text())['remaining_array_gaps']}
(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n')
table='\n'.join('| '+row['case']+' | '+' | '.join(f"{row[key][metric]:+.3f}%"for key in ['median_delta_pct','confidence_upper_mean_delta_pct']for metric in ['cpu_ms','wall_ms'])+' |'for row in speed['records'])
(out/'analysis.md').write_text(f'''# Rust raw-pointer indices

Raw pointers cannot use Rust array indexing. The public repro changes `return p[i];` to `return *p.wrapping_byte_offset((i as isize).wrapping_mul(4));`. The byte scale comes from PTRADD. Loads and stores dereference once; address expressions retain a raw pointer without borrowing a pointee. Index arithmetic wraps at pointer width, and literal-dependent shifts retain their source integer width. Base and index expressions appear once. Different output pointee types receive an explicit pointer cast.

The implementation requires matching 32- or 64-bit raw-pointer and integer-index widths, byte-addressed pointers and a constant scale. Constant bases, relative pointers, array-valued sources and implied PTRSUB/COPY bases retain their existing paths. Cast suppression retains its prior diagnostic output. Rust's [`wrapping_byte_offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_byte_offset) is stable since 1.75, below the repository's 1.90 minimum.

The registered test compiles 57 complete emitted functions at O0/O2 and compares 29,544 function/input results against C. It covers signed and unsigned elements of 1/2/4/8 bytes, valid positive and negative indices, all destination and guard bytes, record fields, modular address bits, callback order and all 64 shift counts. Rust uses `overflow-checks=off`; C uses `-fwrapv`. No emitted body is rewritten. Five supplementary bodies add 1,660 comparisons for explicit pointer-result casts and byte loads/stores at an eight-byte stride. These supplementary bodies are archived but not registered cargo tests.

The first prototype passed the original 54 bodies, but additional literal-shift probes failed Rust width inference. The implementation now uses the existing integer-context helper before the `isize` cast. The failed prototype source, binary hash, captures and compiler diagnostics remain archived. A separate compile-only survey confirms float, boolean and pointer-valued raw-pointer accesses. Array fields and nested arrays still have a signed array-index problem: `array_field` stays unchanged, while `nested_array` fixes the outer raw pointer but retains the failing inner array index. Those diagnostics remain visible and belong to a separate change.

The static comparison covers {review['images']} images, {review['distinct_functions']} distinct functions and {review['language_function_records']} language/function records. All {review['changed_rust_records']} changed Rust records are classified by reconstructing their original indexing/address AST from the pointer method. Base, index, dereference and surrounding expressions are preserved; newly explicit pointer-result casts are recorded separately. The byte scale is recorded but cannot be independently inferred from the old indexing AST alone. C and Go records are unchanged. The initial comparison rejected variable provenance changes; all {review['variable_provenance_records']} changed variable records retain their names, types and storage, with addresses independently verified as the exact union of their source-line mappings. Existing captures were reused after this check. There are no new decompiler or parser failures. {review['existing_rust_parse_failures']} inherited parser failures use review-only identifier normalization, with original bodies and diagnostics retained. The upstream `probestack` image was inspected statically and never executed.

The first audit left pointer-result casts and the inherited `unsafe` parameter name pending. The second audit records pointer-result casts and normalizes that invalid identifier solely for review. The third audit handles functions mixing retained and newly explicit casts. The fourth scopes retained casts to their original statements, distinguishing identical pointer expressions used in different contexts. Existing casts are preserved by complete AST comparison. Twenty-six controls check accepted transformations, preserved casts, changed bases/indices, missing dereferences, nonwrapping operations and duplicated calls. This structural audit does not establish native equivalence or type correctness for whole runtime images.

All four repository checks pass: {workspace['passed']} workspace tests, all original 675 assertions and {gates['stage_assertions']} stage assertions. Eight stage assertions were added without losing prior ones. Original baseline and catalog metadata are unchanged. Production sources stayed fixed throughout final validation and timing.

The declared speed plan uses 64 balanced pairs per case and three warmups per arm. Every CPU and wall median delta is within the 5% budget. The largest median delta is {max_median:.3f}%; the largest 95% upper mean bound is {max_upper:.3f}%. Bounds describe uncertainty and are not acceptance thresholds. All raw samples and CPU telemetry are retained.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
{table}

`record.json` binds the candidate to its validated parent tree, which matches the actual parent merge. Both binaries and all 154 compiled processor specs were checked before and after timing. The complete changed bodies are losslessly archived and hash-checked. `manifest.json` hashes the evidence archive. Full array indexing, multiarchitecture native proof and broader Go/Rust ABI and enum recovery remain separate work.
''')
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()};(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n');assert all(sha(r/p)==digest for p,digest in sources.items());print('Rust pointer evidence finalized',len(manifest),'files',flush=True)
