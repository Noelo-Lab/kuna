from common import *
import gzip,shutil,re,datetime
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());review=json.loads((w/'breadth-review.json').read_text());timing=json.loads((w/'timing-result.json').read_text());option_speed=json.loads((w/'option-timing-result.json').read_text());native={n:json.loads((w/n/'record.json').read_text())for n in ['native-go','native-c','native-extra','native-raw-explicit','native-raw-inferred']}
assert gates['all_four_gates_passed']and review['complete']and timing['all_default_comparisons_passed'];assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']==timing['binary_sha256']['candidate'];assert not subprocess.check_output(['git','diff','HEAD','--','docs/baseline.json'],cwd=r)
source=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==h for p,h in source.items());tests=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w/'workspace-final.log').read_text());totals=[sum(int(row[i])for row in tests)for i in range(3)];assert totals[0]>=10343 and totals[1]==0 and len(tests)>=72
for n in ['native-go','native-c','native-extra']:assert native[n]['cli_sha256']==build['new_cli_sha256']
for n in ['native-raw-explicit','native-raw-inferred']:assert native[n]['capture']['cli_sha256']==build['new_cli_sha256']
root=r/'docs/features/gopairreturns';archive=root/'proposal-evidence';archive.mkdir(exist_ok=False)
for n in ['record.json','analysis.md']:shutil.copy2(root/n,archive/n)
out=root/'implementation';out.mkdir(exist_ok=False)
def copy(base,name,dest=None,compress=False):
 p=base/name;q=out/(dest or name);q=q.with_name(q.name+'.gz')if compress else q;q.parent.mkdir(parents=True,exist_ok=True)
 if compress:
  with gzip.open(q,'wb')as f:f.write(p.read_bytes())
 else:shutil.copy2(p,q)
for n in ['build.json','gates-passed.json','timing-plan.json','option-timing-plan.json','option-timing-result.json','option-timing.py','public-repro.py','timing-cpus.json','timing-cpu-samples.json.gz','common.py','build-focused.py','prepare.py','validate.py','breadth.py','review.py','compare-refinement.py','timing.py','bind-timing.py','finalize.py','review-first-capture.json','allocation-refinement-review.json','revision.json','run-validation.py','native-go.py','native-c.py','native-extra.py','native-extra-run.py','native-raw.py']:
 copy(w,n)
for n in ['production-source-before-validation.json','parent-source-before-timing.json','timing-result.json','breadth-review.json','breadth/record.json','changed-functions.json','build-final.log','console-focused.log','original.log','stages.log','tools.log','lint.log','spec.log','catalog.log','counters.log','workspace-final.log','record-stages.log','stage-focused.log']:
 copy(w,n,compress=True)
for n in native:
 for p in(w/n).iterdir():
  if p.suffix in ['.json','.go','.c','.rs','.py','.body','.log']:copy(w,str(p.relative_to(w)),compress=True)
for p in(w/'public-repro').iterdir():
 if p.suffix in ['.go','.json','.body','.log','.stdout']:copy(w,str(p.relative_to(w)),compress=True)
old=w.parent/'integration-350'
for n in ['gates-passed.json','build.json','breadth/record.json','breadth-review-groups.json','review-go-normalized.txt','review-other-languages-normalized.txt','remaining-signed-field-assignment.json','inferred-raw-probe/unlocked-before-c.body','inferred-raw-probe/unlocked-before-go.body','inferred-raw-probe/capture-all.log','inferred-raw-probe/native/probe.go','inferred-raw-probe/native/go.body','native-c-initial-missing-unknown-typedefs.log','initial-catalog-total-drift.log','initial-prepare-catalog-drift.log','native-raw/initial-driver-option-error.log']:
 copy(old,n,'retained-first-capture/'+n,True)
old=w.parent/'integration-f226'
for n in ['remaining-carrier-conversions.json','raw-carrier-probe/real-call-shape.json','first-validation/workspace-final.log','first-validation/counters.log','carrier-initial-visibility-error.log','carrier-harness-missing-unsafe.log']:
 copy(old,n,'retained-earlier/'+n,True)
old=w.parent/'integration-350-round2'
for n in ['timing-plan.json','timing-result.json','timing-cpus.json','timing-cpu-samples.json.gz','timing-coordinator.log','timing.py','build.json','gates-passed.json','production-source-before-validation.json','breadth-review.json']:
 copy(old,n,'retained-inconclusive-speed/'+n,not n.endswith('.gz'))
for p in(w/'before').rglob('*'):
 if p.is_file()and p.name!='kuna':copy(w,str(p.relative_to(w)),compress=True)
