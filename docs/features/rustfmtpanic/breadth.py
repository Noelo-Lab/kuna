from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
import subprocess, os, json, hashlib, difflib, re, sys

root = Path('/home/mahaloz/kwt/angr-rust-panic-main')
base = Path('/home/mahaloz/kwt/angr-rust-panic-base-d850')
old = Path('/tmp/kuna-rust-panic-d850/empty-cache')
out = Path('/tmp/kuna-rust-panic-d850/breadth')
out.mkdir(exist_ok=True)
patched_sha=hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest()
refs = Path('/tmp/kuna-angr-reference/binaries')
sha = 'd85071a68e917d609d7252004da015da06321b02'
assert subprocess.check_output(['git', 'rev-parse', 'HEAD^{tree}'], cwd=base) == subprocess.check_output(['git', 'rev-parse', sha+'^{tree}'], cwd=base)
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

def image_read(data, address, size):
    import struct
    assert data[:6] == b'\x7fELF\x02\x01'
    phoff=struct.unpack_from('<Q',data,32)[0]
    entsize,count=struct.unpack_from('<HH',data,54)
    for i in range(count):
        kind,flags,offset,vaddr,paddr,filesz,memsz,align=struct.unpack_from('<IIQQQQQQ',data,phoff+i*entsize)
        if kind==1 and vaddr<=address and address+size<=vaddr+filesz:
            return data[offset+address-vaddr:offset+address-vaddr+size]
    raise AssertionError(('not file-backed',hex(address),size))

