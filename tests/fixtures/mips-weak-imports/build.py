#!/usr/bin/env python3
"""Rebuild original weak-import identity objects; explicit GAS ABI/endian inputs."""
import argparse, hashlib, json, subprocess
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--assembler',required=True);a=p.parse_args()
root=Path(__file__).resolve().parent
records=[]
for abi in ('n32','64'):
 for endian in ('be','le'):
  for mixed in (0,1):
   name=f"identity-{abi}-{endian}-{'mixed' if mixed else 'got-only'}.o"
   argv=[a.assembler,'-EB' if endian=='be' else '-EL','-march=octeon','-mabi='+abi,'--defsym','ABI64='+str(int(abi=='64')),'--defsym','MIXED='+str(mixed),'-o',name,'identity.S']
   subprocess.run(argv,cwd=root,check=True)
   records.append({'command':[Path(a.assembler).name,*argv[1:]],'output':name,'sha256':hashlib.sha256((root/name).read_bytes()).hexdigest()})
  name=f"page-{abi}-{endian}.o"
  argv=[a.assembler,'-EB' if endian=='be' else '-EL','-march=octeon','-mabi='+abi,'--defsym','ABI64='+str(int(abi=='64')),'-o',name,'page.S']
  subprocess.run(argv,cwd=root,check=True)
  records.append({'command':[Path(a.assembler).name,*argv[1:]],'output':name,'sha256':hashlib.sha256((root/name).read_bytes()).hexdigest()})
manifest={'source':'identity.S','source_sha256':hashlib.sha256((root/'identity.S').read_bytes()).hexdigest(),'page_source':'page.S','page_source_sha256':hashlib.sha256((root/'page.S').read_bytes()).hexdigest(),'assembler_version':subprocess.check_output([a.assembler,'--version'],text=True).splitlines()[0],'assembler_sha256':hashlib.sha256(Path(a.assembler).read_bytes()).hexdigest(),'records':records}
(root/'provenance.json').write_text(json.dumps(manifest,indent=2)+'\n')
