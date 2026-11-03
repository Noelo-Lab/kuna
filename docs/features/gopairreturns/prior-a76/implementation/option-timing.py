from common import *
import random,statistics,resource,datetime
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
plan=json.loads((w/'option-timing-plan.json').read_text());build=json.loads((w/'build.json').read_text());assert not(w/'option-timing-result.json').exists();assert json.loads((w/'gates-passed.json').read_text())['all_four_gates_passed'];assert json.loads((w/'breadth-review.json').read_text())['complete'];cli=r/'decompiler/target/release/kuna';digest=sha(cli);assert digest==build['new_cli_sha256'];rng=random.Random(plan['seed']);raw={'off':[],'on':[]};orders=[];os.sched_setaffinity(0,{43})
def once(arm):
 cmd=[str(cli),'decompile',plan['binary'],plan['selector'],'--language',plan['language'],'--option','godwarfsig','on','--option',plan['option'],arm];before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.perf_counter();subprocess.run(cmd,env=env|{'KUNA_SPECS':str(r/'specs')},stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=True);elapsed=time.perf_counter()-start;after=resource.getrusage(resource.RUSAGE_CHILDREN);return {'wall_ms':1000*elapsed,'cpu_ms':1000*(after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime)}
for _ in range(plan['warmups_per_arm']):
 for arm in raw:once(arm)
for _ in range(plan['paired_rounds']//2):
 pairs=[['off','on'],['on','off']];rng.shuffle(pairs)
 for order in pairs:
  orders.append(order)
  for arm in order:raw[arm].append(once(arm))
medians={arm:{m:statistics.median(row[m]for row in data)for m in ['cpu_ms','wall_ms']}for arm,data in raw.items()};delta=100*(medians['on']['wall_ms']/medians['off']['wall_ms']-1);block={'speed_off_ms':medians['off']['wall_ms'],'speed_on_ms':medians['on']['wall_ms'],'speed_delta_pct':delta,'speed_samples':plan['paired_rounds'],'speed_budget_pct':plan['budget_pct'],'speed_within_budget':delta<=plan['budget_pct'],'speed_forced_off':delta>plan['budget_pct']};assert sha(cli)==digest;result={'plan':plan,'completed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cli_sha256':digest,'fixture_sha256':sha(plan['binary']),'raw':raw,'orders':orders,'medians':medians,'timeit_compatible_block':block,'method':'CLI measurements include godwarfsig on and Go output, same candidate for off/on; no failed invocation is dropped.'};(w/'option-timing-result.json').write_text(json.dumps(result,indent=2)+'\n');print('OPTION COST RECORDED',json.dumps(block),flush=True)
