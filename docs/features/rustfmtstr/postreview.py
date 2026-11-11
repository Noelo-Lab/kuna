from pathlib import Path
import subprocess,json,re,collections,hashlib
w=Path('/tmp/kuna-rust-fmt-str');ns={};s=(w/'breadth.py').read_text();exec(s[:s.index('\ndef run(row):')],ns)
inputs={name:path for name,path,_ in ns['inputs']};breadth=json.loads((w/'breadth/record.json').read_text())
sites=[];mappings=[];changed=[]
for fixture in breadth['fixtures']:
 for language in fixture['languages']:
  for function in language['changed']:
   changed.append((fixture['fixture'],function))
   for site in function['proof']['changes']:
    assert site['all_template_bytes_and_argument_bindings_verified']
    sites.append({'fixture':fixture['fixture'],'function':function['name'],'address':function['address'],**site})
   for address in function['source_mapping']['lost_instruction_addresses']:
    p=subprocess.run(['objdump','-d','-w',f'--start-address={address}',f'--stop-address={int(address,16)+16}',str(inputs[fixture['fixture']])],capture_output=True,text=True,check=True)
    line=next(line for line in p.stdout.splitlines()if re.match(r'\s*'+address[2:]+':',line));instruction=line.split('\t')[-1].strip()
    mappings.append({'fixture':fixture['fixture'],'function':function['address'],'address':address,'instruction':instruction,'review_pending':True})
(w/'mapping-review-pending.json').write_text(json.dumps(mappings,indent=2)+'\n')
def literal(text):
 result=''
 for c in text:result+={'"':'\\"','\\':'\\\\','\n':'\\n','\r':'\\r','\t':'\\t','\0':'\\0'}.get(c,'\\u{'+format(ord(c),'x')+'}'if ord(c)<32 or ord(c)==127 else c)
 return '"'+result+'"'
source='fn main(){std::panic::set_hook(Box::new(|_|{}));\n'
for i,site in enumerate(sites):
 after=site['after'];bindings={};source+='{let text="é🦀\\0{}\\n";\n'
 def bind(name,code):
  assert re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*',name),name
  if name in bindings:assert bindings[name]==code,(name,code,bindings[name])
  bindings[name]=code
 for j,arg in enumerate(after['parameters']):
  if arg['kind']=='string':
   bind(arg['data'],f'let {arg["data"]}=text.as_ptr();')
   bind(arg['length'],f'let {arg["length"]}=text.len() as u64;')
  else:
   name=arg['pointer'];ty=arg['primitive'];value=ty+'::MIN+1'if ty[0]=='i'else ty+'::MAX'
   bind(name,f'let {name}_value:{ty}={value};let {name}=&{name}_value as *const {ty};')
 source+='\n'.join(bindings.values())+'\n'
 if after['sink']=='format':
  match=re.search(r'\b(a[0-9]+)\b',after['destination']);assert match,after
  source+='let mut destination=std::mem::MaybeUninit::<String>::uninit();let '+match[1]+'=destination.as_mut_ptr();\n'
  source+='unsafe{'+after['raw']+';println!("{:?}",destination.assume_init());}\n'
 elif after['sink']=='panic':
  source+='let error=std::panic::catch_unwind(||unsafe{'+after['raw']+'}).unwrap_err();println!("{:?}",error.downcast_ref::<String>().unwrap());\n'
 else:source+='unsafe{'+after['raw']+';}\n'
 source+='}\n'
source+='}\n';(w/'all-macros-execution.rs').write_text(source)
outputs=[]
for version in ['1.90.0','1.93.1','1.99.1']:
 binary=w/f'all-macros-{version}'
 p=subprocess.run(['taskset','-c','0-78','rustc','+'+version,'--edition=2021','--crate-name','all_strings',str(w/'all-macros-execution.rs'),'-o',str(binary)],capture_output=True,text=True)
 (w/f'all-macros-{version}.compile.log').write_text(p.stderr);assert p.returncode==0,p.stderr
 actual=subprocess.run([str(binary)],capture_output=True,check=True);outputs.append((actual.stdout,actual.stderr))
assert outputs[0]==outputs[1]==outputs[2]
audit=json.loads((w/'type-audit.json').read_text());assert not audit['summary']['newly_diagnosed_functions'];assert not audit['summary']['previously_compiling_now_failing']
new=[]
for record in audit['records']:
 a=collections.Counter((d['code'],d['message'])for d in record['before']['diagnostics']);b=collections.Counter((d['code'],d['message'])for d in record['after']['diagnostics'])
 if b-a:new.append({'fixture':record['fixture'],'address':record['address'],'diagnostics':list((b-a).elements())})
assert not new,new
(w/'all-macros.json').write_text(json.dumps({'changed_functions':len(changed),'recovered_calls':len(sites),'string_arguments':sum(sum(a['kind']=='string'for a in site['after']['parameters'])for site in sites),'execution_compilers':['1.90.0','1.93.1','1.99.1'],'stream_sha256':{'stdout':hashlib.sha256(outputs[0][0]).hexdigest(),'stderr':hashlib.sha256(outputs[0][1]).hexdigest()},'every_macro_expression_unchanged':True,'execution_uses_explicit_input_bindings':True,'all_template_bytes_and_argument_bindings_verified':True,'type_diagnostics':audit['summary'],'new_diagnostic_multisets':new,'source_mapping_classification_pending':True},indent=2)+'\n')
print('ALL MACROS COMPILED AND EXECUTED; SOURCE MAPPING CLASSIFICATION STILL PENDING',len(changed),len(sites),len(mappings),flush=True)
