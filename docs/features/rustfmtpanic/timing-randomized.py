from pathlib import Path
from concurrent.futures import ProcessPoolExecutor
import os,sys,time,json,subprocess,resource,statistics,random,itertools,hashlib,threading,runpy,shutil,multiprocessing

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def sample():
    return {'time':time.monotonic(),'cpus':{int(p[0][3:]):list(map(int,p[1:]))for line in Path('/proc/stat').read_text().splitlines()if(p:=line.split())and p[0].startswith('cpu')and p[0][3:].isdigit()}}

def case(job):
    config,plan,row,index,cpu,digests,attempt=job
    root=Path(config['root']);base=Path(config['base']);name,binary,selector,language=row
    os.sched_setaffinity(0,{cpu});assert os.sched_getaffinity(0)=={cpu}
    binary=Path(binary);fixture_sha=sha(binary);rng=random.Random(plan['seed']+index)
    raw={side:[]for side in ['baseline','off','patched']};orders=[]
    def run(side):
        executable=(base if side=='baseline'else root)/'decompiler/target/release/kuna'
        options=['--option','godwarfsig','on']if language=='go'else[]
        if side=='off':options+=['--option',config['option'],'off']
        command=['taskset','-c',str(cpu),str(executable),'decompile',str(binary),selector,'--language',language]+options
        before=resource.getrusage(resource.RUSAGE_CHILDREN);started=time.perf_counter()
        subprocess.run(command,cwd=root,env=os.environ|{'KUNA_SPECS':str(root/'specs')},stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=True)
        wall=time.perf_counter()-started;after=resource.getrusage(resource.RUSAGE_CHILDREN)
        return {'wall_ms':wall*1000,'cpu_ms':1000*((after.ru_utime+after.ru_stime)-(before.ru_utime+before.ru_stime))}
    for _ in range(plan['warmups_per_arm']):
        order=list(raw);rng.shuffle(order)
        for side in order:run(side)
    for block in range(plan['samples_per_arm']//6):
        permutations=list(itertools.permutations(raw));rng.shuffle(permutations)
        for order in permutations:
            orders.append(order)
            for side in order:raw[side].append(run(side))
        if(block+1)%8==0:print(name,'completed rounds',(block+1)*6,flush=True)
    assert sha(binary)==fixture_sha
    medians={s:{m:statistics.median(x[m]for x in xs)for m in ['cpu_ms','wall_ms']}for s,xs in raw.items()}
    means={s:{m:statistics.mean(x[m]for x in xs)for m in ['cpu_ms','wall_ms']}for s,xs in raw.items()}
    deltas={a+'_to_patched':{m:100*(medians['patched'][m]/medians[a][m]-1)for m in ['cpu_ms','wall_ms']}for a in ['baseline','off']}
    record={'method':'balanced_randomized_block_bootstrap','execution':'independent worker per case on a disjoint physical CPU pair','plan':plan,'baseline_commit':config['baseline_commit'],'fixture':name,'sha256':fixture_sha,'selector':selector,'language':language,'samples':plan['samples_per_arm'],'warmups':plan['warmups_per_arm'],'affinity_cpu':cpu,'binaries_sha256':digests,'medians':medians,'means':means,'delta_pct':deltas,'raw':raw,'orders':orders}
    Path(attempt,f'timing-{name}.json').write_text(json.dumps(record,indent=2)+'\n')
    print(name,'medians',deltas,flush=True)
    return record

def quantile(values,q):
    values=sorted(values);position=q*(len(values)-1);lo=int(position);hi=min(lo+1,len(values)-1)
    return values[lo]+(position-lo)*(values[hi]-values[lo])

def inference(record,plan,index):
    raw=record['raw'];keys=[(s,m)for s in ['baseline','off','patched']for m in ['cpu_ms','wall_ms']]
    blocks=[[sum(raw[s][i][m]for i in range(start,start+6))for s,m in keys]for start in range(0,plan['samples_per_arm'],6)]
    distributions={a+'_to_patched':{m:[]for m in ['cpu_ms','wall_ms']}for a in ['baseline','off']};rng=random.Random(plan['seed']+1000+index)
    for _ in range(plan['bootstrap_resamples']):
        totals=[0.0]*6
        for _ in blocks:
            block=rng.choice(blocks)
            for j,x in enumerate(block):totals[j]+=x
        for side,index in [('baseline',0),('off',2)]:
            for m,j in [('cpu_ms',0),('wall_ms',1)]:distributions[side+'_to_patched'][m].append(100*(totals[4+j]/totals[index+j]-1))
    upper={a:{m:quantile(values,plan['upper_quantile'])for m,values in metrics.items()}for a,metrics in distributions.items()}
    record['confidence_upper_delta_pct']=upper
    record['confidence_method']=f"Percentile bootstrap of {len(blocks)} complete six-round blocks; simultaneous family alpha {plan['family_alpha']} across {plan['comparisons']} CPU/wall comparisons."
    record['speed_budget_passed']=all(v<=plan['budget_pct']for group in [record['delta_pct'],upper]for metrics in group.values()for v in metrics.values())
    return record

def main():
    w=Path(sys.argv[1])if len(sys.argv)>1 else Path(__file__).parent;plan=json.loads((w/'timing-randomized-plan.json').read_text());config=json.loads((w/'timing-cases.json').read_text())
    rows=config['cases'];assert plan['samples_per_arm']==192 and plan['comparisons']==len(rows)*4
    assert plan['cases']==[row[0]for row in rows]and plan['execution']=='parallel cases on disjoint physical CPU pairs'
    config['baseline_commit']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=config['base'],text=True).strip()
    allowed=os.sched_getaffinity(0);pairs=set()
    for cpu in sorted(allowed):
        pair=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
        if len(pair)==2 and set(pair)<=allowed:pairs.add(pair)
    assert len(pairs)>=len(rows)
    a=sample();time.sleep(5);b=sample()
    idle={c:100*(b['cpus'][c][3]-a['cpus'][c][3])/sum(y-x for x,y in zip(a['cpus'][c],b['cpus'][c]))for pair in pairs for c in pair}
    selected=sorted(pairs,key=lambda p:(min(idle[c]for c in p),sum(idle[c]for c in p),p),reverse=True)[:len(rows)]
    assignments={row[0]:{'target_cpu':max(pair),'sibling_cpu':min(pair)}for row,pair in zip(rows,selected)}
    assert len({cpu for pair in selected for cpu in pair})==len(rows)*2
    os.environ['KUNA_TIMING_CPU']=str(max(selected[0]));os.environ['KUNA_TIMING_MIN_IDLE']='0'
    runpy.run_path(str(w/'audit-idle.py'),run_name='__main__')
    attempt=w/'timing-randomized-attempts'/time.strftime('%Y%m%d-%H%M%S',time.gmtime());attempt.mkdir(parents=True)
    for name in ['timing-randomized-plan.json','timing-cases.json','cpu-audit.json','owned-process-audit.json']:shutil.copy2(w/name,attempt/name)
    (attempt/'cpu-selection.json').write_text(json.dumps({'assignments':assignments,'idle_pct':idle,'method':plan['cpu_selection']},indent=2)+'\n')
    binaries={side:Path(config[key])/'decompiler/target/release/kuna'for side,key in [('baseline','base'),('patched','root')]};digests={side:sha(path)for side,path in binaries.items()}
    os.sched_setaffinity(0,allowed-{cpu for pair in selected for cpu in pair})
    telemetry=[sample()];done=threading.Event()
    def monitor():
        while not done.wait(0.25):telemetry.append(sample())
    thread=threading.Thread(target=monitor);thread.start()
    try:
        jobs=[(config,plan,row,i,assignments[row[0]]['target_cpu'],digests,str(attempt))for i,row in enumerate(rows)]
        with ProcessPoolExecutor(max_workers=len(rows),mp_context=multiprocessing.get_context('spawn'))as pool:records=list(pool.map(case,jobs))
    finally:
        done.set();thread.join();telemetry.append(sample());(attempt/'cpu-samples.json').write_text(json.dumps(telemetry,indent=2)+'\n')
    assert all(sha(path)==digests[side]for side,path in binaries.items())
    for i,record in enumerate(records):
        inference(record,plan,i);(attempt/f'timing-{record["fixture"]}.json').write_text(json.dumps(record,indent=2)+'\n')
        print(record['fixture'],'upper bounds',record['confidence_upper_delta_pct'],'passed',record['speed_budget_passed'],flush=True)
    accepted=len(records)==len(rows)and all(record['speed_budget_passed']for record in records)
    result={'method':'balanced_randomized_block_bootstrap','execution':plan['execution'],'attempt':str(attempt),'cpu_assignments':assignments,'plan':plan,'accepted_for_speed_review':accepted,'results':{r['fixture']:{'median_delta_pct':r['delta_pct'],'confidence_upper_delta_pct':r['confidence_upper_delta_pct'],'speed_budget_passed':r['speed_budget_passed']}for r in records}}
    (attempt/'monitor.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2),flush=True)
    assert accepted,'Speed bound unresolved or failed; all samples retained without replacing canonical results.'
    for p in attempt.glob('timing-*.json'):shutil.copy2(p,w/p.name)
    shutil.copy2(attempt/'monitor.json',w/'timing-monitor.json')
    print('RANDOMIZED TIMINGS PASSED',flush=True)

if __name__=='__main__':main()
