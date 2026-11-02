from common import *
import datetime
f=r/'decompiler/crates/kuna-analysis/tests/fixtures';out=w/'enabled-proof';out.mkdir(exist_ok=False)
build=json.loads((w/'build.json').read_text());cli=r/'decompiler/target/release/kuna';assert hashlib.sha256(cli.read_bytes()).hexdigest()==build['new_cli_sha256']
names=json.loads((w/'fixture-names.json').read_text())['positive'];inputs=[0,1,7,12345,2147483647,2147483648,4294967288,4294967295]
records=[]
for label,fixture,selected in [('values','rust_probestack',names),('buffer','rust_probestack_buffer',['probe_buffer'])]:
 cmd=[str(cli),'decompile-all',str(f/(fixture+'_x86_64')),'--language','c','--option','rustprobestack','auto','--json','--functions',','.join(selected),'--assert','@'+str(f/(fixture+'.kuna'))]
 p=subprocess.run(cmd,cwd=r,env=env|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True);assert p.returncode==0;(out/f'{label}.json').write_text(p.stdout);(out/f'{label}.log').write_text(p.stderr)
 doc=json.loads(p.stdout);fs={v['name']:v for v in doc['functions']};assert set(fs)==set(selected)and all(not v.get('error')for v in fs.values())
 bodies='\n'.join(fs[n]['code']for n in selected);assert '__rust_probestack'not in bodies
 if label=='buffer':assert '[4096]'in bodies;helper=(f/'rust_probestack_buffer.c').read_text()
 else:helper=''
 calls=','.join(n+'(x)'for n in selected);fmt=' '.join(['%u']*len(selected))
 source=out/(label+'.c');source.write_text('#include <stdint.h>\n#include <stdio.h>\n#include <stdlib.h>\n'+helper+'\n'+bodies+'\nint main(int argc,char **argv){unsigned int x=(unsigned int)strtoul(argv[1],0,10);printf("'+fmt+'\\n",'+calls+');}\n')
 source_checks=[]
 for x in inputs:
  p=subprocess.run([str(f/(fixture+'_x86_64')),str(x)],capture_output=True,text=True,check=True);source_checks.append({'input':x,'values':list(map(int,p.stdout.split()))[:len(selected)]})
 for compiler in ['gcc','clang']:
  for opt in ['0','2']:
   binary=out/f'{label}-{compiler}-O{opt}';cmd=[compiler,'-std=c11','-O'+opt]
   if label=='buffer':cmd+=['-fsanitize=address','-fno-omit-frame-pointer','-no-pie']
   cmd+=[str(source),'-o',str(binary)];p=subprocess.run(cmd,capture_output=True,text=True);(out/f'{binary.name}-compile.log').write_text(p.stdout+p.stderr);assert p.returncode==0
   checked=[]
   for row in source_checks:
    p=subprocess.run([str(binary),str(row['input'])],capture_output=True,text=True,env=env|{'ASAN_OPTIONS':'detect_leaks=0'});(out/f'{binary.name}-{row["input"]}.log').write_text(p.stdout+p.stderr);assert p.returncode==0;values=list(map(int,p.stdout.split()));assert values==row['values'];checked.append({'input':row['input'],'values':values})
   records.append({'fixture':fixture,'compiler':compiler,'optimization':opt,'command':cmd,'asan':label=='buffer','complete_emitted_bodies_unchanged':True,'helper_source_unchanged':True,'checks':checked})
# The pinned general test is C output; source Rust output has a separate literal limitation.
for language in ['c','rust']:
 cmd=[str(cli),'decompile','/tmp/kuna-angr-reference/general/probestack','0xb720','--json','--language',language,'--option','rustprobestack','auto']
 p=subprocess.run(cmd,cwd=r,env=env|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True);assert p.returncode==0;(out/f'upstream-{language}.json').write_text(p.stdout);(out/f'upstream-{language}.log').write_text(p.stderr);code=json.loads(p.stdout)['functions'][0]['code'];checks={'name':'linux_encryptor::files::create_note'in code,'literal':'Luna.ini.exe.dll.lnk'in code,'probe_stack_absent':'probe_stack'not in code,'register_placeholders_absent':'{reg 'not in code,'rust_probestack_absent':'__rust_probestack'not in code}
 if language=='c':assert all(checks.values())
 (out/f'upstream-{language}-assertions.json').write_text(json.dumps(checks,indent=2)+'\n')
assert hashlib.sha256(cli.read_bytes()).hexdigest()==build['new_cli_sha256']
(out/'record.json').write_text(json.dumps({'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cli_sha256':build['new_cli_sha256'],'explicit_option':'rustprobestack auto','native_values':sum(len(row['values'])for record in records for row in record['checks']),'complete_emitted_functions':12,'asan_buffer_values':32,'records':records,'supplied_upstream_binary_executed':False,'pinned_general_C_test_textual_assertions_passed':True,'full_angr_parity_claimed':False},indent=2)+'\n')
print('Final native384 and pinned general C witness PASS',flush=True)
