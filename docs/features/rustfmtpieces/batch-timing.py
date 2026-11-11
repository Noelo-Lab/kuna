from pathlib import Path
from concurrent.futures import ProcessPoolExecutor,as_completed
import importlib.util,os,json,time,hashlib,subprocess,threading,multiprocessing,shutil,runpy

spec=importlib.util.spec_from_file_location('timing', '/tmp/kuna-angr-reference/parallel-timing.py')
timing=importlib.util.module_from_spec(spec);spec.loader.exec_module(timing)

def worker(job):
    return timing.case(job)

def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def main():
    declaration=Path('/tmp/kuna-angr-reference/batch-timing-plan.json')
    batch=json.loads(declaration.read_text());allowed=os.sched_getaffinity(0)
    workdirs=[Path(p)for p in batch['workdirs']];configs=[];plans=[];jobspecs=[]
    for feature,w in enumerate(workdirs):
        config=json.loads((w/'timing-cases.json').read_text());plan=json.loads((w/'timing-randomized-plan.json').read_text())
        config['baseline_commit']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=config['base'],text=True).strip()
        config['baseline_source_pending']=feature!=0
        configs.append(config);plans.append(plan)
        for index,row in enumerate(config['cases']):
            if feature==0 and row[0] not in batch['parent_confirmation_cases']:continue
            jobspecs.append((feature,index,row))
    pairs=set()
    for cpu in sorted(allowed):
        pair=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
        if len(pair)==2 and set(pair)<=allowed:pairs.add(pair)
    assert len(pairs)>len(jobspecs)
    a=timing.sample();time.sleep(5);b=timing.sample()
    idle={c:100*(b['cpus'][c][3]-a['cpus'][c][3])/sum(y-x for x,y in zip(a['cpus'][c],b['cpus'][c]))for pair in pairs for c in pair}
    selected=sorted(pairs,key=lambda p:(min(idle[c]for c in p),sum(idle[c]for c in p),p),reverse=True)[:len(jobspecs)]
    assignments={f'{feature}:{row[0]}':{'target_cpu':max(pair),'sibling_cpu':min(pair)}for(feature,index,row),pair in zip(jobspecs,selected)}
    os.environ['KUNA_TIMING_CPU']=str(max(selected[0]));os.environ['KUNA_TIMING_MIN_IDLE']='0'
    runpy.run_path(str(workdirs[0]/'audit-idle.py'),run_name='__main__')
    stamp=time.strftime('%Y%m%d-%H%M%S',time.gmtime());attempts=[];digests=[]
    for feature,(w,config)in enumerate(zip(workdirs,configs)):
        attempt=w/'timing-randomized-attempts'/stamp;attempt.mkdir(parents=True);attempts.append(attempt)
        for name in ['timing-randomized-plan.json','timing-cases.json']:shutil.copy2(w/name,attempt/name)
        for name in ['cpu-audit.json','owned-process-audit.json']:
            shutil.copy2(workdirs[0]/name,attempt/name)
            if feature:shutil.copy2(workdirs[0]/name,w/name)
        shutil.copy2(declaration,attempt/declaration.name);shutil.copy2(__file__,attempt/'batch-timing.py')
        (attempt/'cpu-selection.json').write_text(json.dumps({'assignments':assignments,'idle_pct':idle},indent=2)+'\n')
        digests.append({side:sha(Path(config[key])/'decompiler/target/release/kuna')for side,key in [('baseline','base'),('patched','root')]})
    os.sched_setaffinity(0,allowed-{c for pair in selected for c in pair})
    telemetry=[timing.sample()];done=threading.Event()
    def monitor():
        while not done.wait(0.25):telemetry.append(timing.sample())
    thread=threading.Thread(target=monitor);thread.start();records=[{}, {}, {}]
    try:
        with ProcessPoolExecutor(max_workers=len(jobspecs),mp_context=multiprocessing.get_context('spawn'))as pool:
            futures={}
            for feature,index,row in jobspecs:
                plan=dict(plans[feature])
                if feature==0:plan['samples_per_arm']=batch['parent_additional_rounds'];plan['seed']=batch['parent_confirmation_seed']
                cpu=assignments[f'{feature}:{row[0]}']['target_cpu']
                futures[pool.submit(worker,(configs[feature],plan,row,index,cpu,digests[feature],str(attempts[feature])))]=(feature,index,row)
            for future in as_completed(futures):
                feature,index,row=futures[future];record=future.result();records[feature][row[0]]=record
                print('FINISHED',feature,row[0],flush=True)
    finally:
        done.set();thread.join();telemetry.append(timing.sample());raw=json.dumps(telemetry,indent=2)+'\n'
        for attempt in attempts:(attempt/'cpu-samples.json').write_text(raw)
    for feature,config in enumerate(configs):
        assert all(sha(Path(config[key])/'decompiler/target/release/kuna')==digests[feature][side]for side,key in [('baseline','base'),('patched','root')])
    import statistics
    for feature,(w,config,plan,attempt)in enumerate(zip(workdirs,configs,plans,attempts)):
        results=[]
        for index,row in enumerate(config['cases']):
            name=row[0]
            if feature==0:
                original=json.loads((Path(batch['parent_initial_attempt'])/f'timing-{name}.json').read_text())
                assert original['binaries_sha256']==digests[feature]
                record=dict(original);record['original_plan']=original['plan'];record['plan']=plan
                record['study_looks']=2;record['family_alpha_each_look']=0.025
                if name in records[feature]:
                    extra=records[feature][name];assert extra['sha256']==original['sha256']
                    record['raw']={arm:original['raw'][arm]+extra['raw'][arm]for arm in original['raw']}
                    record['orders']=original['orders']+extra['orders'];record['samples']=768
                    record['affinity_cpu_by_run']=[original['affinity_cpu'],extra['affinity_cpu']]
                    record['included_runs']=[batch['parent_initial_attempt'],str(attempt)]
                    record['medians']={arm:{metric:statistics.median(x[metric]for x in xs)for metric in ['cpu_ms','wall_ms']}for arm,xs in record['raw'].items()}
                    record['means']={arm:{metric:statistics.mean(x[metric]for x in xs)for metric in ['cpu_ms','wall_ms']}for arm,xs in record['raw'].items()}
                    record['delta_pct']={arm+'_to_patched':{metric:100*(record['medians']['patched'][metric]/record['medians'][arm][metric]-1)for metric in ['cpu_ms','wall_ms']}for arm in ['baseline','off']}
                else:
                    record['included_runs']=[batch['parent_initial_attempt']]
                inference_plan=plan|{'samples_per_arm':record['samples']}
                timing.inference(record,inference_plan,index)
            else:
                record=records[feature][name];record['baseline_has_pending_feature']=True
                record['baseline_manifest_sha256']=sha(w/'parent-source-before-commit.json')
                timing.inference(record,plan,index)
            (attempt/f'combined-{name}.json').write_text(json.dumps(record,indent=2)+'\n')
            results.append(record)
            print('RESULT',feature,name,record['confidence_upper_delta_pct'],record['speed_budget_passed'],flush=True)
        accepted=all(r['speed_budget_passed']for r in results)
        result={'method':'balanced_randomized_block_bootstrap','execution':batch['execution'],'attempt':str(attempt),'cpu_assignments':assignments,'plan':plan,'accepted_for_speed_review':accepted,'baseline_commit':config['baseline_commit'],'baseline_has_pending_feature':feature!=0,'results':{r['fixture']:{'samples':r['samples'],'median_delta_pct':r['delta_pct'],'confidence_upper_delta_pct':r['confidence_upper_delta_pct'],'speed_budget_passed':r['speed_budget_passed']}for r in results}}
        (attempt/'monitor.json').write_text(json.dumps(result,indent=2)+'\n')
        if accepted:
            for record in results:(w/f'timing-{record["fixture"]}.json').write_text(json.dumps(record,indent=2)+'\n')
            shutil.copy2(attempt/'monitor.json',w/'timing-monitor.json')
        print('FEATURE',feature,'PASSED'if accepted else'UNRESOLVED',flush=True)
    assert all((a/'monitor.json').exists()and json.loads((a/'monitor.json').read_text())['accepted_for_speed_review']for a in attempts)
    print('BATCH TIMINGS PASSED',flush=True)

if __name__=='__main__':main()
