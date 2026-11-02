from pathlib import Path
from concurrent.futures import ProcessPoolExecutor,as_completed
import importlib.util,os,json,time,hashlib,subprocess,threading,multiprocessing,shutil,statistics

spec=importlib.util.spec_from_file_location('timing','/tmp/kuna-angr-reference/parallel-timing.py')
timing=importlib.util.module_from_spec(spec);spec.loader.exec_module(timing)
def worker(job):return timing.case(job)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def main():
    declaration=Path('/tmp/kuna-angr-reference/feature-confirmation-plan.json');study=json.loads(declaration.read_text())
    workdirs=[Path(p)for p in study['workdirs']];configs=[json.loads((w/'timing-cases.json').read_text())for w in workdirs]
    initials=[w/'timing-randomized-attempts'/study['initial_attempt']for w in workdirs]
    reserved={c for pair in json.loads((initials[0]/'cpu-selection.json').read_text())['assignments'].values()for c in pair.values()}
    allowed=os.sched_getaffinity(0);os.sched_setaffinity(0,allowed-reserved)
    while not all((a/f'timing-{row[0]}.json').exists()for a,config in zip(initials,configs)for row in config['cases']):time.sleep(5)
    plans=[];oldrecords=[];jobspecs=[]
    for feature,(w,config,initial)in enumerate(zip(workdirs,configs,initials)):
        plan=json.loads((initial/'timing-randomized-plan.json').read_text());original_plan=dict(plan)
        plan.update(family_alpha=.025,upper_quantile=1-.025/(len(config['cases'])*4),study_family_alpha=.05,maximum_looks=2,confirmation_additional_rounds=384,confirmation_final_rounds=576,confirmation_declaration=study)
        records={};selected=[]
        for index,row in enumerate(config['cases']):
            record=json.loads((initial/f'timing-{row[0]}.json').read_text());assert record['samples']==192
            timing.inference(record,plan,index);records[row[0]]=record
            if not record['speed_budget_passed']:jobspecs.append((feature,index,row));selected.append(row[0])
        plan['confirmation_cases']=selected;plan['final_samples_by_case']={row[0]:(576 if row[0]in selected else 192)for row in config['cases']}
        (w/'timing-protocol-history/plan-before-final-confirmation.json').write_text(json.dumps(original_plan,indent=2)+'\n')
        (w/'timing-randomized-plan.json').write_text(json.dumps(plan,indent=2)+'\n')
        (w/'first-look-strict-bounds.json').write_text(json.dumps(records,indent=2)+'\n')
        shutil.copy2(declaration,w/declaration.name);shutil.copy2(__file__,w/'feature-confirmation.py')
        plans.append(plan);oldrecords.append(records);print('CONFIRMATION CASES',feature,selected,flush=True)
    pairs=set()
    for cpu in sorted(allowed-reserved):
        pair=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
        if len(pair)==2 and set(pair)<=allowed-reserved:pairs.add(pair)
    assert len(pairs)>len(jobspecs)
    a=timing.sample();time.sleep(5);b=timing.sample()
    idle={c:100*(b['cpus'][c][3]-a['cpus'][c][3])/sum(y-x for x,y in zip(a['cpus'][c],b['cpus'][c]))for pair in pairs for c in pair}
    selected=sorted(pairs,key=lambda p:(min(idle[c]for c in p),sum(idle[c]for c in p),p),reverse=True)[:len(jobspecs)]
    assignments={f'{feature}:{row[0]}':{'target_cpu':max(pair),'sibling_cpu':min(pair)}for(feature,index,row),pair in zip(jobspecs,selected)}
    assert not {c for pair in selected for c in pair}&reserved
    processes={}
    for p in Path('/proc').iterdir():
        if not p.name.isdigit():continue
        try:
            args=(p/'cmdline').read_bytes().decode(errors='replace').split('\0');status=(p/'status').read_text();parent=int(next(line.split()[1]for line in status.splitlines()if line.startswith('PPid:')))
            processes[int(p.name)]=(parent,args)
        except(OSError,StopIteration):continue
    roots={pid for pid,(_,args)in processes.items()if args[:3]==['python3','-u','/tmp/kuna-angr-reference/batch-timing.py']}
    trusted=set(roots)
    for _ in range(10):trusted|={pid for pid,(parent,_)in processes.items()if parent in trusted}
    owned_prefixes=['/home/mahaloz/kwt/angr-rust-','/tmp/kuna-rust-']
    unexpected=[{'pid':pid,'arguments':args}for pid,(_,args)in processes.items()if pid!=os.getpid()and pid not in trusted and args and Path(args[0]).name not in ['bash','timeout','rg']and any(prefix in ' '.join(args)for prefix in owned_prefixes)]
    audit={'allowed_shared_measurement_roots':sorted(roots),'allowed_descendants':sorted(trusted),'unexpected_owned_processes':unexpected};assert not unexpected,audit
    stamp=time.strftime('%Y%m%d-%H%M%S',time.gmtime());attempts=[];digests=[]
    for feature,(w,config)in enumerate(zip(workdirs,configs)):
        attempt=w/'timing-randomized-attempts'/stamp;attempt.mkdir(parents=True);attempts.append(attempt)
        for name in ['timing-randomized-plan.json','timing-cases.json','feature-confirmation-plan.json','feature-confirmation.py']:shutil.copy2(w/name,attempt/name)
        (attempt/'owned-process-audit.json').write_text(json.dumps(audit,indent=2)+'\n')
        (attempt/'cpu-selection.json').write_text(json.dumps({'assignments':assignments,'reserved_initial_pairs':sorted(reserved),'idle_pct':idle},indent=2)+'\n')
        hashes={side:sha(Path(config[key])/'decompiler/target/release/kuna')for side,key in [('baseline','base'),('patched','root')]};digests.append(hashes)
        assert all(r['binaries_sha256']==hashes for r in oldrecords[feature].values())
        config['baseline_commit']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=config['base'],text=True).strip()
    os.sched_setaffinity(0,allowed-reserved-{c for pair in selected for c in pair})
    telemetry=[timing.sample()];done=threading.Event()
    def monitor():
        while not done.wait(.25):telemetry.append(timing.sample())
    thread=threading.Thread(target=monitor);thread.start();extra=[{},{}]
    try:
        with ProcessPoolExecutor(max_workers=max(1,len(jobspecs)),mp_context=multiprocessing.get_context('spawn'))as pool:
            futures={}
            for feature,index,row in jobspecs:
                plan=plans[feature]|{'samples_per_arm':384,'seed':study['seeds'][feature]}
                cpu=assignments[f'{feature}:{row[0]}']['target_cpu'];futures[pool.submit(worker,(configs[feature],plan,row,index,cpu,digests[feature],str(attempts[feature])))]=(feature,row[0])
            for future in as_completed(futures):
                feature,name=futures[future];extra[feature][name]=future.result();print('CONFIRMATION FINISHED',feature,name,flush=True)
    finally:
        done.set();thread.join();telemetry.append(timing.sample());raw=json.dumps(telemetry,indent=2)+'\n'
        for attempt in attempts:(attempt/'cpu-samples.json').write_text(raw)
    while not all((a/'monitor.json').exists()for a in initials):time.sleep(5)
    accepted_features=[]
    for feature,(w,config,plan,initial,attempt)in enumerate(zip(workdirs,configs,plans,initials,attempts)):
        assert all(sha(Path(config[key])/'decompiler/target/release/kuna')==digests[feature][side]for side,key in [('baseline','base'),('patched','root')])
        results=[]
        for index,row in enumerate(config['cases']):
            name=row[0];old=oldrecords[feature][name];record=dict(old);record['original_plan']=old['plan'];record['plan']=plan
            record['study_looks']=2;record['family_alpha_each_look']=.025;record['included_runs']=[str(initial)]
            if name in extra[feature]:
                new=extra[feature][name];assert new['sha256']==old['sha256'];record['samples']=576
                record['raw']={arm:old['raw'][arm]+new['raw'][arm]for arm in old['raw']};record['orders']=old['orders']+new['orders']
                record['affinity_cpu_by_run']=[old['affinity_cpu'],new['affinity_cpu']];record['included_runs'].append(str(attempt))
                record['medians']={arm:{m:statistics.median(x[m]for x in xs)for m in ['cpu_ms','wall_ms']}for arm,xs in record['raw'].items()}
                record['means']={arm:{m:statistics.mean(x[m]for x in xs)for m in ['cpu_ms','wall_ms']}for arm,xs in record['raw'].items()}
                record['delta_pct']={arm+'_to_patched':{m:100*(record['medians']['patched'][m]/record['medians'][arm][m]-1)for m in ['cpu_ms','wall_ms']}for arm in ['baseline','off']}
            record['baseline_has_pending_feature']=True;record['baseline_manifest_sha256']=sha(w/'parent-source-before-commit.json')
            timing.inference(record,plan|{'samples_per_arm':record['samples']},index)
            (attempt/f'combined-{name}.json').write_text(json.dumps(record,indent=2)+'\n');results.append(record)
            print('CONFIRMATION RESULT',feature,name,record['confidence_upper_delta_pct'],record['speed_budget_passed'],flush=True)
        accepted=all(r['speed_budget_passed']for r in results);accepted_features.append(accepted)
        result={'method':'balanced_randomized_block_bootstrap','execution':'One fixed confirmation with all initial samples retained; physical CPU pairs disjoint from the shared first study','attempt':str(attempt),'cpu_assignments':assignments,'plan':plan,'accepted_for_speed_review':accepted,'baseline_commit':config['baseline_commit'],'baseline_has_pending_feature':True,'results':{r['fixture']:{'samples':r['samples'],'median_delta_pct':r['delta_pct'],'confidence_upper_delta_pct':r['confidence_upper_delta_pct'],'speed_budget_passed':r['speed_budget_passed']}for r in results}}
        (attempt/'monitor.json').write_text(json.dumps(result,indent=2)+'\n')
        if accepted:
            for r in results:(w/f'timing-{r["fixture"]}.json').write_text(json.dumps(r,indent=2)+'\n')
            shutil.copy2(attempt/'monitor.json',w/'timing-monitor.json')
        print('FEATURE CONFIRMATION',feature,'PASSED'if accepted else'UNRESOLVED',flush=True)
    assert all(accepted_features),'Fixed final confirmation unresolved; no further repeat authorized by this study.'
    print('FEATURE CONFIRMATIONS PASSED',flush=True)

if __name__=='__main__':main()
