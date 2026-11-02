from pathlib import Path
import json,re,hashlib,shutil,subprocess,gzip

w=Path('/tmp/kuna-rust-fmt-str');r=Path('/home/mahaloz/kwt/angr-rust-fmt-str');out=r/'docs/features/rustfmtstr'
assert json.loads((w/'gates-passed.json').read_text())['all_four_gates_passed']
assert all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in json.loads((w/'production-source-before-validation.json').read_text()).items())
assert 'ALL 12 CHANGED MACROS AND SOURCE MAPPINGS REVIEWED' in (w/'postreview.log').read_text()
assert not (r/'decompiler/target/debug').exists()
rows=re.findall(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w/'workspace-final.log').read_text())
assert all(row[0]=='ok' and row[2]=='0' for row in rows)
tests=dict(passed=sum(int(x[1])for x in rows),failed=sum(int(x[2])for x in rows),ignored=sum(int(x[3])for x in rows),suites=len(rows))
assert tests==dict(passed=10102,failed=0,ignored=39,suites=71),tests

(w/'workspace-summary.json').write_text(json.dumps(tests,indent=2)+'\n')
assert json.loads((w/'main-69fa-equivalence.json').read_text())['all_complete_function_records_identical']
assert json.loads((w/'candidate-index-equivalence.json').read_text())['all_function_records_identical']
assert all('PARITY OK' in(w/f'{name}.log').read_text()for name in ['original','stages'])
assert json.loads((r/'docs/baseline-stages.json').read_text())['data_footer']==[3531,3531]
subprocess.run(['git','diff','--exit-code','--','docs/baseline.json'],cwd=r,check=True)
breadth=json.loads((w/'breadth/record.json').read_text());summary=json.loads((w/'breadth-summary.json').read_text());macros=json.loads((w/'all-macros.json').read_text());native=json.loads((w/'native-execution.json').read_text())
digest=hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest()

ready=json.loads((w/'breadth/capture-ready.json').read_text())
assert ready['patched_kuna_sha256']==digest
for fixture in ready['fixtures']:
    for language in fixture['languages']:
        prefix=w/'breadth'/f"{fixture['fixture']}-{language['language']}"
        assert hashlib.sha256(Path(str(prefix)+'.json').read_bytes()).hexdigest()==language['after_file_sha256']
        if language['language']=='rust':assert hashlib.sha256(Path(str(prefix)+'-off.json').read_bytes()).hexdigest()==language['off_file_sha256']

assert breadth['patched_kuna_sha256']==digest and not breadth['semantic_evidence_pending'] and not breadth['baseline_has_pending_feature']
assert summary['changed_functions']==macros['changed_functions']==macros['recovered_calls']==12
assert macros['all_template_bytes_and_argument_bindings_verified'] and not macros['source_mapping_classification_pending']
assert native['strings']==156 and native['integer_values']==9 and len(native['executions'])==3
assert all(e['stdout_stderr_exact'] and e['stream_calls']==3744 and e['panic_payloads']==156 for e in native['executions'])
config=json.loads((w/'timing-cases.json').read_text());timings=[json.loads((w/f'timing-{row[0]}.json').read_text())for row in config['cases']]
assert len(timings)==8
monitor=json.loads((w/'timing-monitor.json').read_text());assert monitor['accepted_for_speed_review']
config=json.loads((w/'timing-cases.json').read_text())
parent_identity=json.loads((w/'parent-source-before-commit.json').read_text())

reanchor=json.loads((w/'reanchor-proof.json').read_text())
parent_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=config['base'],text=True).strip()
assert reanchor['new_baseline_commit']==parent_head==(w/'baseline-commit.txt').read_text().strip()
assert reanchor['release_binary_unchanged']and reanchor['release_binary_sha256']==digest
assert not subprocess.check_output(['git','status','--porcelain'],cwd=config['base'])

