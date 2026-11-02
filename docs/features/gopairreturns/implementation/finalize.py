from common import *
import gzip,shutil,re,datetime
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());review=json.loads((w/'breadth-review.json').read_text());timing=json.loads((w/'timing-result.json').read_text());option_speed=json.loads((w/'option-timing-result.json').read_text());public=json.loads((w/'public-repro/record.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and timing['all_default_comparisons_passed'];assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']==timing['binary_sha256']['candidate']==public['records']['candidate']['cli_sha256']==option_speed['cli_sha256'];assert public['records']['parent']['cli_sha256']==build['parent_cli_sha256'];assert not subprocess.check_output(['git','diff','HEAD','--','docs/baseline.json'],cwd=r)
source=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==h for p,h in source.items());tests=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w/'workspace-final.log').read_text());totals=[sum(int(row[i])for row in tests)for i in range(3)];assert totals[0]>=10349 and totals[1]==0 and len(tests)>=72
native={n:json.loads((w/n/'record.json').read_text())for n in ['native-go','native-c','native-extra','native-raw-explicit','native-raw-inferred']}
for n in ['native-go','native-c','native-extra']:assert native[n]['cli_sha256']==build['new_cli_sha256']
for n in ['native-raw-explicit','native-raw-inferred']:assert native[n]['capture']['cli_sha256']==build['new_cli_sha256']
root=r/'docs/features/gopairreturns';record=json.loads((root/'record.json').read_text());assert record['build']['new_cli_sha256']=='3877ad7912406ee1c8c6007ec69241b5b66c2fc1362c79846687d65d59ec3131';manifest=json.loads((root/'manifest.json').read_text());assert all(sha(root/name)==digest for name,digest in manifest.items());assert not(root/'prior-a76').exists();shutil.copytree(root,w/'prior-a76-docs');assert all(sha(w/'prior-a76-docs'/name)==digest for name,digest in manifest.items());shutil.move(str(w/'prior-a76-docs'),root/'prior-a76');shutil.rmtree(root/'implementation');out=root/'implementation';out.mkdir()
def copy(path,compress=False):
 q=out/path.relative_to(w);q=q.with_name(q.name+'.gz')if compress else q;q.parent.mkdir(parents=True,exist_ok=True)
 if compress:
  with gzip.open(q,'wb')as f:f.write(path.read_bytes())
 else:shutil.copy2(path,q)
for p in w.iterdir():
 if p.is_file()and p.suffix in ['.py','.json','.log']:
  copy(p,compress=p.suffix=='.log'or p.name in ['production-source-before-validation.json','parent-source-before-timing.json','timing-result.json','breadth-review.json','changed-functions.json','workspace-final.log'])
copy(w/'timing-cpu-samples.json.gz');copy(w/'breadth/record.json',True)
for p in w.glob('*-samples.json'):copy(p,True)
prior=w.parent/'integration-a324';archive=root/'prior-a324';archive.mkdir()
paths=[p for p in prior.iterdir()if p.is_file()and p.suffix in ['.py','.json','.log','.txt']]
paths.append(prior/'breadth/record.json')
for name in ['native-go','native-c','native-extra','native-raw-explicit','native-raw-inferred','public-repro']:
 paths.extend(p for p in(prior/name).iterdir()if p.is_file()and p.suffix in ['.json','.go','.c','.rs','.py','.body','.bodies','.log','.stdout'])
for p in paths:
 dest=archive/p.relative_to(prior);dest=dest.with_name(dest.name+'.gz');dest.parent.mkdir(parents=True,exist_ok=True)
 with gzip.open(dest,'wb')as output:output.write(p.read_bytes())

for name in [*native,'public-repro']:
 for p in(w/name).iterdir():
  if p.is_file()and p.suffix in ['.json','.go','.c','.rs','.py','.body','.bodies','.log','.stdout']:copy(p,True)
