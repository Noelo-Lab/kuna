from common import *
cases = [
 ('conversion', 'return a;', 'return a as u64;', True),
 ('context', 'return (1 << a);', 'return ::core::convert::identity::<i64>(1 << a) as u64;', True),
 ('grouping', 'return a + b - 9;', 'return a + ((b - 9) as u64);', True),
 ('local_assignment', 'a += b;', 'a = a + b as i64;', True),
 ('negative_constant', 'return a + -9;', 'return a - (9 as i64);', True),
 ('bad_keyword', 'let mut fn: i64; fn = a; return fn;', 'let mut fn: i64; fn = a as i64; return fn;', True),
 ('bad_namespace', 'return _::path(a);', 'return _::path(a) as u64;', True),
 ('call_order', 'return first() + second();', 'return second() + first();', False),
 ('duplicated_call', 'return first();', 'return first() + first();', False),
 ('literal_change', 'return a + 9;', 'return a + 8;', False),
 ('operator_change', 'return a + b;', 'return a - b;', False),
 ('precedence_change', 'return (a + b) * 9;', 'return a + b * 9;', False),
 ('pointer_repetition', '*pointer() += a;', '*pointer() = *pointer() + a;', False),
]
records=[]
for name,before,after,accepted in cases:
 wrap=lambda body:'unsafe fn probe(mut a: i64, b: i64) -> i64 { '+body+' }'
 records.append({'fixture':name,'changes':[{'address':'0x1','name':name,'gopairreturns':'off','fields':['code'],'parent':{'code':wrap(before)},'candidate':{'code':wrap(after)}}]})
(w/'audit-selftest-input.json').write_text(json.dumps({'records':records},indent=2)+'\n')
run(r,w,[str(w.parent/'ast-audit/target/release/kuna-rust-conversion-audit'),str(w/'audit-selftest-input.json'),str(w/'audit-selftest-output.json')],'audit-selftest.log')
d=json.loads((w/'audit-selftest-output.json').read_text())
assert {x['fixture'] for x in d['records']}=={name for name,_,_,accepted in cases if accepted}
assert {x['fixture'] for x in d['pending']}=={name for name,_,_,accepted in cases if not accepted}
print('Audit self-checks passed:',len(cases))
