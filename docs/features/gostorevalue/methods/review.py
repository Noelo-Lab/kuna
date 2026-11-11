from common import *
import re,difflib,collections
out=w/'breadth';d=json.loads((out/'record.json').read_text());good=[];pending=[]
for row in d['records']:
 for change in row['changes']:
  old=change['parent'];new=change['candidate'];fields={k for k in set(old)|set(new)if old.get(k)!=new.get(k)};assert fields<={'code','line_mappings'}
  a=old['code'].splitlines(keepends=True);b=new['code'].splitlines(keepends=True);converted=[];unmatched=[]
  if len(a)!=len(b):unmatched.append('line count')
  else:
   for index,(before,after)in enumerate(zip(a,b)):
    if before==after:continue
    lhs,sep,rhs=before.partition(' = ')
    if not sep:unmatched.append({'line':index,'before':before,'after':after});continue
    for ty in ['int8','byte','int16','uint16','int32','uint32','int64','uint64']:
     expected=lhs+sep+ty+'('+rhs.removesuffix('\n')+')'+('\n'if rhs.endswith('\n')else'')
     if after==expected:converted.append({'line':index+1,'type':ty,'before':before,'after':after});break
    else:unmatched.append({'line':index+1,'before':before,'after':after})
  record={'fixture':row['fixture'],'name':new['name'],'address':hex(new['address']),'gopairreturns':change['gopairreturns'],'conversion_lines':converted,'unmatched':unmatched}
  (pending if unmatched else good).append(record)
result={'scope':'Full integration structural review with Go pairs off/on. Each accepted changed line is exactly one integer conversion around the old complete RHS, with byte-identical LHS and all other function text. Whole-runtime equivalence is not claimed.','complete':not pending,'images':d['images'],'changed_functions':len(good)+len(pending),'verified_functions':len(good),'pending_functions':len(pending),'records':good,'pending':pending};(out/'structural-review.json').write_text(json.dumps(result,indent=2)+'\n');print('Review',len(good),'complete',len(pending),'pending');print(json.dumps(pending,indent=2)[:12000])