required=[row for row in timing['records']if row['case']in timing['plan']['required_cases']];maximum=max(value for row in required for value in row['median_delta_pct'].values());upper=max(value for row in required for value in row['confidence_upper_mean_delta_pct'].values());enabled=[{k:v for k,v in row.items()if k not in ['raw','orders']}for row in timing['records']if row['case']not in timing['plan']['required_cases']]
record.update(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),build=build,workspace={'passed':totals[0],'failed':totals[1],'ignored':totals[2],'suites':len(tests)},breadth={k:v for k,v in review.items()if k not in ['records','pending']},public_reproduction=public,speed={'default_off_budget_passed':True,'maximum_default_off_median_pct':maximum,'maximum_default_off_upper_pct':upper,'opt_in_records':enabled,'acceptance':timing['plan']['criterion'],'confidence_interpretation':'Reported per-case 95% upper mean-ratio bounds are uncertainty, not acceptance gates. Final integration uses the median5% budget.','prior_a76':'prior-a76/ retains the earlier passed stronger simultaneous-bound study and earlier inconclusive evidence.','prior_a324':'prior-a324/ retains full correctness/native checks and the explicitly cancelled, unanalysed timing study after main advanced.'},prior_integration_evidence=['prior-a76/','prior-a324/'],remaining_gaps=review['remaining_gaps']);record.update(option_speed['timeit_compatible_block']);record['speed_measurement']='Same final candidate, gopairreturns off/on, godwarfsig on, Go main.forward; separate from parent comparison.'
(root/'record.json').write_text(json.dumps(record,indent=2)+'\n')
text=f"""With `godwarfsig on` and `gopairreturns on`, complete amd64 Go ABIInternal
DWARF contracts recover exactly two full-width signed or unsigned integer
results. The option remains off in every preset. Go declarations, calls and
returns use two-result syntax; C and Rust retain ordinary structs.

The final build includes current main's GOT tail-call feature and DWARF member-layout corrections. Its full
workspace passes {totals[0]} tests with zero failures, and all 675 original
assertions remain unchanged. The stage baseline passes {build['stage_assertions'][0]}
assertions; catalog and corpus counts are {build['catalog_count']}/{build['corpus_files']}.

Native proofs bind this final binary: sixteen complete Go bodies on Go 1.22.5
and 1.27.3, the same sixteen C bodies at O0/O2, seven callback and projection
bodies, and explicit/inferred opaque-return forwarders. All recovered bodies
execute unchanged. These provide 1370 native input comparisons, including
exact callback counts, signed boundaries, zero/high-word constructors and
Go division panics. C division by zero is excluded; C integer widths and
wrapping arithmetic are explicit. Native Rust equivalence is not claimed.

Fresh whole-image comparisons cover {review['images']} images, including three GOT
tail-call fixtures and two DWARF member-layout fixtures. They retain {review['distinct_functions']} distinct functions and
{review['language_records']} language records per arm. Default-off records are identical
to current main. All {review['changed_records']} enabled changes are reviewed using complete
before/after records, with unfamiliar changes held pending. The exact review
basis is preserved in implementation/breadth-review.json.gz. This is not a
whole-runtime native equivalence claim.

Remaining limitations include untyped bool/slice results and call arguments,
signed Go field assignments, and Rust aggregate carriers. Individual cases
remain listed in record.json. No complete Go or Rust parity is claimed.

The six required default-off speed comparisons pass their predeclared 5%
median criteria: largest median increase {maximum:.3f}%. Each parent comparison
has 128 balanced paired rounds. Per-case 95% upper bounds on mean ratios are
reported as uncertainty, with a largest default-off bound of {upper:.3f}%;
these bounds are not acceptance gates. Three opt-in comparisons report costs
separately. A distinct 32-round
same-binary option flip supplies the standard speed fields. Sources, binaries,
fixtures and compiled specs are hash-bound. Every sample is retained.

Prior evidence is preserved under prior-a76/ and prior-a324/. The a76 build
passed its stronger simultaneous-bound study. The a324 integration passed
correctness and native checks; its timing was cancelled before analysis when
main gained DWARF member-layout changes. Its worker kept samples in memory,
so only its plan and progress log survived cancellation. No pass is claimed.
The final integration study was declared before its builds and samples, using
the repository median budget and explicit uncertainty reporting. These studies
are distinct. Earlier inconclusive measurements and failed probes are retained.
""";(root/'analysis.md').write_text(text)
proposal=json.loads((root/'proposal-evidence/record.json').read_text());side=[]
for version,capture in proposal['angr_captures'].items():
 side.append('Historical angr '+version+' fixture SHA256 '+capture['fixture_sha256']);side.extend(f['code']for f in capture['functions']if f['name']in ['main.pair','main.forward'])
side.append('Final public reproduction, fixture SHA256 '+public['fixture_sha256'])
for arm in ['parent','candidate']:side.extend([arm+' CLI '+public['records'][arm]['cli_sha256'],public['records'][arm]['code']])
(root/'angr-vs-kuna.txt').write_text('Presentation whitespace normalized; exact captures are preserved in JSON.\n\n'+'\n'.join(line.rstrip()for line in '\n\n'.join(side).splitlines())+'\n')
manifest={str(p.relative_to(root)):sha(p)for p in root.rglob('*')if p.is_file()and p!=root/'manifest.json'};(root/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
body=(w.parent/'integration-350-round3/pr-body.txt').read_text();before_signature=next(line.rstrip(' {')for line in public['records']['parent']['code'].splitlines()if line.startswith('func pair('));after_signature=next(line.rstrip(' {')for line in public['records']['candidate']['code'].splitlines()if line.startswith('func pair('));body,n=re.subn(r'Before: `[^`]+`\. With the option: `[^`]+`\.',lambda _:f'Before: `{before_signature}`. With the option: `{after_signature}`.',body);assert n==1;old_speed=re.search(r'Largest measured default-off median change: [^\n]+',body);assert old_speed;body=body[:old_speed.start()]+f'Largest measured default-off median change: {maximum:.2f}%; 95% upper mean bound: {upper:.2f}% (reported uncertainty).\n';body=body.replace('across 67 images','across 72 images');(w/'pr-body.txt').write_text(body);subprocess.run(['git','diff','--check'],cwd=r,check=True);print('Final current-main Go pair evidence prepared',len(manifest),'artifacts',flush=True)