assert hashlib.sha256((Path(config['base'])/'decompiler/target/release/kuna').read_bytes()).hexdigest()==parent_identity['binary_sha256']
for row in timings:
    assert row['binaries_sha256']['baseline']==parent_identity['binary_sha256']
    assert row['binaries_sha256']['patched']==digest and row['plan']['maximum_looks']==1 and row['plan']['family_alpha']==0.05
    assert row['samples']==row['plan']['samples_per_arm']==768 and row['warmups']==3 and row['speed_budget_passed']
    assert row['method']=='balanced_randomized_block_bootstrap'
    assert all(v<=5 for group in [row['delta_pct'],row['confidence_upper_delta_pct']]for metrics in group.values()for v in metrics.values())
maxima={comparison:{metric:max(row['delta_pct'][comparison][metric]for row in timings)for metric in ['cpu_ms','wall_ms']}for comparison in ['baseline_to_patched','off_to_patched']}
out.mkdir(parents=True,exist_ok=True)
record={
    'option':'rustfmtstr','default':'on','baseline_commit':(w/'baseline-commit.txt').read_text().strip(),
    'integrated_main_commit':'69fad3ce4cbdbb3232680938252bd75c2bec5a68','kuna_sha256':digest,
    'reference_angr_commit':'6afa0c79a83d9f677a46b0275cdb63bf16c1979e','reference_binaries_commit':'76ad9e296dde8f82b8ede93ce06a578cd1933639',
    'datatests':dict(passed=675,total=675,baseline_unchanged=True),
    'stages':dict(passed=3531,total=3531,new_case_before=0,new_case_after=8,new_case_assertions=8,before_case_omits_new_option_commands=True),
    'workspace':tests,'tools':dict(passed=114,skipped=1),'lint':True,'spec':True,
    'catalog':dict(rows=366,live_fields=172,corpus_files=645,check=True),
    'breadth':summary,'native':native,'macro_review':macros,
    'timeit':dict(budget_pct=5,maxima_delta_pct=maxima,cases=timings,monitor=monitor),
    'limitations':[
        'Only stable Rust 1.93.0/1.93.1 x86-64 little-endian ELF and the exact verified Display forwarding body qualify.',
        'Both live string words must be complete, aligned and tied to the original formatting call. Valid UTF-8 follows the original Formatter::pad contract.',
        'Unchanged macro expressions execute with explicit input bindings; no complete-body recompilation or original panic-location equivalence is claimed.',
        'Custom and derived callbacks, unknown wrappers, older layouts, aggregate types and full Go/Rust parity remain separate work.',
    ],
}
out.mkdir(parents=True,exist_ok=True)
for name in ['integration-source.json','integration-validation.py','integration-validation.log']:
    shutil.copy2(w/name,out/name)
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n')
files=['finalize.py','timing-monitor.json','timing-cases.json','timing-randomized-plan.json','timing-randomized.py','timing-randomized.log','audit-idle.py','cpu-audit.json','owned-process-audit.json','breadth-summary.json','all-macros.json','mapping-review.json','type-audit.json','native-execution.json','reanchor.py','reanchor-proof.json','native-oracle.rs','recovered-execution.rs','all-macros-execution.rs','repro.rs','repro-before.rs','repro-after.rs','repro-build.log','breadth.py','evidence.py','execute-native.py','postreview.py','type-audit.py','descriptor-evidence.rs','evidence-selfcheck.json','verifier-selfcheck.log','stage-before.log','stage-after.log','stage-status.json','original.log','stages.log','tools.log','lint.log','spec.log','catalog.log','counters.log','native-execution.log','postreview.log','validate.log','workspace-summary.json','review-native-first.log','breadth-new-images.log','review-native-final.log']
for name in files:shutil.copy2(w/name,out/name)
shutil.copy2(w/'breadth/record.json',out/'breadth.json')
for name in ['capture-race-69fa.json','ambiguous-overlap-capture.log','ambiguous-breadth-69fa.log']:
    shutil.copy2(w/name,out/name)
shutil.copy2('/tmp/kuna-angr-reference/breadth-69fa-recovery.py',out/'breadth-69fa-recovery.py')
shutil.copy2('/tmp/kuna-angr-reference/breadth-69fa-recovery.log',out/'breadth-69fa-recovery.log')
for name in ['overlap-capture-plan.json','capture-ready.json']:
    shutil.copy2(w/'breadth'/name,out/name)
