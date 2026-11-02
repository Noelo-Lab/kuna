from pathlib import Path
import json,re,hashlib,shutil,subprocess
w=Path('/tmp/kuna-rust-fmt-write');r=Path('/home/mahaloz/kwt/angr-rust-fmt-write');out=r/'docs/features/rustfmtwrite'
def read(name):return json.loads((w/name).read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
gates=read('gates-passed.json');assert gates['all_four_gates_passed']and gates['additional_tools_lint_catalog_counters_passed']
assert not(r/'decompiler/target/debug').exists()
rows=re.findall(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w/'workspace-final.log').read_text())
assert all(row[0]=='ok'and row[2]=='0'for row in rows)
tests=dict(passed=sum(int(x[1])for x in rows),failed=sum(int(x[2])for x in rows),ignored=sum(int(x[3])for x in rows),suites=len(rows))
assert tests==dict(passed=10109,failed=0,ignored=39,suites=71),tests
(w/'workspace-summary.json').write_text(json.dumps(tests,indent=2)+'\n')
assert all('PARITY OK'in(w/f'{name}.log').read_text()for name in ['original','stages'])
assert json.loads((r/'docs/baseline-stages.json').read_text())['data_footer']==[3541,3541]
subprocess.run(['git','diff','--exit-code','--','docs/baseline.json'],cwd=r,check=True)
source=read('production-source-before-validation.json');assert all(sha(r/p)==h for p,h in source.items())
digest=sha(r/'decompiler/target/release/kuna');assert digest==gates['release_sha256']

ready=read('breadth/capture-ready.json');assert ready['patched_kuna_sha256']==digest
for fixture in ready['fixtures']:
 for language in fixture['languages']:
  prefix=w/'breadth'/f"{fixture['fixture']}-{language['language']}"
  assert sha(Path(str(prefix)+'.json'))==language['after_file_sha256']
  if language['language']=='rust':assert sha(Path(str(prefix)+'-off.json'))==language['off_file_sha256']

breadth=read('breadth/record.json');summary=read('breadth-summary.json');macros=read('all-macros.json');native=read('native-execution.json')
assert breadth['patched_kuna_sha256']==digest and not breadth['semantic_evidence_pending']and not breadth['baseline_has_pending_feature']
assert summary['images']==27 and summary['changed_functions']==macros['changed_functions']==macros['recovered_calls']==8
context=read('reference-context-review.json');assert context['context_difference_preexists_feature']and context['total_comparisons']==7128
assert all(x['original_string_bytes_and_result_bytes_match']for x in context['executions'])
assert macros['all_template_bytes_and_argument_bindings_verified']and macros['all_writer_vtables_and_result_bytes_verified']and not macros['source_mapping_classification_pending']
assert native['total_comparisons']==21168 and native['texts']==156 and all(x['original_string_bytes_and_result_bytes_match']and x['calls']==7056 for x in native['results'])
config=read('timing-cases.json');timings=[read(f'timing-{row[0]}.json')for row in config['cases']];monitor=read('timing-monitor.json')
assert len(timings)==10 and monitor['accepted_for_speed_review']
parent=read('parent-source-before-commit.json');assert sha(Path(config['base'])/'decompiler/target/release/kuna')==parent['binary_sha256']

reanchor=json.loads((w/'reanchor-proof.json').read_text())
parent_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=config['base'],text=True).strip()
assert reanchor['new_baseline_commit']==parent_head==(w/'baseline-commit.txt').read_text().strip()
assert reanchor['release_binary_unchanged']and reanchor['release_binary_sha256']==digest
assert not subprocess.check_output(['git','status','--porcelain'],cwd=config['base'])

for row in timings:
 assert row['binaries_sha256']=={'baseline':parent['binary_sha256'],'patched':digest}
 assert row['method']=='balanced_randomized_block_bootstrap'and row['speed_budget_passed']
 assert row['samples']==row['plan']['samples_per_arm']==768 and row['warmups']==3
 assert row['plan']['maximum_looks']==1 and row['plan']['family_alpha']==.05 and row['plan']['comparisons']==40
 assert all(v<=5 for group in [row['delta_pct'],row['confidence_upper_delta_pct']]for metrics in group.values()for v in metrics.values())
