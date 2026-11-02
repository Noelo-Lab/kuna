from pathlib import Path
import json,re,difflib
import capstone
w=Path('/tmp/kuna-rust-io-proposal');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal')
exec((w/'evidence.py').read_text())
fixtures={'native':r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_x86_64','promoted':r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_promoted_x86_64','reference':Path('/tmp/kuna-angr-reference/binaries/rust-fmtdemo-1931')}
records=[]
for label,binary in fixtures.items():
 a={x['address']:x for x in json.loads((w/f'{label}-before.json').read_text())['functions']};b={x['address']:x for x in json.loads((w/f'{label}-after.json').read_text())['functions']};proofs=json.loads((w/f'{label}-ast.json').read_text());memory=Image(binary.read_bytes())
 assert a.keys()==b.keys()
 for addr in a:
  x,y=a[addr],b[addr]
  if x==y:continue
  assert hex(addr)in proofs
  assert {k:v for k,v in x.items()if k not in ['code','line_mappings','variables']}=={k:v for k,v in y.items()if k not in ['code','line_mappings','variables']}
  normalize=lambda v:{k:value for k,value in v.items()if k not in ['line_numbers','addresses']}
  assert list(map(normalize,x['variables']))==list(map(normalize,y['variables']))
  lines={line for site in proofs[hex(addr)]['changes']for line in range(site['before']['source_line_start'],site['before']['source_line_end']+1)}
  call_addresses={address for row in x['line_mappings']if row['line_number']in lines for address in row['addresses']}
  collect=lambda record:{address for row in record['line_mappings']for address in row['addresses']}
  lost=collect(x)-collect(y);added=collect(y)-collect(x);assert not added and lost<=call_addresses
  instructions=[]
  for address in sorted(lost):
   decoder=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
   instruction=next(decoder.disasm(memory.read(address,15),address))
   assert instruction.mnemonic=='lea'and instruction.op_str.split(',')[0]in ['rdx','edx'],(label,x['name'],hex(address),instruction.mnemonic,instruction.op_str)
   instructions.append({'address':hex(address),'bytes':instruction.bytes.hex(),'mnemonic':instruction.mnemonic,'operands':instruction.op_str,'classification':'Descriptor-address LEA into RDX on the replaced call line; no memory read/write or flag effect.'})
  variable_changes=[]
  for va,vb in zip(x['variables'],y['variables']):
   if va==vb:continue
   assert set(va['addresses'])^set(vb['addresses'])<=call_addresses
   assert set(va['line_numbers'])^set(vb['line_numbers'])<=lines
   variable_changes.append({'name':va['name'],'added_addresses':sorted(set(vb['addresses'])-set(va['addresses'])),'lost_addresses':sorted(set(va['addresses'])-set(vb['addresses']))})
  diff=''.join(difflib.unified_diff(x['code'].splitlines(True),y['code'].splitlines(True),fromfile='before/'+x['name'],tofile='after/'+y['name']))
  (w/f'{label}-{addr:x}.diff').write_text(diff)
  records.append({'fixture':label,'function':x['name'],'address':hex(addr),'function_and_variable_metadata_unchanged':True,'call_lines':sorted(lines),'lost_instruction_mappings':instructions,'variable_mapping_changes':variable_changes,'hunks':sum(line.startswith('@@')for line in diff.splitlines())})
  print(label,x['name'],'METADATA AND MAPPINGS VERIFIED',len(instructions),'descriptor LEAs',flush=True)
assert len(records)==11
(w/'mapping-review.json').write_text(json.dumps({'records':records,'changed_functions':len(records),'lost_instruction_mappings':sum(len(r['lost_instruction_mappings'])for r in records),'hunks':sum(r['hunks']for r in records),'all_metadata_and_mapping_changes_classified':True},indent=2)+'\n')
