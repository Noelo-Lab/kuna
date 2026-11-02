from common import *
base=Path('/home/mahaloz/kwt/angr-base-83a6f');out=w/'native-go';out.mkdir(exist_ok=False);names=['make_pair','first','second','both','forward','narrow_second','scalar','invoke_void'];tools=['/tmp/kuna-toolchains/go1.22.5/go/bin/go','/tmp/kuna-toolchains/go1.27.3/go/bin/go'];header='package main\ntype Pair struct { first int64; second int64 }\nvar calls uint64\n';record={}
harness='''
func scalar_op(a,b int64) int64 { calls++; return a^b }
func void_op(a,b int64) { calls++ }
func main() {
 values:=[]int64{-9223372036854775808,-9223372036854775807,-4294967296,-65536,-17,-1,0,1,17,65536,4294967295,9223372036854775806,9223372036854775807}
 for _,a:=range values { for _,b:=range values {
  calls=0;x:=first(make_pair,a,b);if x!=a+b || calls!=1 {panic("first")}
  calls=0;y:=second(make_pair,a,b);if y!=a-b || calls!=1 {panic("second")}
  calls=0;z:=both(make_pair,a,b);if z!=(a+b)^(a-b) || calls!=1 {panic("both")}
  calls=0;p:=forward(make_pair,a,b);if p.first!=a+b || p.second!=a-b || calls!=1 {panic("forward")}
  calls=0;s:=scalar(scalar_op,a,b);if s!=a^b || calls!=1 {panic("scalar")}
  calls=0;invoke_void(void_op,a,b);if calls!=1 {panic("void")}
 } }
}
'''
for arm,root in [('parent',base),('candidate',r)]:
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(w/'callback'),'--language','go','--functions',','.join(names),'--assert','@'+str(w/'callback.kuna'),'--json'];p=subprocess.run(cmd,env=env|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/f'{arm}.json').write_text(p.stdout);(out/f'{arm}.log').write_text(p.stderr);d=json.loads(p.stdout);assert len(d['functions'])==8 and all(not f.get('error')for f in d['functions']);assert all(a['status']=='applied'for a in d['assertions']);fs={f['name']:f['code']for f in d['functions']};full=out/f'{arm}-all.go';full.write_text(header+'\n'.join(fs.values())+'\nfunc main(){}\n');record[arm]={}
 for tool in tools:
  version=subprocess.check_output([tool,'version'],text=True).strip();binary=out/(arm+'-all-'+Path(tool).parts[-4]);p=subprocess.run([tool,'build','-o',str(binary),str(full)],text=True,capture_output=True);(out/f'{arm}-{Path(tool).parts[-4]}-full-build.log').write_text(p.stdout+p.stderr);assert p.returncode!=0
  record[arm][version]={'all_eight_bodies_compile':False,'full_compile_exit':p.returncode}
  if arm=='candidate':
   assert 'cannot use fn(a, b).second'in p.stderr and 'as int32'in p.stderr,p.stderr
   partial=out/'candidate-seven.go';partial.write_text(header+'\n'.join(code for name,code in fs.items()if name!='narrow_second')+'\n'+harness);p=subprocess.run([tool,'run',str(partial)],text=True,capture_output=True);(out/f'candidate-{Path(tool).parts[-4]}-seven-run.log').write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr
   record[arm][version].update(seven_complete_bodies_pass=True,input_pairs=169,single_callback_evaluation=True)
(out/'record.json').write_text(json.dumps({'scope':'Seven unchanged emitted Go bodies verified. The narrowed second-field return still needs an explicit int32 conversion and is an emitter follow-up; it is excluded from the passing claim.','records':record},indent=2)+'\n');print('Go seven-body native proof PASS on two compilers; narrowed result emission remains a recorded compile failure.',flush=True)