def run(row):
    name,binary,languages=row;data=binary.read_bytes();digest=hashlib.sha256(data).hexdigest()
    cached=(old/f'{name}-record.json').exists()
    if cached: assert json.loads((old/f'{name}-record.json').read_text())['sha256']==digest
    def capture(executable,language,destination,off=False):
        with destination.open('w') as f,destination.with_suffix('.log').open('w') as log:
            cmd=['taskset','-c','0-78',str(executable),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
            if off: cmd += ['--option','rustfmtpanic','off']
            subprocess.run(cmd,cwd=root,env=os.environ|{'KUNA_SPECS':str(root/'specs')},stdout=f,stderr=log,check=True)
    def language_capture(language):
        before=(old if cached else out)/f'{name}-{language}.json' if cached else out/f'{name}-{language}-before.json'
        after=out/f'{name}-{language}.json';off=out/f'{name}-{language}-off.json'
        if '--review-existing' not in sys.argv:
            if not cached: capture(base/'decompiler/target/release/kuna',language,before)
            capture(root/'decompiler/target/release/kuna',language,after)
            if language=='rust': capture(root/'decompiler/target/release/kuna',language,off,True)
        return language,before,after,off
    with ThreadPoolExecutor(max_workers=3) as pool:
        captures=list(pool.map(language_capture,languages))
    result={'fixture':name,'sha256':digest,'languages':[]}
    for language,prior,path,off in captures:
        a={x['address']:x for x in json.loads(prior.read_text())['functions']}
        b={x['address']:x for x in json.loads(path.read_text())['functions']};assert a.keys()==b.keys()
        former={x['address']:x for x in json.loads((Path('/tmp/kuna-rust-panic/breadth')/f'{name}-{language}.json').read_text())['functions']};assert b==former,('integrated output differs from previously reviewed panic output',name,language)
        if language=='rust':
            ablation={x['address']:x for x in json.loads(off.read_text())['functions']}
            assert a==ablation, ('option off differs from parent',name)
            verifier=subprocess.run(['/tmp/kuna-rust-panic/verifier/target/debug/rust-panic-verifier',str(prior),str(path)],capture_output=True,text=True)
            (out/f'{name}-ast.log').write_text(verifier.stderr)
            assert verifier.returncode==0, verifier.stderr
            proofs=json.loads(verifier.stdout)
        changes=[];patch=[]
        for addr in a:
            assert not a[addr].get('error') and not b[addr].get('error')
            if a[addr]==b[addr]:continue
            assert language=='rust'
            assert {k:v for k,v in a[addr].items() if k not in ['code','line_mappings','variables']}=={k:v for k,v in b[addr].items() if k not in ['code','line_mappings','variables']}
            variables=lambda e:[{k:v for k,v in v.items() if k not in ['line_numbers','addresses']} for v in e['variables']]
            assert variables(a[addr])==variables(b[addr]), ('variable metadata',name,hex(addr))
            addresses=lambda e:{address for line in e['line_mappings'] for address in line['addresses']}
            aa,bb=addresses(a[addr]),addresses(b[addr]);proof=proofs[hex(addr)]
            call_lines={line for site in proof['changes'] for line in range(site['before']['source_line_start'],site['before']['source_line_end']+1)}
            call_addresses={address for line in a[addr]['line_mappings'] if line['line_number'] in call_lines for address in line['addresses']}
            assert aa-bb <= call_addresses,('lost address outside replaced call',name,hex(addr))
            assert bb-aa == set(),('new global instruction mapping',name,hex(addr))
            argument_names={identifier for site in proof['changes'] for arg in site['after']['arguments'] for identifier in re.findall(r'\b[A-Za-z_][A-Za-z_0-9]*\b',arg['pointer'])}
            variable_mappings=[]
            for x,y in zip(a[addr]['variables'],b[addr]['variables']):
                if x['addresses']==y['addresses']:continue
                added=set(y['addresses'])-set(x['addresses']);lost=set(x['addresses'])-set(y['addresses'])
                assert x['name'] in argument_names,(name,hex(addr),x['name'],argument_names)
                assert added <= call_addresses and lost <= call_addresses,(name,hex(addr),added,lost,call_addresses)
                variable_mappings.append({'name':x['name'],'added_call_addresses':[hex(v) for v in sorted(added)],'lost_call_addresses':[hex(v) for v in sorted(lost)]})
            for site in proof['changes']:
                args=site['before']['arguments'];literal=site['after']['literal']
                try:tag=int(args[1],0)
                except ValueError:tag=None
                try:pointer=int(args[0],0)
                except ValueError:
                    evidence=out/f'{name}-{addr:x}-call-constants.jsonl'
                    if not evidence.exists():
                        evidence.write_bytes(subprocess.check_output(['/tmp/kuna-rust-panic-d850/call-constants',str(binary),hex(addr)]))
                    calls=[json.loads(line) for line in evidence.read_text().splitlines()]
                    candidates=[call for call in calls if call[2]==tag and call[3]==int(args[2],0)]
                    assert len(candidates)==1,(name,hex(addr),args,calls)
                    pointer=candidates[0][1]
                    site['symbolic_template_pointer']={'spelling':args[0],'address':hex(pointer),'call_instruction':hex(candidates[0][0]),'binding':'unique IR call with matching descriptor and caller-location operands'}
                if tag is not None and tag&1:
                    text=image_read(data,pointer,tag>>1).decode('utf8') if tag>>1 else ''
                    assert literal==text.replace('{','{{').replace('}','}}'), (name,hex(addr),text,literal)
                    assert not site['after']['arguments']
                    site['message_bytes_independently_verified']=True
                else:
                    assert site['after']['arguments'],(name,hex(addr))
                    assert image_read(data,pointer,1)
                    site['message_bytes_independently_verified']=False
            diff=list(difflib.unified_diff(a[addr]['code'].splitlines(True),b[addr]['code'].splitlines(True),fromfile='before/'+a[addr]['name'],tofile='after/'+b[addr]['name']))
            changes.append({'address':hex(addr),'name':b[addr]['name'],'proof':proof,'source_mapping':{'added_instruction_addresses':[hex(x) for x in sorted(bb-aa)],'lost_instruction_addresses':[hex(x) for x in sorted(aa-bb)],'lost_addresses_belong_to_replaced_call_lines':True,'variable_mapping_changes':variable_mappings,'declarations_and_function_metadata_unchanged':True},'hunks':[{'range':line.strip(),'classification':'only verified panic runtime calls become panic macros; every other AST node and comment unchanged'} for line in diff if line.startswith('@@')]})
            patch+=diff+['\n']
        parsed=0
        if language=='go':
            p=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True,text=True);assert p.returncode==0,p.stdout+p.stderr;parsed=len(b)
        (out/f'{name}-{language}.diff').write_text(''.join(patch))
        result['languages'].append({'language':language,'functions':len(b),'changed':changes,'parsed':parsed,'errors':0,'off_equals_parent':language=='rust'})
    (out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n')
    print(name,[(x['language'],x['functions'],len(x['changed'])) for x in result['languages']],flush=True)
    return result

results=[]
with ThreadPoolExecutor(max_workers=4) as pool:
    for future in as_completed([pool.submit(run,row) for row in inputs]): results.append(future.result())
assert patched_sha==hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest()
(out/'record.json').write_text(json.dumps({'patched_kuna_sha256':patched_sha,'baseline_commit':sha,'options':{'godwarfsig':'on'},'fixtures':results},indent=2)+'\n')
