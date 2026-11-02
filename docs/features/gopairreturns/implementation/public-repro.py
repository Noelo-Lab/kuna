from common import *
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();out=w/'public-repro';out.mkdir();source='''package main
import "fmt"
//go:noinline
func pair(a,b int64)(int64,int64){return a+b,a-b}
func main(){fmt.Println(pair(7,3))}
''';(out/'pair.go').write_text(source);go='/tmp/kuna-toolchains/go1.27.3/go/bin/go';run(r,out,[go,'build','-trimpath','-o',str(out/'pair'),str(out/'pair.go')],'build.log');expected=subprocess.check_output([str(out/'pair')]);assert expected==b'10 4\n';(out/'source.stdout').write_bytes(expected);records={}
for arm,root in [('parent',Path('/home/mahaloz/kwt/angr-integer-field-extract-main')),('candidate',r)]:
 cli=root/'decompiler/target/release/kuna';cmd=[str(cli),'decompile',str(out/'pair'),'main.pair','--language','go','--option','godwarfsig','on','--json']
 if arm=='candidate':cmd+=['--option','gopairreturns','on']
 p=subprocess.run(cmd,env=env|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/(arm+'.json')).write_text(p.stdout);(out/(arm+'.log')).write_text(p.stderr);d=json.loads(p.stdout);f=d['functions'][0]if'functions'in d else d;assert not f.get('error');(out/(arm+'.go.body')).write_text(f['code']);records[arm]={'cli_sha256':sha(cli),'command':cmd,'code':f['code'],'address':f['address']}
body=records['candidate']['code'];(out/'recovered.go').write_text('package main\nimport "fmt"\n'+body+'\nfunc main(){fmt.Println(pair(7,3))}\n');run(r,out,[go,'build','-o',str(out/'recovered'),str(out/'recovered.go')],'recovered-build.log');actual=subprocess.check_output([str(out/'recovered')]);assert actual==expected;(out/'recovered.stdout').write_bytes(actual);record={'source_sha256':sha(out/'pair.go'),'fixture_sha256':sha(out/'pair'),'toolchain':subprocess.check_output([go,'version'],text=True).strip(),'native_stdout':actual.decode(),'body_rewrites':False,'records':records};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
