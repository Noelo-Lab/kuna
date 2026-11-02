from pathlib import Path
import datetime,json,subprocess,hashlib
w=Path('/tmp/kuna-highslice-optin');r=Path('/home/mahaloz/kwt/angr-narrow-table-guard');out=w/'language-native';out.mkdir(exist_ok=False);names=json.loads((w/'fixture-names.json').read_text());breadth=json.loads((w/'breadth-on/highslice-native-record.json').read_text());expected=json.loads((w/'native-proof/record.json').read_text())['records'][0]['checks'];records=[];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for language in ['rust','go']:
 item=next(row for row in breadth['records']if row['language']==language);capture=Path(item['candidate_file']);assert sha(capture)==item['candidate_sha256'];fs={f['name']:f for f in json.loads(capture.read_text())['functions']};bodies='\n'.join(fs[n]['code']for n in names)
 if language=='rust':
  text=bodies+'\nfn main(){ let x: u64=std::env::args().nth(1).unwrap().parse().unwrap(); unsafe { println!("'+(' '.join(['{}']*len(names)))+'",'+','.join(n+'(x)'for n in names)+'); }}\n';source=out/'recovered.rs';source.write_text(text);commands=[('rustc-O'+opt,['rustc','--edition=2021','-C','opt-level='+opt,str(source),'-o',str(out/('rustc-O'+opt))])for opt in ['0','2']]
 else:
  text='package main\nimport ("fmt";"os";"strconv")\n'+bodies+'\nfunc main(){ x,err:=strconv.ParseUint(os.Args[1],10,64); if err!=nil { panic(err) }; fmt.Printf("'+' '.join(['%d']*len(names))+'\\n",'+','.join(n+'(x)'for n in names)+') }\n';source=out/'recovered.go';source.write_text(text);go='/tmp/kuna-toolchains/go1.27.3/go/bin/go';commands=[('go-default',[go,'build','-o',str(out/'go-default'),str(source)]),('go-unoptimized',[go,'build','-gcflags=all=-N -l','-o',str(out/'go-unoptimized'),str(source)])]
 for name,command in commands:
  p=subprocess.run(command,capture_output=True,text=True);(out/(name+'-build.log')).write_text(p.stdout+p.stderr);assert p.returncode==0,(language,name,p.stderr)
  for row in expected:
   p=subprocess.run([str(out/name),str(row['input'])],capture_output=True,text=True,check=True);assert list(map(int,p.stdout.split()))==row['values'],(language,name,row,p.stdout,p.stderr)
  records.append({'language':language,'build':name,'command':command,'source_sha256':sha(source),'capture_sha256':sha(capture),'function_bodies_unchanged':True,'source_prototypes_not_overridden':True,'checks':expected});print(language,name,'PASS',len(expected)*len(names),'values',flush=True)
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cli_sha256':json.loads((w/'build.json').read_text())['new_cli_sha256'],'native_values':sum(len(row['checks'])*len(names)for row in records),'complete_emitted_bodies_per_language':6,'all_function_bodies_unchanged':True,'supplied_upstream_executable_executed':False,'records':records};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n')
