from common import *
import gzip,shutil,re,datetime
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());gates=json.loads((w/'gates-passed.json').read_text());review=json.loads((w/'breadth-review.json').read_text());captures=json.loads((w/'final-captures.json').read_text());timing=json.loads((w/'timing-result.json').read_text())
assert gates['all_four_gates_passed']and review['complete']and timing['all_default_comparisons_passed'];assert captures['all_six_function_record_sets_equal_native_proof']and captures['all_callback_records_equal_native_proof'];assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert timing['binary_sha256']['candidate']==build['new_cli_sha256'];assert not subprocess.check_output(['git','diff','HEAD','--','docs/baseline.json'],cwd=r)
source=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/name)==digest for name,digest in source.items())
results=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', (w/'workspace-final.log').read_text());totals=[sum(int(row[i])for row in results)for i in range(3)];assert totals[0]>10000 and totals[1]==0 and len(results)>=70
medians=[n for row in timing['records']for n in row['median_delta_pct'].values()];bounds=[n for row in timing['records']for n in row['confidence_upper_mean_delta_pct'].values()];assert max(medians)<=5 and max(bounds)<=5
root=r/'docs/features/fieldextract';out=root/'integration-c5ea';out.mkdir(exist_ok=False)
for name in ['record.json','analysis.md','manifest.json']:shutil.copy2(root/name,root/('before-main-integration-'+name))
record=json.loads((root/'record.json').read_text());record.update(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),parent_commit=build['parent_commit'],parent_cli_sha256=build['parent_cli_sha256'],candidate_cli_sha256=build['new_cli_sha256'],stage_assertions=build['stage_assertions'][0],catalog_count=374,workspace={'passed':totals[0],'failed':totals[1],'ignored':totals[2],'suites':len(results)},breadth={k:review[k]for k in ['images','distinct_functions','language_records','changed_records','unchanged_existing_images']},speed={'all_default_comparisons_passed':True,'maximum_median_delta_pct':max(medians),'maximum_upper_delta_pct':max(bounds),'records':[{k:row[k]for k in ['case','median_delta_pct','confidence_upper_mean_delta_pct','speed_budget_passed']}for row in timing['records']]},native_proof_binding=captures,validation_artifacts='integration-c5ea/',previous_validation_record='before-main-integration-record.json')
(root/'record.json').write_text(json.dumps(record,indent=2)+'\n')
text=f'''The field extraction fix has been validated with main's big-endian joined
return handling at {build['parent_commit']}. The two changes share a corpus
counter and stage baseline; their implementation changes combine without edits.
The combined stage baseline has {build['stage_assertions'][0]} passing assertions,
including the same 14 added extraction assertions. The original 675 baseline
is unchanged, and the workspace reports {totals[0]} passed, zero failed.

Fresh parent/candidate captures cover {review['images']} images,
{review['distinct_functions']} distinct functions and {review['language_records']}
language records. The same 57 records change; {review['unchanged_existing_images']}
images have identical function records, including both new big-endian fixtures.
Every changed hunk is reviewed. All 26 direct caller records in each language
and all eight callback records match the previously executed native proofs
exactly. Their Rust exclusions remain unchanged and are not claimed to pass.

A separate predeclared timing comparison against the updated parent uses nine
cases and 384 balanced paired rounds. Maximum median change: {max(medians):.3f}%;
maximum simultaneous upper bound: {max(bounds):.3f}%. All comparisons pass the
5% criterion. Source, binary and compiled-spec hashes remain stable.

The integration-c5ea directory contains the current validation artifacts.
The earlier validation and its retained failures are preserved below and in
before-main-integration-record.json.

'''
prior=(root/'analysis.md').read_text().replace('Final breadth:', 'Before main integration, breadth:').replace('Timing uses nine cases,','Before main integration, timing used nine cases,');(root/'analysis.md').write_text(text+prior)
def copy(name,compress=False):
 src=w/name;dest=out/(name+'.gz'if compress else name);dest.parent.mkdir(parents=True,exist_ok=True)
 if compress:
  with gzip.open(dest,'wb')as f:f.write(src.read_bytes())
 else:shutil.copy2(src,dest)
for name in ['build.json','gates-passed.json','final-captures.json','timing-plan.json','timing-cpus.json','timing-cpu-samples.json.gz','integration.json','merge.log','common.py','build-both.py','finish-build.py','run-validation.py','validate.py','breadth.py','review.py','final-captures.py','timing.py','finalize-integration.py']:
 copy(name)
for name in ['production-source-before-validation.json','parent-source-before-timing.json','timing-result.json','breadth-review.json','breadth-final/record.json','candidate-build.log','parent-build.log','harness-build.log','record-stages.log','original.log','stages.log','tools.log','lint.log','spec.log','catalog.log','counters.log','workspace-final.log','final-captures.log','review.log','initial-driver-import.log','initial-missing-harness.log']:
 copy(name,True)
for pattern in ['final-*-c.json','final-*-go.json','final-*-rust.json','callback-*.json']:
 for p in w.glob(pattern):copy(p.name,True)
manifest={str(p.relative_to(root)):sha(p)for p in root.rglob('*')if p.is_file()and p!=root/'manifest.json'};(root/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
body=Path('/tmp/kuna-integer-field-extract/pr-body-template.txt').read_text()+f'\nNine timing cases against current main: largest median increase {max(medians):.2f}%; all simultaneous CPU/wall bounds below 5% (maximum {max(bounds):.2f}%).\n';(w/'pr-body.txt').write_text(body)
subprocess.run(['git','diff','--check'],cwd=r,check=True);print('Integration evidence complete',totals,len(manifest),'artifacts',flush=True)
