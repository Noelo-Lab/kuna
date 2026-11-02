from common import *
base=Path('/home/mahaloz/kwt/angr-base-c5fc')
run(base,w,['cargo','build','--release','-p','kuna-cli'],'parent-build-c5fc.log',base/'decompiler')
files=subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','decompiler','tests','specs','VERSION','Makefile','docs/spec','docs/options.md'],cwd=base,text=True).splitlines()
source={p:hashlib.sha256((base/p).read_bytes()).hexdigest()for p in files if(base/p).is_file()}
assert not subprocess.check_output(['git','status','--porcelain'],cwd=base)
(w/'parent-source-before-timing.json').write_text(json.dumps({'root':str(base),'commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip(),'files':source,'cli_sha256':hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest()},indent=2)+'\n')
print('Clean independent parent built and manifested',flush=True)
