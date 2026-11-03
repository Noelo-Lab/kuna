from common import *
f=w/'native';old=json.loads((f/'parent-rust.json').read_text());new=json.loads((f/'candidate-rust.json').read_text());before={x['name']:x for x in old['functions']};changes=[{'name':x['name'],'language':'rust','gopairreturns':'off','address':hex(x['address']),'parent':before[x['name']],'candidate':x}for x in new['functions']if x!=before[x['name']]]
source=w/'native-review-input.json';source.write_text(json.dumps({'records':[{'fixture':'pointer-native','changes':changes}]})+'\n')
run(r,w,[str(w/'ast-audit/target/release/kuna-rust-pointer-audit'),str(source),str(w/'native-review.json')],'native-audit.log');result=json.loads((w/'native-review.json').read_text());print(result['changed_records'],'native changes;',result['pending'],'pending');assert result['pending']==0