shutil.copy2(w/'overlap-capture.log',out/'overlap-capture.log')
for pattern in ['*.diff','*-record.json','*-descriptor.jsonl']:
    for p in(w/'breadth').glob(pattern):shutil.copy2(p,out/p.name)
for pattern in ['*-execution.rs','*.compile.log','*.stdout','*.stderr','timing-*.json']:
    for p in w.glob(pattern):shutil.copy2(p,out/p.name)
(out/'verifier').mkdir(exist_ok=True)
for p in(w/'verifier/src').glob('*.rs'):shutil.copy2(p,out/'verifier'/p.name)
for name in ['Cargo.toml','Cargo.lock']:shutil.copy2(w/'verifier'/name,out/'verifier'/name)
for raw in(w/'timing-randomized-attempts').glob('*/cpu-samples.json'):
    data=raw.read_bytes();compressed=raw.with_suffix('.json.gz');compressed.write_bytes(gzip.compress(data,mtime=0));assert gzip.decompress(compressed.read_bytes())==data
    raw.with_name('cpu-telemetry.json').write_text(json.dumps({'file':compressed.name,'uncompressed_sha256':hashlib.sha256(data).hexdigest(),'uncompressed_bytes':len(data),'encoding':'gzip-compressed JSON; lossless copy of retained raw telemetry'},indent=2)+'\n')
