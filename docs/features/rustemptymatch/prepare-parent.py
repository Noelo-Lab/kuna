exec(open('/tmp/kuna-rust-empty-match-db2d/parent-common.py').read())
run(r,w,['cargo','build','--release','-p','kuna-cli'],'parent-build.log',r/'decompiler')
files=subprocess.check_output(['git','ls-files','decompiler','tests','specs','VERSION','Makefile','docs/spec','docs/options.md'],cwd=r,text=True).splitlines()
manifest={'commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip(),'files':{p:hashlib.sha256((r/p).read_bytes()).hexdigest()for p in files if(r/p).is_file()},'release_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest()}
(w/'parent-source-before-timing.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('New independent parent built',manifest['release_sha256'],flush=True)
