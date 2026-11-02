from pathlib import Path
import subprocess,json,re,collections,hashlib
w=Path('/tmp/kuna-rust-fmt-pieces-f874');ns={};s=(w/'breadth.py').read_text();exec(s[:s.index('\ndef run(row):')],ns)
inputs={name:path for name,path,_ in ns['inputs']};breadth=json.loads((w/'breadth/record.json').read_text())
sites=[];mappings=[];changed=[]
for fixture in breadth['fixtures']:
 for language in fixture['languages']:
  for function in language['changed']:
   changed.append((fixture['fixture'],function))
   for site in function['proof']['changes']:sites.append({'fixture':fixture['fixture'],'function':function['name'],'address':function['address'],**site})
   for address in function['source_mapping']['lost_instruction_addresses']:
    p=subprocess.run(['objdump','-d','-w',f'--start-address={address}',f'--stop-address={int(address,16)+16}',str(inputs[fixture['fixture']])],capture_output=True,text=True,check=True)
    line=next(line for line in p.stdout.splitlines()if re.match(r'\s*'+address[2:]+':',line));instruction=line.split('\t')[-1].strip()
    classification='stack descriptor address ceases to be a rendered runtime argument; all IR and setup remain'
    if fixture['fixture']=='pieces-x86_64':
     assert (function['name'],function['address'],address) in [('literal_shape','0x1000','0x1000'),('stderr_shape','0x1042','0x1042')]
     assert instruction=='sub    $0x38,%rsp',instruction
     q=subprocess.run(['objdump','-d','-w',f'--start-address={address}',f'--stop-address={int(address,16)+66}',str(inputs[fixture['fixture']])],capture_output=True,text=True,check=True)
     assert 'lea    (%rsp),%rdi' in q.stdout and 'call' in q.stdout
     before=json.loads((w/'breadth/pieces-x86_64-rust-before.json').read_text())
     fn=next(v for v in before['functions'] if v['address']==int(address,16))
     assert any(v['name']=='v1' and v['stack_offset']==-56 for v in fn['variables'])
     assert '(&mut v1);' in fn['code']
     classification='synthetic prologue stack adjustment mapped through the folded descriptor address; RDI receives RSP, all five stack stores remain'
    else:
     assert re.match(r'(lea\s+.*\(%r(?:sp|bp)\),%rdi|mov\s+%r(?:sp|bp),%rdi)$',instruction),(fixture['fixture'],address,line)
    mappings.append({'fixture':fixture['fixture'],'function':function['address'],'address':address,'instruction':instruction,'classification':classification})
def literal(text):
 result=''
 for c in text:result+={'"':'\\"','\\':'\\\\','\n':'\\n','\r':'\\r','\t':'\\t','\0':'\\0'}.get(c,'\\u{'+format(ord(c),'x')+'}'if ord(c)<32 or ord(c)==127 else c)
 return '"'+result+'"'
source='fn main(){\n';expected={'stdout':b'','stderr':b''}
for site in sites:
 a=site['after'];args=[];values=[];source+='{\n'
 for n,arg in enumerate(a['arguments']):
  ty=arg['primitive'];bits=int(ty[1:]);value=-(1<<(bits-1))+1 if ty[0]=='i'else(1<<bits)-1
  source+=f'let value{n}:{ty}={value};\n';args.append('&value'+str(n));values.append(value)
 name=('e'if a['stderr']else'')+'print'+('ln'if a['newline']else'')+'!'
 source+=name+'('+','.join([literal(a['literal'])]+args)+');}\n'
 text=a['literal'].format(*values)+('\n'if a['newline']else'');expected['stderr'if a['stderr']else'stdout']+=text.encode()
source+='}\n';(w/'all-macros-execution.rs').write_text(source)
for version in ['1.90.0','1.93.1','1.99.1']:
 binary=w/f'all-macros-{version}'
 p=subprocess.run(['taskset','-c','0-78','rustc','+'+version,'--edition=2021','--crate-name','all_pieces',str(w/'all-macros-execution.rs'),'-o',str(binary)],capture_output=True,text=True)
 (w/f'all-macros-{version}.compile.log').write_text(p.stderr);assert p.returncode==0,p.stderr
 actual=subprocess.run([str(binary)],capture_output=True,check=True);assert actual.stdout==expected['stdout']and actual.stderr==expected['stderr']
audit=json.loads((w/'type-audit.json').read_text());assert not audit['summary']['newly_diagnosed_functions'];assert not audit['summary']['previously_compiling_now_failing']
new=[]
for record in audit['records']:
 a=collections.Counter((d['code'],d['message'])for d in record['before']['diagnostics']);b=collections.Counter((d['code'],d['message'])for d in record['after']['diagnostics'])
 if b-a:new.append({'fixture':record['fixture'],'address':record['address'],'diagnostics':list((b-a).elements())})
assert not new,new
(w/'mapping-review.json').write_text(json.dumps(mappings,indent=2)+'\n')
(w/'all-macros.json').write_text(json.dumps({'changed_functions':len(changed),'recovered_calls':len(sites),'literal_calls':sum(not site['after']['arguments']for site in sites),'primitive_calls':sum(bool(site['after']['arguments'])for site in sites),'execution_compilers':['1.90.0','1.93.1','1.99.1'],'stream_sha256':{name:hashlib.sha256(value).hexdigest()for name,value in expected.items()},'all_piece_bytes_and_formatter_identities_verified':all(site['all_piece_bytes_and_formatter_identities_verified']for site in sites),'primitive_execution_uses_explicit_test_values':True,'type_diagnostics':audit['summary'],'new_diagnostic_multisets':new},indent=2)+'\n')
print('ALL CHANGED MACROS AND SOURCE MAPPINGS REVIEWED',len(changed),len(sites),len(mappings),flush=True)
