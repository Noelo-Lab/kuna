"""Replay high-slice table proofs from archived Ghidra bytes, without executing an image."""
from pathlib import Path
import base64,capstone,datetime,hashlib,json,struct
w=Path(__file__).resolve().parent
read=lambda name:json.loads((w/name).read_text())
sha=lambda name:hashlib.sha256((w/name).read_bytes()).hexdigest()
old=read('static-table-review.json');decoder=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);records=[];inputs={}
for row in old['records']:
 prefix=row['binary']+'-'+row['table'][2:];table_name=prefix+'-table.json';cases_name=prefix+'-cases.json';asm_name=row['binary']+'-'+row['entry'][2:]+'-asm.json'
 table=read(table_name);cases=read(cases_name);asm=read(asm_name)['text'];table_bytes=base64.b64decode(table['bytes_b64']);case_bytes=base64.b64decode(cases['bytes_b64']);assert table_bytes.hex()==table['hex']and case_bytes.hex()==cases['hex'];assert len(table_bytes)==164;join=int(row['join'],16);base=int(row['table'],16);assert table['addr']==base
 for line in row['guard']+[row['default']]:assert line in asm,(asm_name,line)
 assert any('\tSHR 'in x and x.endswith(',0x20')for x in row['guard'])and any('\tCMP 'in x and x.endswith(',0x28')for x in row['guard'])and any('\tJA 'in x for x in row['guard'])and any('\tXOR 'in x for x in row['guard'])
 targets=[base+offset for offset in struct.unpack('<41i',table_bytes)];assert targets[0]==join;verified=[]
 for index,target in enumerate(targets[1:],1):
  offset=target-cases['addr'];assert 0<=offset<len(case_bytes);instructions=list(decoder.disasm(case_bytes[offset:offset+15],target));move=instructions[0];assert move.mnemonic=='mov';register,value=move.op_str.split(', ');assert register in ['al','cl']and int(value,0)==index
  if move.address+move.size==join:used=instructions[:1]
  else:
   jump=instructions[1];assert jump.address==move.address+move.size and jump.mnemonic=='jmp'and int(jump.op_str,0)==join;used=instructions[:2]
  result={'index':index,'target':hex(target),'instructions':[[hex(i.address),i.mnemonic,i.op_str]for i in instructions[:2]]};assert result==row['cases'][index-1],(row['table'],result,row['cases'][index-1]);verified.append(result)
 low_register=verified[0]['instructions'][0][2].split(', ')[0]
 assert all(case['instructions'][0][2].split(', ')[0]==low_register for case in verified)
 zero_register={'al':'EAX','cl':'ECX'}[low_register]
 assert any('\tXOR '+zero_register+','+zero_register in line for line in row['guard'])
 assert '\tMOV '+low_register.upper()+',0x29' in row['default']
 inputs.update({name:sha(name)for name in [table_name,cases_name,asm_name]});records.append({'entry':row['entry'],'table':row['table'],'rows':len(targets),'zero_reaches_join':True,'nonzero_set_index_then_join':len(verified)})
result={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'supplied_binary_executed':False,'capstone':capstone.__version__,'tables':len(records),'rows':sum(x['rows']for x in records),'records':records,'inputs':inputs,'original_review_sha256':sha('static-table-review.json')};assert result['tables']==6 and result['rows']==246;(w/'static-replay.json').write_text(json.dumps(result,indent=2)+'\n');print('All 246 table rows verified from archived bytes; no executable run.')
