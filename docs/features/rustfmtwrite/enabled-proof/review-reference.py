from pathlib import Path
import json,subprocess,re
w=Path('/tmp/kuna-rust-fmt-write');r=Path('/home/mahaloz/kwt/angr-rust-fmt-write');binary=Path('/tmp/kuna-angr-reference/binaries/rust-fmtdemo-1931')
result=subprocess.run([str(w/'verifier/target/debug/rust-write-verifier'),str(w/'reference-write-before.json'),str(w/'reference-write-after.json')],capture_output=True,check=True)
proof=json.loads(result.stdout);assert list(proof)==['0x11230'];site=proof['0x11230']['changes'][0]
call=subprocess.check_output([str(w/'descriptor-evidence'),str(binary),'0x11230','1'],cwd=r)
(w/'reference-write-descriptor.jsonl').write_bytes(call)
exec((w/'evidence.py').read_text());body=json.loads((w/'reference-write-before.json').read_text())['functions'][0]
verify_site(site,body,[json.loads(line)for line in call.decode().splitlines()],Image(binary.read_bytes()),demangled_symbols(binary))
(w/'reference-write-verified.json').write_text(json.dumps(proof,indent=2)+'\n')
print('Pinned angr write_str call: complete AST and image/ABI proof PASS',flush=True)