maxima={comparison:{metric:max(row['delta_pct'][comparison][metric]for row in timings)for metric in ['cpu_ms','wall_ms']}for comparison in ['baseline_to_patched','off_to_patched']}
out.mkdir(parents=True,exist_ok=True)
record={'option':'rustfmtwrite','default':'on','baseline_commit':(w/'baseline-commit.txt').read_text().strip(),'integrated_main_commit':'69fad3ce4cbdbb3232680938252bd75c2bec5a68','kuna_sha256':digest,'reference_angr_commit':'6afa0c79a83d9f677a46b0275cdb63bf16c1979e','reference_binaries_commit':'76ad9e296dde8f82b8ede93ce06a578cd1933639','datatests':{'passed':675,'total':675,'baseline_unchanged':True},'stages':{'passed':3541,'total':3541,'new_case_before':0,'new_case_after':10,'new_case_assertions':10},'workspace':tests,'tools':{'passed':114,'skipped':1},'lint':True,'spec':True,'catalog':{'rows':367,'live_fields':173,'corpus_files':646,'check':True},'breadth':summary,'native':native,'macro_review':macros,'timeit':{'budget_pct':5,'maxima_delta_pct':maxima,'cases':timings,'monitor':monitor},'limitations':['Only exact stable Rust 1.93.0/1.93.1 x86-64 little-endian ELF writer identities and layouts qualify.','Only verified alloc::string::String writer tables and supported literal, primitive integer and string descriptors are recovered.','Unchanged emitted expressions execute with explicit typed inputs. Complete-body recompilation is not claimed.','Custom writers, I/O writers, dynamic formatting, derived callbacks, older writer layouts and full Go/Rust parity remain separate work.']}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n')
files=['finalize.py','breadth.py','breadth-69fa.py','evidence.py','descriptor-evidence.rs','execute-native.py','postreview.py','type-audit.py','type-audit-native.py','review-reference.py','build-controls.py','reanchor.py','reanchor-proof.json','gates-passed.json','breadth-summary.json','all-macros.json','mapping-review.json','mapping-review-breadth.json','type-audit.json','native-execution.json','native-verified.json','reference-write-verified.json','unsupported-producer-controls.json','unsupported-producer-controls.log','native-69fa-equivalence.json','verifier-selfcheck.log','stage-status.json','stage-before.log','stage-after.log','native-oracle.rs','recovered-execution.rs','all-macros-execution.rs','repro.rs','repro-before.rs','repro-after.rs','repro-build.log','original.log','stages.log','tools.log','lint.log','spec.log','catalog.log','counters.log','workspace-summary.json','postreview.log','native-execution.log','capture-native-final-review.log','capture-native-69fa.log','breadth-69fa.log','timing-plan.json','timing-cases.json','timing-randomized-plan.json','timing-monitor.json','timing-candidate-index.py','timing-candidate-index.log','cpu-audit.json','owned-process-audit.json']
for name in files:shutil.copy2(w/name,out/name)
for name in ['review-reference-context.py','reference-context-review.json','reference-context-review.log','reference-context-native.rs','reference-context-execution.rs','postreview-initial-context-failure.log','breadth-69fa-initial-context-failure.log','complete-context-review.py','complete-context-review.log']:
 shutil.copy2(w/name,out/name)
shutil.copy2(w/'breadth/record.json',out/'breadth.json')
for name in ['overlap-capture-plan.json','capture-ready.json']:
    shutil.copy2(w/'breadth'/name,out/name)
shutil.copy2(w/'overlap-capture.log',out/'overlap-capture.log')
for pattern in ['*.diff','*-record.json','*-descriptor.jsonl']:
 for p in(w/'breadth').glob(pattern):shutil.copy2(p,out/p.name)
for pattern in ['*.compile.log','*.stdout','timing-*.json','result-abi*','initial-integer-writer-binding-*.rs','initial-integer-writer-binding-*.log']:
 for p in w.glob(pattern):
  if p.is_file()and(p.suffix in ['.rs','.json','.log','.stdout']):shutil.copy2(p,out/p.name)
(out/'verifier').mkdir(exist_ok=True)
for p in(w/'verifier/src').glob('*.rs'):shutil.copy2(p,out/'verifier'/p.name)
for name in ['Cargo.toml','Cargo.lock']:shutil.copy2(w/'verifier'/name,out/'verifier'/name)
for name in ['timing-randomized-attempts','timing-before-parent-commit']:shutil.copytree(w/name,out/name,dirs_exist_ok=True)
integration=out/'main-69fa';integration.mkdir(exist_ok=True);nw=Path('/tmp/kuna-rust-fmt-write-69fa')
for name in ['integration-source.json','validate-counter-fix.py','validate-counter-fix.log','counters-initial.log','phase-tests-before-counter-fix.rs','production-source-before-counter-fix.json']:
 shutil.copy2(nw/name,integration/name)
