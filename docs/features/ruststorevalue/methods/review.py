from common import *
import collections
out=w/'breadth';data=json.loads((out/'record.json').read_text());audit=json.loads((out/'assignment-review-v2.json').read_text());build=json.loads((w/'build.json').read_text());native=json.loads((w/'native-record.json').read_text())
key=lambda row:(row['fixture'],row['address'],row['gopairreturns'])
all_keys=set()
for row in data['records']:
 for change in row['changes']:
  identity=(row['fixture'],change['address'],change['gopairreturns']);assert identity not in all_keys;all_keys.add(identity)
  assert change['language']=='rust' and set(change['fields'])<={'code','line_mappings'}
  for arm in ['parent','candidate']:assert '__audit_' not in change[arm]['code']
classified={key(row):row for row in audit['records']};pending={key(row):row for row in audit['pending']};assert len(classified)==audit['verified']and len(pending)==audit['pending_count'];assert not classified.keys()&pending.keys();assert all_keys==classified.keys()|pending.keys()
not_strict=[row for row in audit['records']if row['whole_assignment_rhs_conversions']is None]
parse_states=collections.Counter((bool(row['original_parse_errors']['parent']),bool(row['original_parse_errors']['candidate']))for row in audit['records']+audit['pending'])
assert parse_states[(False,True)]==0,'New Rust parse error'
assert native['status']=='PASS'and native['complete_bodies']==33 and native['native_function_comparisons']==21384 and not native['body_rewrites']
assert native['candidate_cli_sha256']==build['new_cli_sha256']==data['candidate_cli_sha256']
assert native['parent_cli_sha256']==build['parent_cli_sha256']==data['parent_cli_sha256']
summary={'complete':not pending and not not_strict,'scope':audit['strict_assignment_scope'],'images':data['images'],'distinct_functions':data['distinct_functions'],'language_function_records':data['language_function_records'],'changed_rust_records':len(all_keys),'strict_assignment_records':len(classified)-len(not_strict),'pending_records':len(pending)+len(not_strict),'c_go_records_unchanged':True,'native_complete_bodies':33,'native_function_comparisons':21384,'new_decompiler_errors':0,'new_rust_parse_errors':0,'existing_rust_parse_failures':parse_states[(True,True)],'rust_parse_failures_resolved':parse_states[(True,False)],'identifier_normalized_records':sum(bool(row.get('strict_assignment_identifier_normalization'))for row in audit['records']),'ast_report_sha256':hashlib.sha256((out/'assignment-review-v2.json').read_bytes()).hexdigest(),'audit_source_sha256':hashlib.sha256((w/'ast-audit/src/main.rs').read_bytes()).hexdigest(),'audit_binary_sha256':hashlib.sha256((w/'ast-audit/target/release/kuna-rust-conversion-audit').read_bytes()).hexdigest(),'limitations':['Structural comparison does not establish native equivalence or type correctness for entire runtime images.','Original parser failures are retained; identifier-only normalization is used solely for structural comparison.','Native Rust uses overflow-checks=off and C uses-fwrapv for wrapping integer arithmetic.','Raw-pointer array indexing remains an inherited compiler error; actual STORE width repair, arbitrary inconsistent inferred types and unknown128-bit values are not established by this feature.'],'pending':list(pending.values())+not_strict}
(out/'structural-review.json').write_text(json.dumps(summary,indent=2)+'\n');print('Rust STORE review',len(all_keys),'changes;',summary['pending_records'],'pending',flush=True);assert summary['complete'],'Inspect non-strict changed records before speed or publication'
