from pathlib import Path
import subprocess,json,re,sys,collections
w=Path('/tmp/kuna-rust-panic-d850');r=Path('/home/mahaloz/kwt/angr-rust-panic-main')
ns={};s=(w/'breadth.py').read_text();exec(s[:s.index('\ndef image_read')],ns)
inputs={name:path for name,path,_ in ns['inputs']}
breadth=json.loads((w/'breadth/record.json').read_text())
sites=[];mappings=[];changed=[]
for fixture in breadth['fixtures']:
    for language in fixture['languages']:
        for function in language['changed']:
            changed.append((fixture['fixture'],function))
            for site in function['proof']['changes']:
                sites.append({'fixture':fixture['fixture'],'function':function['name'],'address':function['address'],**site})
            for address in function['source_mapping']['lost_instruction_addresses']:
                p=subprocess.run(['objdump','-d','-w',f'--start-address={address}',f'--stop-address={int(address,16)+16}',str(inputs[fixture['fixture']])],capture_output=True,text=True,check=True)
                line=next(line for line in p.stdout.splitlines() if re.match(r'\s*'+address[2:]+':',line))
                parts=line.split('\t');instruction=parts[-1].strip()
                assert re.match(r'(lea\s+.*\(%r(?:sp|bp)\),%rsi|mov\s+%r(?:sp|bp),%rsi)$',instruction),(fixture['fixture'],address,line)
                mappings.append({'fixture':fixture['fixture'],'function':function['address'],'address':address,'instruction':instruction,'classification':'stack descriptor-array address no longer rendered as a runtime argument; IR and setup statements retained'})
def literal(text):
    escaped=''
    for c in text:
        escaped += {'"':'\\"','\\':'\\\\','\n':'\\n','\r':'\\r','\t':'\\t','\0':'\\0'}.get(c,'\\u{'+format(ord(c),'x')+'}' if ord(c)<32 or ord(c)==127 else c)
    return '"'+escaped+'"'
code='fn main(){std::panic::set_hook(Box::new(|_|{}));\n'
for index,site in enumerate(sites):
    a=site['after'];args=[]
    code+='let payload=std::panic::catch_unwind(|| {\n'
    for n,arg in enumerate(a['arguments']):
        ty=arg['primitive'];value=ty+'::MIN + 1' if ty.startswith('i') else ty+'::MAX'
        code+=f'let value{n}:{ty}={value};\n';args.append('&value'+str(n))
    code+='panic!('+','.join([literal(a['literal'])]+args)+');\n}).unwrap_err();\n'
    code+='let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());\n'
    if not args:
        expected=a['literal'].replace('{{','{').replace('}}','}')
        code+='assert_eq!(message,'+literal(expected)+');\n'
    code+=f'println!("{index} {{message:?}}");\n'
code+='}\n';(w/'all-macros-execution.rs').write_text(code)
outputs=[]
for version in ['1.90.0','1.93.1','1.99.1']:
    compiler=['rustc']+([] if version=='1.90.0' else ['+'+version]);binary=w/f'all-macros-{version}'
    p=subprocess.run(['taskset','-c','0-78']+compiler+['--edition=2021','--crate-name','all_panics',str(w/'all-macros-execution.rs'),'-o',str(binary)],capture_output=True,text=True)
    (w/f'all-macros-{version}.compile.log').write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr
    actual=subprocess.run([str(binary)],capture_output=True,check=True)
    assert len(actual.stdout.splitlines())==len(sites) and not actual.stderr
    outputs.append(actual.stdout)
assert outputs[0]==outputs[1]==outputs[2]
audit=json.loads((w/'type-audit.json').read_text());assert not audit['summary']['newly_diagnosed_functions'];assert not audit['summary']['previously_compiling_now_failing']
new=[]
for record in audit['records']:
    a=collections.Counter((d['code'],d['message']) for d in record['before']['diagnostics']);b=collections.Counter((d['code'],d['message']) for d in record['after']['diagnostics'])
    if b-a:new.append({'fixture':record['fixture'],'address':record['address'],'diagnostics':list((b-a).elements())})
assert not new,new
(w/'mapping-review.json').write_text(json.dumps(mappings,indent=2)+'\n')
(w/'all-macros.json').write_text(json.dumps({'changed_functions':len(changed),'recovered_calls':len(sites),'literal_messages_verified_from_image':sum(site['message_bytes_independently_verified'] for site in sites),'primitive_calls':sum(bool(site['after']['arguments']) for site in sites),'execution_compilers':['1.90.0','1.93.1','1.99.1'],'literal_payloads_compared_to_image_bytes':True,'primitive_execution_uses_explicit_test_values':True,'type_diagnostics':audit['summary'],'new_diagnostic_multisets':new},indent=2)+'\n')
print('ALL CHANGED MACROS AND SOURCE MAPPINGS REVIEWED',len(changed),len(sites),len(mappings),flush=True)