required=[x for x in timing['records']if x['case']in timing['plan']['required_cases']];enabled=[x for x in timing['records']if x['case']not in timing['plan']['required_cases']];maximum=max(v for x in required for v in x['median_delta_pct'].values());upper=max(v for x in required for v in x['confidence_upper_mean_delta_pct'].values())
record={'feature':'gopairreturns','status':'implemented-default-off','at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'profile':'amd64 Go ABIInternal, exactly two full-width signed/unsigned integer results with complete consistent DWARF contracts','option':'gopairreturns','default':'off in every mode','requires_option':'godwarfsig','build':build,'workspace':{'passed':totals[0],'failed':totals[1],'ignored':totals[2],'suites':len(tests)},'original_baseline_unchanged':True,'breadth':{k:v for k,v in review.items()if k!='records'},'native':{'go_whole_functions':16,'go_native_comparisons':210,'c_whole_functions':16,'c_native_comparisons':420,'c_nonzero_divisions':360,'callback_whole_functions':7,'callback_native_comparisons':180,'opaque_whole_forward_callers':{'explicit':280,'inferred':280},'body_rewrites':False,'c_width_options':['ctypes off','realtypes off'],'rust_native_equivalence_claim':False},'speed':{'prior_candidate_default_off_result':'inconclusive; retained-inconclusive-speed contains the failed 384-round comparison','default_off_budget_passed':True,'maximum_default_off_median_pct':maximum,'maximum_default_off_upper_pct':upper,'opt_in_records':[{k:v for k,v in x.items()if k not in['raw','orders']}for x in enabled]},'remaining_gaps':json.loads((w/'review-first-capture.json').read_text())['remaining_gaps'],'proposal_evidence':'proposal-evidence/','artifacts':'implementation/'}
public=json.loads((w/'public-repro/record.json').read_text());assert public['records']['candidate']['cli_sha256']==build['new_cli_sha256'];assert public['records']['parent']['cli_sha256']==build['parent_cli_sha256']
record.update({'opportunity':'test_go_dwarf_signatures::paired_integer_results','test_name':'test_go_dwarf_signatures.DwarfSignatureChecks.test_functions','binary':'tests/fixtures/go-pair-results/go1.27.3-pair','selector':'main.pair','func_addr':next(f['address_hex']for f in json.loads((w/'native-go/go1.27.3.json').read_text())['functions']if f['name']=='main.pair'),'angr_version':'10.0.6.dev0','angr_commit':'6afa0c79a83d9f677a46b0275cdb63bf16c1979e','flag':'analysis_gopairreturns','source_decompiler':'kuna','reference_decompiler':'angr','inspiration':'angr Go multiple-result signatures and Go ABIInternal independent result registers','change_kind':'analysis-enablement','default_on':False,'ablation_changed':None,'ablation_note':'Original corpus measured at the shipped default; enabled behavior is exercised by the stage/native/breadth tests. No enabled whole-original-corpus count is claimed.','parity':'OK','full_language_parity':False,'public_reproduction':public,'decisions':['Require complete scalar argument and two integer result contracts with independent ABI storage proof.','Remain opt-in in every mode.','Use ordinary joined structs and validated Go emission adapters with one evaluation.']})
proposal=json.loads((archive/'record.json').read_text());side=[]
for version,capture in proposal['angr_captures'].items():
 side.append('Historical angr '+version+' fixture SHA256 '+capture['fixture_sha256'])
 side.extend(f['code']for f in capture['functions']if f['name']in ['main.pair','main.forward'])
side.append('Final public reproduction, fixture SHA256 '+public['fixture_sha256'])
for arm in ['parent','candidate']:side.extend([arm+' CLI '+public['records'][arm]['cli_sha256'],public['records'][arm]['code']])
(root/'angr-vs-kuna.txt').write_text('Presentation whitespace normalized; exact captures are preserved in the JSON evidence.\n\n'+'\n'.join(line.rstrip()for line in '\n\n'.join(side).splitlines())+'\n')
(root/'plan.md').write_text('''Import only complete amd64 Go ABIInternal contracts with scalar inputs and exactly two 64-bit integer results. Prove RAX/RBX joined storage, reject repeated concrete entries and metadata collisions, and attach a validated two-field language facet. Use fixed two-slot result storage so scalar contracts do not acquire result-list allocations.

Keep ordinary struct storage throughout the engine. Go emission uses two-result declarations and single-evaluation assignments, projections and forwarding adapters. Bounded carrier conversions handle zero extensions, exact word shifts and verified opaque results. C and Rust retain ordinary struct representation; unsupported Rust carrier expressions remain documented.

Keep the feature off in every preset and require godwarfsig. Validate native complete bodies, every changed whole-image record, the original corpus and stages, all workspace checks, and hash-bound default-overhead and option-flip timing. Preserve previous failed or inconclusive evidence.
''')
assert option_speed['cli_sha256']==build['new_cli_sha256'];record.update(option_speed['timeit_compatible_block']);record['speed_measurement']='Same candidate, gopairreturns off/on, godwarfsig on, Go main.forward; distinct from the parent comparison.'
(root/'record.json').write_text(json.dumps(record,indent=2)+'\n')
text=f'''The option `gopairreturns on` recovers exactly two 64-bit integer results for
amd64 Go ABIInternal functions with complete, consistent DWARF contracts. It
requires `godwarfsig on` and remains off in every preset. C and Rust use the
ordinary struct representation; Go exposes two source-level results.

Sixteen complete recovered functions execute unchanged against authored Go
source on Go 1.22.5 and 1.27.3. The same sixteen C bodies pass at O0/O2 with
explicit-width types and wrapping arithmetic. The native checks cover 210 Go
and 420 C input comparisons; C division by zero is excluded. Seven complete
callback/projection/identity/mixed-sign bodies add 180 comparisons, including
a narrowed second result and an exact one-call counter. Complete opaque-return
forwarders add 560 comparisons across both languages, both fixture versions,
both Go compilers and both C optimization levels. Half exercise an unresolved
unlocked prototype while retaining the recovered 16-byte result.

The full workspace passes {totals[0]} tests, with zero failures. The original
675 baseline assertions are unchanged. The stages contain 3733 assertions,
including eight new pair assertions. Catalog and corpus counts are 375/682.

The final whole-image comparison covers {review['images']} images. Default-off
function records are identical to the independent parent. Enabled changes,
all corresponding full function records, and review limits are retained in
implementation/. Structural review is not a whole-runtime equivalence proof.
In particular, untyped callers still have incomplete bool/slice return and
argument inference; some signed Go field stores and Rust aggregate-carrier
expressions still need work. These are listed individually in record.json.
No claim of complete Go or Rust parity is made by this feature.

Six default-off timing comparisons pass the predeclared 5% criterion: largest
median change {maximum:.3f}%, largest simultaneous upper bound {upper:.3f}%.
Three parent-to-opt-in comparisons report costs separately. A distinct 32-round
measurement flips the option on the same final binary and supplies the
timeit-compatible speed fields in record.json. Each parent comparison uses
{timing['plan']['samples_per_arm']} balanced paired rounds; binaries, sources and compiled
specs are hash-bound.

The proposal record and previous failures remain preserved. The previous
candidate's 384-round timing study did not establish the required wall-time
bounds. That full result is retained. The revision removes two heap allocations
per imported scalar result and bypasses disabled pair call dispatch. All
176670 complete enabled/disabled function records equal the prior candidate.
The revised sample size and unchanged acceptance criteria were declared before
the final measurement; no failed sample was removed.
The first complete
capture found invalid raw-to-pair conversions at unresolved runtime calls;
the final implementation handles those through a single-evaluation adapter.
'''
(root/'analysis.md').write_text(text)
manifest={str(p.relative_to(root)):sha(p)for p in root.rglob('*')if p.is_file()and p!=root/'manifest.json'};(root/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
body='''[AUTOMATED] Recover Go functions returning two 64-bit integers, including callers that discard, forward or use both results.

```sh
cat >/tmp/pair.go <<'GO'
package main
import "fmt"
//go:noinline
func pair(a,b int64)(int64,int64){return a+b,a-b}
func main(){fmt.Println(pair(7,3))}
GO
go build -trimpath -o /tmp/pair /tmp/pair.go
kuna decompile /tmp/pair main.pair --language go --option godwarfsig on
# With this change: add --option gopairreturns on
```

Before: `{before_signature}`. With the option: `{after_signature}`.

- Validate complete amd64 ABIInternal/DWARF contracts and RAX/RBX storage; reject unsupported or conflicting signatures.
- Emit paired declarations, single-evaluation calls, projections and return adapters. Keep the option off in every mode.
- Native Go/C checks cover 16 complete functions, seven callback cases, boundaries and call counts. Untyped runtime callers and some Rust carrier expressions remain limited; details are in `docs/features/gopairreturns/analysis.md`.

'''+f'Default-off output is unchanged across 67 images and the original 675 assertions. Largest measured default-off median change: {maximum:.2f}%; upper bound: {upper:.2f}%.\n'
before_signature=next(line.rstrip(' {')for line in public['records']['parent']['code'].splitlines()if line.startswith('func pair('));after_signature=next(line.rstrip(' {')for line in public['records']['candidate']['code'].splitlines()if line.startswith('func pair('));body=body.replace('{before_signature}',before_signature).replace('{after_signature}',after_signature)
(w/'pr-body.txt').write_text(body);subprocess.run(['git','diff','--check'],cwd=r,check=True);print('Final Go pair evidence prepared',len(manifest),'artifacts',flush=True)
