from pathlib import Path
from concurrent.futures import ProcessPoolExecutor,as_completed
import importlib.util,os,json,time,hashlib,subprocess,threading,multiprocessing,shutil,gzip
spec=importlib.util.spec_from_file_location('timing','/tmp/kuna-angr-reference/parallel-timing.py')
timing=importlib.util.module_from_spec(spec);spec.loader.exec_module(timing)
def worker(job):return timing.case(job)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def main():
    workdirs=[Path('/tmp/kuna-rust-fmt-'+slug)for slug in ['pieces-f874','str','write']]
    configs=[json.loads((w/'timing-cases.json').read_text())for w in workdirs]
    plan_names=['timing-candidate-index-plan.json','timing-candidate-index-plan.json','timing-plan.json']
    plans=[json.loads((w/name).read_text())for w,name in zip(workdirs,plan_names)]
    assert 'ALL CANDIDATE-INDEX CORRECTNESS AND COMPLETE OUTPUT EQUIVALENCE PASSED'in Path('/tmp/kuna-angr-reference/validate-candidate-index.log').read_text()
    assert 'BOTH CANDIDATE-INDEX FEATURES READY FOR DECLARED TIMINGS' in Path('/tmp/kuna-angr-reference/breadth-69fa-recovery.log').read_text()
    assert 'WRITE RECOVERY READY FOR DECLARED TIMINGS' in (workdirs[2]/'breadth-69fa.log').read_text()
    unexpected=[]
    for p in Path('/proc').iterdir():
        if not p.name.isdigit()or int(p.name)==os.getpid():continue
        try:args=(p/'cmdline').read_bytes().decode(errors='replace').split('\0')
        except OSError:continue
        if args and Path(args[0]).name not in ['bash','timeout','rg']and any(prefix in ' '.join(args)for prefix in ['/home/mahaloz/kwt/angr-rust-','/tmp/kuna-rust-']):unexpected.append({'pid':int(p.name),'args':args})
    assert not unexpected,unexpected
    allowed=os.sched_getaffinity(0);pairs=set()
    for cpu in sorted(allowed):
        pair=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
        if len(pair)==2 and set(pair)<=allowed:pairs.add(pair)
    jobspecs=[(feature,index,row)for feature,config in enumerate(configs)for index,row in enumerate(config['cases'])]
    a=timing.sample();time.sleep(5);b=timing.sample()
    idle={c:100*(b['cpus'][c][3]-a['cpus'][c][3])/sum(y-x for x,y in zip(a['cpus'][c],b['cpus'][c]))for pair in pairs for c in pair}
    selected=sorted(pairs,key=lambda p:(min(idle[c]for c in p),sum(idle[c]for c in p),p),reverse=True)[:len(jobspecs)]
    assert len(selected)==len(jobspecs)
    assignments={f'{feature}:{row[0]}':{'target_cpu':max(pair),'sibling_cpu':min(pair)}for(feature,index,row),pair in zip(jobspecs,selected)}
    stamp=time.strftime('%Y%m%d-%H%M%S',time.gmtime());attempts=[];digests=[]
    for feature,(w,config,plan)in enumerate(zip(workdirs,configs,plans)):
        assert plan['samples_per_arm']==768 and plan['comparisons']==len(config['cases'])*4 and plan['maximum_looks']==1
        assert plan['cases']==[r[0]for r in config['cases']]
        source=json.loads((w/'production-source-before-validation.json').read_text())
        assert all(sha(Path(config['root'])/p)==h for p,h in source.items())
        parent=json.loads((w/'parent-source-before-commit.json').read_text())
        assert all(sha(Path(config['base'])/p)==h for p,h in parent['files'].items())
        hashes={side:sha(Path(config[key])/'decompiler/target/release/kuna')for side,key in [('baseline','base'),('patched','root')]}
        assert hashes['baseline']==parent['binary_sha256'];digests.append(hashes)
        config['baseline_commit']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=config['base'],text=True).strip()
        config['baseline_has_pending_feature']=bool(subprocess.check_output(['git','status','--porcelain'],cwd=config['base']))
        attempt=w/'timing-randomized-attempts'/stamp;attempt.mkdir(parents=True);attempts.append(attempt)
        (w/'timing-randomized-plan.json').write_text(json.dumps(plan,indent=2)+'\n')
        for name in ['timing-randomized-plan.json','timing-cases.json',plan_names[feature]]:shutil.copy2(w/name,attempt/name)
        shutil.copy2(__file__,attempt/'timing-candidate-index.py');shutil.copy2(__file__,w/'timing-candidate-index.py')
        (attempt/'owned-process-audit.json').write_text(json.dumps({'unexpected_owned_processes':unexpected},indent=2)+'\n')
        (attempt/'cpu-selection.json').write_text(json.dumps({'assignments':assignments,'idle_pct':idle},indent=2)+'\n')
    os.sched_setaffinity(0,allowed-{c for pair in selected for c in pair})
    telemetry=[timing.sample()];done=threading.Event()
    def monitor():
        while not done.wait(.25):telemetry.append(timing.sample())
    thread=threading.Thread(target=monitor);thread.start();results=[[]for _ in workdirs]
    try:
        with ProcessPoolExecutor(max_workers=len(jobspecs),mp_context=multiprocessing.get_context('spawn'))as pool:
            futures={pool.submit(worker,(configs[f],plans[f],row,index,assignments[f'{f}:{row[0]}']['target_cpu'],digests[f],str(attempts[f]))):(f,index)for f,index,row in jobspecs}
            for future in as_completed(futures):
                f,index=futures[future];record=future.result();record['baseline_has_pending_feature']=configs[f]['baseline_has_pending_feature'];record['baseline_manifest_sha256']=sha(workdirs[f]/'parent-source-before-commit.json');results[f].append((index,record))
                print('CASE SAMPLING COMPLETE',f,record['fixture'],flush=True)
    finally:
        done.set();thread.join();telemetry.append(timing.sample())
        with gzip.open(attempts[0]/'cpu-samples.json.gz','wt')as file:json.dump(telemetry,file)
        for attempt in attempts[1:]:shutil.copy2(attempts[0]/'cpu-samples.json.gz',attempt/'cpu-samples.json.gz')
    accepted_features=[]
    for f,(w,config,plan,attempt)in enumerate(zip(workdirs,configs,plans,attempts)):
        assert all(sha(Path(config[key])/'decompiler/target/release/kuna')==digests[f][side]for side,key in [('baseline','base'),('patched','root')])
        records=[]
        for index,record in sorted(results[f]):
            timing.inference(record,plan,index);records.append(record);(attempt/f'timing-{record["fixture"]}.json').write_text(json.dumps(record,indent=2)+'\n')
            print('FINAL BOUNDS',f,record['fixture'],record['confidence_upper_delta_pct'],record['speed_budget_passed'],flush=True)
        accepted=len(records)==len(config['cases'])and all(r['speed_budget_passed']for r in records);accepted_features.append(accepted)
        result={'method':'balanced_randomized_block_bootstrap','execution':plan['execution'],'attempt':str(attempt),'cpu_assignments':assignments,'plan':plan,'accepted_for_speed_review':accepted,'baseline_commit':config['baseline_commit'],'baseline_has_pending_feature':config['baseline_has_pending_feature'],'results':{r['fixture']:{'samples':r['samples'],'median_delta_pct':r['delta_pct'],'confidence_upper_delta_pct':r['confidence_upper_delta_pct'],'speed_budget_passed':r['speed_budget_passed']}for r in records}}
        (attempt/'monitor.json').write_text(json.dumps(result,indent=2)+'\n')
        if accepted:
            for record in records:shutil.copy2(attempt/f'timing-{record["fixture"]}.json',w/f'timing-{record["fixture"]}.json')
            shutil.copy2(attempt/'monitor.json',w/'timing-monitor.json')
            shutil.copy2(attempt/'cpu-selection.json',w/'cpu-audit.json')
            shutil.copy2(attempt/'owned-process-audit.json',w/'owned-process-audit.json')
        print('FEATURE FINAL',f,'PASS'if accepted else'UNRESOLVED',flush=True)
    assert all(accepted_features),'Candidate-index study did not clear all final bounds; no repeat authorized for unchanged code.'
    print('ALL CANDIDATE-INDEX TIMINGS PASSED',flush=True)
if __name__=='__main__':main()
