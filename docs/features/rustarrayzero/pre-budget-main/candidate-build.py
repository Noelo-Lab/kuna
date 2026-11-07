from common import *
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'candidate-release.log',cwd=r/'decompiler')
print('Rebased candidate release ready',hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest(),flush=True)