shutil.copytree(w/'timing-randomized-attempts',out/'timing-randomized-attempts',dirs_exist_ok=True,ignore=shutil.ignore_patterns('cpu-samples.json'))
shutil.copytree(w/'timing-protocol-history',out/'timing-protocol-history',dirs_exist_ok=True)
shutil.copytree(w/'timing-before-parent-commit',out/'timing-before-parent-commit',dirs_exist_ok=True)
for name in ['batch-timing-plan.json','batch-timing.py','batch-timing.log']:shutil.copy2(w/name,out/name)
for name in ['feature-confirmation-plan.json','feature-confirmation.py','feature-confirmation.log','first-look-strict-bounds.json']:shutil.copy2(w/name,out/name)
for name in ['gates-passed.json','candidate-index-equivalence.json','main-69fa-equivalence.json','timing-candidate-index.py','timing-candidate-index-plan.json','timing-candidate-index.log','workspace-summary.json']:shutil.copy2(w/name,out/name)
(out/'analysis.md').write_text(f'''# Verified Rust string formatting

`rustfmtstr` exposes string Display arguments inside existing print, allocating
format and panic recovery. It accepts stable Rust 1.93.0/1.93.1 on x86-64
little-endian ELF. Full symbol identities must establish a reference Display
callback and `core::fmt::Formatter::pad`, with the exact 18-byte forwarding body
that loads string data and length and directly jumps to that pad implementation.
Generic names alone never establish a concrete string type. Conflicting aliases,
other wrappers, derived Debug and custom formatting remain explicit calls.

Both eight-byte aligned string words must be complete call-linked stack inputs.
The renderer constructs a string view from those same values. Valid UTF-8 and
storage follow the original `pad(&str)` contract. It changes no setup statements,
IR, prototypes or return statements; the existing format, destination and panic
checks still apply. Recovered panic source has its own file, line and backtrace.

## Native and refusal evidence

The Rust 1.93.1 native fixture covers nine positive functions: stdout, stderr,
mixed primitive/string arguments, two strings, Unicode width/precision/fill,
owned String results and panic payloads. A separate oracle exercises 156 valid
UTF-8 strings, including every ASCII character, embedded NUL, Unicode boundaries,
combining and zero-width joiners, Arabic, Japanese and strings through 4096 bytes.
Nine unsigned boundary values are included. Each of three execution compilers
(1.90.0, 1.93.1 and 1.99.1) matches 3,744 native stream calls and 156 panic payloads:
11,700 comparisons. The emitted macro expressions are unchanged; explicit typed
bindings and uninitialized String destinations supply their inputs.

Six integration tests cover the positive fixture and refusal boundaries: option,
source/output language, exact producer identity, missing and conflicting symbols,
altered wrapper bytes/targets, incomplete or misaligned words, scalar size/type
constraints and unsupported callbacks. Five native unsupported forms and six
other-producer controls remain unchanged. A non-LTO wrapper with a different
instruction sequence also remains explicit. These results do not establish
complete-function recompilation or equivalence of original panic locations.

## Complete output comparison

The 26-image comparison covers 28,720 distinct functions and 60,876 language
records. All 48,223 C/Go records are identical, and all 19,503 Go records parse.
Disabling the option restores all 12,653 Rust records exactly. Twelve functions
change in twelve hunks: ten in the new fixture and two in the modern angr format
fixture, including `make_string`. All surrounding parsed statements and comments
remain identical. Independent ELF reads, full callback names, exact forwarding
bytes, template decoding and call-linked descriptor operands verify every new
literal and all thirteen string argument bindings. Eight AST mutations and five
descriptor-evidence mutations are rejected by the review tools.

All twelve unchanged recovered macro expressions compile and execute under the
three execution compilers with explicit bindings. A partial compiler audit finds
one selected diagnostic in one function both before and after, with no new
diagnostic multiset; none of these complete bodies compiles standalone. Source
mappings omit twelve stack-descriptor LEAs into RSI or RDX from replaced call
lines. Disassembly verifies each address computation. Call instruction mappings,
descriptor stores, declarations and function metadata remain. Variable-use
mapping changes are confined to the recovered inputs and hidden destination.

The initial review tool did not account for the existing `param_1` metadata name
corresponding to signature `a0` in String-result functions. Its rejected capture
is retained. The correction verifies that mapping against the actual signature,
and all complete captures were reviewed again without changing production code.
The original stripped angr fixture remains unchanged; symbol recovery and older
consumer layouts remain separate gaps.

## Checks and speed

The original corpus passes 675/675 without changing its baseline; stages pass
3,531/3,531, with the new case moving from 0/8 to 8/8. The parent capture omits
the new option commands unknown to its catalog. The workspace passes 10,102 tests
with zero failures and 39 ignored across 71 suites. Tooling passes 114 tests with
one optional skip. Lint, spec, catalog, documentation and shared counts pass.

Eight cases compare the parent, the new binary with recovery disabled and the
default using 768 balanced randomized rounds after three warmups per arm.
All six arm orders occur in each six-round block. Independent case workers use
disjoint physical CPU pairs, with each case's arms pinned to the same CPU.
No owned build, test or corpus jobs run during measurement. The fixed protocol
and CPU selection were declared before sampling; all raw observations remain.
Maximum parent-to-default median CPU delta is {maxima['baseline_to_patched']['cpu_ms']:+.3f}%
and wall delta is {maxima['baseline_to_patched']['wall_ms']:+.3f}%; maximum
off-to-default CPU delta is {maxima['off_to_patched']['cpu_ms']:+.3f}% and wall delta
is {maxima['off_to_patched']['wall_ms']:+.3f}%. All median comparisons and upper
mean-ratio bootstrap bounds fit five percent. Twenty thousand bootstrap resamples
preserve six-round blocks. One final look allocates family alpha 0.05 across
32 CPU/wall comparisons, with upper quantile 1 - 0.05/32. No interim acceptance
or automatic repetition is allowed. Main commit
`69fad3ce4cbdbb3232680938252bd75c2bec5a68` is included in all final captures.

The earlier implementation's fixed confirmation did not clear its bounds:
the large Rust control's parent wall-time upper bound was 6.135351%, and its
off-to-default bound was 5.261491%. The observations remain under their original
attempt IDs. Callback discovery now indexes only forwarding/pad candidates,
then checks every symbol for conflicting or unknown aliases. A new fixed study
tests that changed implementation; observations from different binaries are not
combined. Fresh captures preserve every previously reviewed complete function
record, so the native and manual mapping evidence still applies.

Two overlapping capture processes were detected during the final integration
review. Both were stopped before timing, and their ambiguous files were retained
locally. An exclusive lock now prevents duplicate capture or review jobs. The
complete 26-image capture was rerun, file hashes were checked again, and every
complete function record matched the previously reviewed output. Timing used
only this clean capture.

This closes the verified string argument portion of modern format-macro coverage.
It does not establish full Go/Rust parity.
''')
print(json.dumps({'workspace':tests,'breadth':summary,'maxima':maxima},indent=2))
