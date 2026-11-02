from common import *
import datetime
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text())
records=[]
for arm,root in [('parent',Path('/home/mahaloz/kwt/angr-base-c5fc')),('candidate',r)]:
 cli=root/'decompiler/target/release/kuna'
 cmd=[str(cli),'decompile',str(w/'public-probe'),'check','--language','c','--option','sourcelang','rust','--json']
 if arm=='candidate':cmd+=['--option','rustprobestack','auto']
 p=subprocess.run(cmd,cwd=r,env=env|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True)
 (w/f'public-off-{arm}.json').write_text(p.stdout);(w/f'public-off-{arm}.log').write_text(p.stderr)
 body=json.loads(p.stdout)['functions'][0]['code']
 if arm=='parent':assert 'helper();'in body and 'return v1;'in body
 else:assert 'helper'not in body and 'return 8;'in body
 records.append({'arm':arm,'command':cmd,'cli_sha256':sha(cli),'body':body})
assert records[0]['cli_sha256']==build['parent_cli_sha256'] and records[1]['cli_sha256']==build['new_cli_sha256']
(w/'public-proof-off-final.json').write_text(json.dumps({'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'records':records,'source_sha256':sha(w/'public-probe.S'),'binary_sha256':sha(w/'public-probe'),'binary_executed':False},indent=2)+'\n')
print('Final public helper reproduction PASS')
