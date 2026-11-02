import fcntl
_capture_lock=open('/tmp/kuna-rust-fmt-str/breadth-exclusive.lock','w')
fcntl.flock(_capture_lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import subprocess,os,json,hashlib,difflib,re,sys
root=Path('/home/mahaloz/kwt/angr-rust-fmt-str');base=Path('/home/mahaloz/kwt/angr-rust-fmt-pieces')
old=Path('/tmp/kuna-rust-fmt-pieces-f874/breadth');out=Path('/tmp/kuna-rust-fmt-str/breadth');out.mkdir(exist_ok=True)
refs=Path('/tmp/kuna-angr-reference/binaries')
capture_only='--capture-only'in sys.argv
marker=out/'overlap-capture-plan.json'
if not capture_only and marker.exists():
    import time
    while not(out/'capture-ready.json').exists():time.sleep(1)
    ready=json.loads((out/'capture-ready.json').read_text())
    assert ready['patched_kuna_sha256']==hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest()
    assert ready['baseline_kuna_sha256']==hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest()
    if '--review-existing'not in sys.argv:sys.argv.append('--review-existing')
patched_sha=hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest()
baseline_sha=hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest()
parent_source=json.loads(Path('/tmp/kuna-rust-fmt-str/parent-source-before-commit.json').read_text())
assert parent_source['binary_sha256']==baseline_sha
assert all(hashlib.sha256((base/p).read_bytes()).hexdigest()==h for p,h in parent_source['files'].items())
inputs = [
    ('hermetic', root/'tests/fixtures/go-dwarf-signatures/compressed', ['c','go']),
    ('hermetic-plain', root/'tests/fixtures/go-dwarf-signatures/uncompressed', ['c','go']),
    ('native', Path('/tmp/kuna-go-output/output-go'), ['c','go']),
    ('basics122', refs/'tests/x86_64/go/go1.22.5/basics', ['c','go']),
    ('basics127', refs/'tests/x86_64/go/go1.27.1/basics', ['c','go']),
    ('langdetect', refs/'tests/x86_64/langdetect_go', ['c','go']),
    ('stripped122', refs/'tests/x86_64/go/go1.22.5/basics_stripped', ['c','go']),
    ('native-c', root/'tests/fixtures/go-output/output-c', ['c','rust','go']),
    ('grep', root/'tests/bug-repro/grep', ['c','rust']),
    ('rust', refs/'rust-fmtdemo-1931', ['c','rust']),
    ('entry-ranges', root/'tests/fixtures/go-dwarf-entry-ranges/entry-ranges', ['c','go']),
    ('native1273', Path('/tmp/kuna-go-dwarf-signatures/native1273'), ['c','go']),
]
inputs += [('entry-plain', root/'tests/fixtures/go-dwarf-entry-ranges/entry-ranges-plain', ['c','go'])]
inputs += [(name, Path('/tmp/kuna-go-dwarf-formals-5e05-review')/name, ['c','go']) for name in ['native-plain','native1273-plain','basics127-plain']]

inputs += [('rust2025', refs/'rust-fmt-2025-O3', ['c','rust']), ('native-rust', Path('/tmp/kuna-rust-extensions-c281-review/native-1.93.1'), ['c','rust'])]


inputs += [(name, root/'decompiler/crates/kuna-analysis/tests/fixtures'/fixture, ['c','rust','go']) for name,fixture in [('panic-native','rust_fmt_panic_real_x86_64'),('panic-shapes','rust_fmt_panic_x86_64')]]

inputs += [("pieces-"+version, root/'decompiler/crates/kuna-analysis/tests/fixtures'/('rust_fmt_pieces_'+version), ['c','rust','go']) for version in ['1.89.0','1.90.0','nightly-2025-05-22','x86_64']]
inputs += [('str-native',root/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_str_real_x86_64',['c','rust','go']),('str-nonlto',Path('/tmp/kuna-rust-fmt-str/native-nonlto'),['c','rust','go'])]

exec(Path('/tmp/kuna-rust-fmt-str/evidence.py').read_text())

def run(row):
    name,binary,languages=row;data=binary.read_bytes();digest=hashlib.sha256(data).hexdigest();memory=Image(data);symbols=demangled_symbols(binary)
    previous=out/f'{name}-record.json'
    reuse='--reuse-existing'in sys.argv and previous.exists()
    if reuse:
        record=json.loads(previous.read_text());assert record['sha256']==digest and record['patched_kuna_sha256']==patched_sha and record['baseline_kuna_sha256']==baseline_sha
    cached=(old/f'{name}-record.json').exists()
    if cached:
        assert json.loads((old/f'{name}-record.json').read_text())['sha256']==digest
        if not capture_only:assert json.loads((old/'record.json').read_text())['patched_kuna_sha256']==baseline_sha
    def capture(executable,language,destination,off=False):
        with destination.open('w') as f,destination.with_suffix('.log').open('w') as log:
            command=['taskset','-c','0-78',str(executable),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
            if off:command+=['--option','rustfmtstr','off']
            subprocess.run(command,cwd=root,env=os.environ|{'KUNA_SPECS':str(root/'specs')},stdout=f,stderr=log,check=True)
    def language_capture(language):
        before=old/f'{name}-{language}.json'if cached else out/f'{name}-{language}-before.json'
        after=out/f'{name}-{language}.json';off=out/f'{name}-{language}-off.json'
        if '--review-existing'not in sys.argv and not reuse:
            if not cached:capture(base/'decompiler/target/release/kuna',language,before)
            capture(root/'decompiler/target/release/kuna',language,after)
            if language=='rust':capture(root/'decompiler/target/release/kuna',language,off,True)
        return language,before,after,off
    with ThreadPoolExecutor(max_workers=3)as pool:captures=list(pool.map(language_capture,languages))
    result={'fixture':name,'sha256':digest,'patched_kuna_sha256':patched_sha,'baseline_kuna_sha256':baseline_sha,'languages':[]}
    if capture_only:
        result['capture_only']=True
        result['languages']=[{'language':language,'after_file_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'off_file_sha256':hashlib.sha256(off.read_bytes()).hexdigest()if language=='rust'else None}for language,prior,path,off in captures]
        print('CAPTURE READY',name,flush=True)
        return result
    if marker.exists():
        expected=next(f for f in ready['fixtures']if f['fixture']==name)
        assert expected['sha256']==digest
        for language,prior,path,off in captures:
            row=next(l for l in expected['languages']if l['language']==language)
            assert hashlib.sha256(path.read_bytes()).hexdigest()==row['after_file_sha256']
            if language=='rust':assert hashlib.sha256(off.read_bytes()).hexdigest()==row['off_file_sha256']
    for language,prior,path,off in captures:
        a={x['address']:x for x in json.loads(prior.read_text())['functions']};b={x['address']:x for x in json.loads(path.read_text())['functions']};assert a.keys()==b.keys()
        previous_capture=Path('/tmp/kuna-rust-fmt-str-pre-candidate-index/breadth')/f'{name}-{language}.json'
        previous_functions={x['address']:x for x in json.loads(previous_capture.read_text())['functions']}
        assert b==previous_functions,('candidate indexing changed reviewed output',name,language)
        if language=='rust':
            ablation={x['address']:x for x in json.loads(off.read_text())['functions']};assert a==ablation,('off differs from parent',name)
            verifier=subprocess.run(['/tmp/kuna-rust-fmt-str/verifier/target/debug/rust-str-verifier',str(prior),str(path)],capture_output=True,text=True)
            (out/f'{name}-ast.log').write_text(verifier.stderr);assert verifier.returncode==0,(name,verifier.stderr)
            proofs=json.loads(verifier.stdout)
        changes=[];patch=[]
        for addr in a:
            assert not a[addr].get('error')and not b[addr].get('error')
            if a[addr]==b[addr]:continue
            assert language=='rust',(name,language,hex(addr))
            assert {k:v for k,v in a[addr].items()if k not in ['code','line_mappings','variables']}=={k:v for k,v in b[addr].items()if k not in ['code','line_mappings','variables']}
            variables=lambda e:[{k:v for k,v in v.items()if k not in ['line_numbers','addresses']}for v in e['variables']]
            assert variables(a[addr])==variables(b[addr]),('variable metadata',name,hex(addr))
            addresses=lambda e:{address for line in e['line_mappings']for address in line['addresses']}
            aa,bb=addresses(a[addr]),addresses(b[addr]);proof=proofs[hex(addr)]
            call_lines={line for site in proof['changes']for line in range(site['before']['source_line_start'],site['before']['source_line_end']+1)}
            call_addresses={address for line in a[addr]['line_mappings']if line['line_number']in call_lines for address in line['addresses']}
            assert aa-bb<=call_addresses,('lost address outside call',name,hex(addr));assert not bb-aa,('new global mapping',name,hex(addr))
            arguments={identifier for site in proof['changes']for arg in site['after']['parameters']for key in ['pointer','data','length']for identifier in re.findall(r'\b[A-Za-z_][A-Za-z_0-9]*\b',arg.get(key,''))}
            arguments.update(identifier for site in proof['changes']for identifier in re.findall(r'\b[A-Za-z_][A-Za-z_0-9]*\b',site['after'].get('destination')or''))
            variable_mappings=[]
            for x,y in zip(a[addr]['variables'],b[addr]['variables']):
                if x['addresses']==y['addresses']:continue
                added=set(y['addresses'])-set(x['addresses']);lost=set(x['addresses'])-set(y['addresses'])
                spelling=x['name']
                if x['kind']=='arg':
                    spelling='a'+str(x['arg_index']);assert re.search(r'\b'+spelling+r'\s*:',a[addr]['code'])
                assert spelling in arguments,(name,hex(addr),x['name'],arguments)
                assert added<=call_addresses and lost<=call_addresses,(name,hex(addr),added,lost,call_addresses)
                variable_mappings.append({'name':x['name'],'added_call_addresses':[hex(v)for v in sorted(added)],'lost_call_addresses':[hex(v)for v in sorted(lost)]})
            for site in proof['changes']:
                count=len(site['after']['parameters']);evidence=out/f'{name}-{addr:x}-{count}-descriptor.jsonl'
                if not evidence.exists():
                    evidence.write_bytes(subprocess.check_output(['/tmp/kuna-rust-fmt-str/descriptor-evidence',str(binary),hex(addr),str(count)]))
                calls=[json.loads(line)for line in evidence.read_text().splitlines()]
                verify_site(site,a[addr],calls,memory,symbols)
            diff=list(difflib.unified_diff(a[addr]['code'].splitlines(True),b[addr]['code'].splitlines(True),fromfile='before/'+a[addr]['name'],tofile='after/'+b[addr]['name']))
            changes.append({'address':hex(addr),'name':b[addr]['name'],'proof':proof,'source_mapping':{'added_instruction_addresses':[hex(x)for x in sorted(bb-aa)],'lost_instruction_addresses':[hex(x)for x in sorted(aa-bb)],'lost_addresses_belong_to_replaced_call_lines':True,'variable_mapping_changes':variable_mappings,'declarations_and_function_metadata_unchanged':True},'hunks':[{'range':line.strip(),'classification':'only runtime formatting calls replaced; every other AST node and comment unchanged; template bytes, formatter identities and original string-data slots independently verified'}for line in diff if line.startswith('@@')]})
            patch+=diff+['\n']
        parsed=0
        if language=='go':
            p=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True,text=True);assert p.returncode==0,p.stdout+p.stderr;parsed=len(b)
        (out/f'{name}-{language}.diff').write_text(''.join(patch))
        result['languages'].append({'language':language,'functions':len(b),'changed':changes,'parsed':parsed,'errors':0,'off_equals_parent':language=='rust'})
    (out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n')
    print(name,[(x['language'],x['functions'],len(x['changed']))for x in result['languages']],flush=True)
    return result

partial=os.environ.get('KUNA_BREADTH_ONLY')
if partial:inputs=[r for r in inputs if r[0]in partial.split(',')]
results=[]
with ThreadPoolExecutor(max_workers=8)as pool:
    for future in as_completed([pool.submit(run,row)for row in inputs]):results.append(future.result())
assert patched_sha==hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest()
assert baseline_sha==hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest()
(out/('capture-ready.json'if capture_only else'record-partial.json'if partial else'record.json')).write_text(json.dumps({'patched_kuna_sha256':patched_sha,'baseline_kuna_sha256':baseline_sha,'baseline_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip(),'baseline_has_pending_feature':True,'options':{'godwarfsig':'on'},'semantic_evidence_pending':True,'fixtures':results},indent=2)+'\n')
