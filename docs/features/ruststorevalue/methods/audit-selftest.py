from common import *
cases=[
 ('whole_rhs','*p=a+b;','*p=(a+b)as i64;',True),
 ('old_cast_retained','*p=a as u64;','*p=(a as u64)as i64;',True),
 ('context','*p=1<<a;','*p=::core::convert::identity::<u64>(1<<a)as i64;',True),
 ('negative_first','*p=-1+a;','*p=(-1+a)as i64;',True),
 ('keyword_field','(*p).fn=a+b;','(*p).fn=(a+b)as i64;',True),
 ('keyword_rhs','*p=(*q).fn;','*p=(*q).fn as i64;',True),
 ('partial_rhs','*p=a+b;','*p=a+(b as i64);',False),
 ('changed_lhs','*p=a;','*q=a as i64;',False),
 ('changed_call_order','*p=first()+second();','*p=(second()+first())as i64;',False),
 ('duplicated_call','*p=first();','*p=(first()+first())as i64;',False),
 ('old_cast_lost','*p=a as u32;','*p=a as i64;',False),
 ('changed_literal','*p=a+1;','*p=(a+2)as i64;',False),
 ('outside_assignment','return a;','return a as i64;',False),
]
rows=[]
for name,a,b,expected in cases:
 wrap=lambda text:'unsafe fn probe(p:*mut i64,q:*mut i64,a:i64,b:i64){'+text+'}'
 rows.append({'fixture':name,'changes':[{'address':'0x1','name':name,'gopairreturns':'off','fields':['code'],'parent':{'code':wrap(a)},'candidate':{'code':wrap(b)}}]})
(w/'audit-selftest-input.json').write_text(json.dumps({'records':rows},indent=2)+'\n')
run(r,w,[str(w/'ast-audit/target/release/kuna-rust-conversion-audit'),str(w/'audit-selftest-input.json'),str(w/'audit-selftest-output.json')],'audit-selftest.log')
d=json.loads((w/'audit-selftest-output.json').read_text());records={x['fixture']:x for x in d['records']+d['pending']}
assert set(records)=={name for name,*_ in cases}
for name,_,_,expected in cases:assert (records[name]['whole_assignment_rhs_conversions'] is not None)==expected,(name,records[name])
print('Strict assignment audit self-checks passed:',len(cases))
