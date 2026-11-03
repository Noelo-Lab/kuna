from common import *
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(w/'public-repro.c').write_text('int indexed(int*p,long i){return p[i];}\nint main(void){return 0;}\n')
cmd=['cc','-O2','-g','-fno-pie','-no-pie',str(w/'public-repro.c'),'-o',str(w/'public-repro')];subprocess.run(cmd,capture_output=True,check=True);records=[]
for arm,root in [('parent',Path('/home/mahaloz/kwt/angr-base-0b7117')),('candidate',r)]:
 command=[str(root/'decompiler/target/release/kuna'),'decompile',str(w/'public-repro'),'indexed','--language','rust','--assert','prototype indexed int4 indexed(int4*p,int8 i)'];p=subprocess.run(command,capture_output=True,check=True,env=env|{'KUNA_SPECS':str(root/'specs')});(w/('public-repro-'+arm+'.rs')).write_bytes(p.stdout);records.append({'arm':arm,'command':command,'cli_sha256':sha(root/'decompiler/target/release/kuna'),'output_sha256':hashlib.sha256(p.stdout).hexdigest()})
 assert (b'return p[i];'in p.stdout)if arm=='parent'else(b'return *p.wrapping_byte_offset((i as isize).wrapping_mul(4));'in p.stdout)
(w/'public-repro.json').write_text(json.dumps({'fixture_command':cmd,'source_sha256':sha(w/'public-repro.c'),'fixture_sha256':sha(w/'public-repro'),'records':records},indent=2)+'\n');print('Public pointer-index repro verified')