(out/'analysis.md').write_text(f'''# Verified Rust String writes

`rustfmtwrite` recovers `write!` and `writeln!` for exact String writer
tables from stable Rust 1.93.0/1.93.1 x86-64 little-endian ELF images.
The direct `core::fmt::write` call must have four runtime arguments and a
one-byte bool, u8 or i8 result. The original writer must come from an entry
value through copies or casts. A complete immutable, aligned 48-byte table
must identify String drop, size 24, alignment 8, String write_str/write_char
and the exact write_fmt method. Every alias must agree. ELF RELRO supplies
private immutability evidence, rounded down at the upper page boundary;
volatile annotations override it.

The existing literal, integer and string proofs verify the format descriptor.
Recovery borrows the same String pointer and scopes the formatting-trait import
inside the expression. `.is_err()` preserves the native result, including its
byte conversion where required. Setup, branches, declarations, prototypes and
IR remain unchanged. Custom writers, dynamic width, unsupported callbacks and
unknown tables remain explicit runtime calls. Candidate symbol maps index only
possible formatting identities and then reject conflicting or unknown aliases.

## Native evidence

Seven integration tests cover seven positive writes plus refusal boundaries,
metadata conflicts, producer/layout/language checks, volatile tables and result
shapes. A zero-instruction compiler barrier in the literal fixture preserves the
initialized Arguments words while preventing LLVM from replacing the test call.
The stage case moves from 0/10 to 10/10. Twenty-two functions from unsupported
1.90.0 and 1.99.1 producers remain identical with recovery on and off.

Three native ABI probes confirm the one-byte fmt::Result representation,
Ok=0, Err=1, String size/alignment and original-buffer mutation. The emitted
expressions execute unchanged with explicit typed inputs under Rust 1.90.0,
1.93.1 and 1.99.1. Each executor matches 7,056 original native calls, totaling
21,168 comparisons. Inputs include 156 UTF-8 strings, 263 unsigned and 264 signed
values, nine mixed unsigned values and three preexisting buffer prefixes.
Output bytes, appending, allocation-capacity variation and result bytes match.

## Complete output review

The {summary['images']}-image comparison covers {summary['distinct_functions']:,}
distinct functions and {summary['function_records']:,} language records.
All {summary['C_Go_identical']:,} C/Go records remain identical; all
{summary['Go_parses']:,} Go records parse. Disabling recovery restores all
{summary['off_matches_parent_rust_records']:,} Rust records exactly. Eight
functions change in {summary['hunks']} hunks: the seven native examples and
angr's modern `write_str` example. Full AST and comment comparisons preserve
surrounding statements. Independent image reads and call-linked operands prove
every table, callback, literal and argument binding. Seven native complete function records match the earlier review. Whole-image
analysis gives the reference writer a pointer type where the earlier single
function analysis used an integer. This difference already exists on the parent.
The review admits only that exact parameter/metadata change, its original-call
pointer casts and the corresponding recovered-writer cast. Every other field
and instruction mapping is identical. The fresh emitted expression matches
7,128 additional native comparisons: 264 signed values, three existing prefixes
and three initial capacities under each of three execution compilers.

All eight unchanged recovered expressions compile and execute under the three
execution compilers. The partial compiler audit reduces selected diagnostics
from eight to zero, introduces no diagnostic multiset and leaves complete-body
standalone compilation at zero before and after. Six omitted instruction
annotations are stack-descriptor LEAs into RCX, confined to replaced call lines;
they have no load, store or flag effect. Call addresses and function metadata
remain. Variable-use mapping changes stay within those calls.

The retained initial review failures concern a proof helper's copy-chain
handling and an execution harness's writer-pointer binding. The corrections
preserve the emitted expressions. Initial stale catalog-count tests were also
corrected; the complete repository checks below ran afterward.

## Repository checks and speed

Original assertions pass 675/675 with their baseline unchanged. Stages pass
3,541/3,541. The workspace passes 10,109 tests, zero failures and 39 ignored
across 71 suites. Tooling passes 114 tests with one optional skip. Lint, spec,
catalog, generated documentation and shared counts pass.

Ten cases compare parent, disabled-option and default binaries in 768 balanced
randomized rounds after three warmups. Each six-round block contains all six
arm orders. Cases use disjoint physical CPU pairs; the coordinator and inference
exclude those pairs, and no owned build, test or corpus work runs during sampling.
The fixed plan was declared before any observations. One final look uses 20,000
six-round-block bootstrap resamples and family alpha 0.05 across 40 comparisons.
Every ratio-of-medians comparison and simultaneous upper mean-ratio bound fits
five percent. Maximum parent-to-default median CPU/wall deltas are
{maxima['baseline_to_patched']['cpu_ms']:+.3f}%/{maxima['baseline_to_patched']['wall_ms']:+.3f}%;
maximum disabled-to-default deltas are
{maxima['off_to_patched']['cpu_ms']:+.3f}%/{maxima['off_to_patched']['wall_ms']:+.3f}%.
Raw observations and CPU telemetry are retained. Main commit
`69fad3ce4cbdbb3232680938252bd75c2bec5a68` is included in all final captures.

This establishes the verified String-writer portion of format-macro support.
It does not establish full Go/Rust parity.
''')
print(json.dumps({'workspace':tests,'breadth':summary,'maxima':maxima},indent=2))
