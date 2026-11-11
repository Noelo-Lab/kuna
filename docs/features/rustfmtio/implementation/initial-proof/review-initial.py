from pathlib import Path
import json,hashlib
w=Path('/tmp/kuna-rust-io-proposal');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal')
exec((w/'evidence.py').read_text())
rows=json.loads((w/'parent-descriptors-complete.json').read_text())['records']
fixtures={'native':r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_x86_64','promoted':r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_promoted_x86_64','reference':Path('/tmp/kuna-angr-reference/binaries/rust-fmtdemo-1931')}
all_proofs=[]
for label,binary in fixtures.items():
 before={x['address']:x for x in json.loads((w/f'{label}-before.json').read_text())['functions']}
 proofs=json.loads((w/f'{label}-ast.json').read_text());memory=Image(binary.read_bytes());symbols=demangled_symbols(binary)
 for address,proof in proofs.items():
  body=before[int(address,16)]
  calls=[row['call']for row in rows if row['fixture']==label and row['address']==body['address']]
  assert len(calls)==1
  for site in proof['changes']:verify_site(site,body,calls,memory,symbols)
  all_proofs.append({'fixture':label,'function':body['name'],'address':address,'proof':proof})
  print(label,body['name'],'CALL-LINKED IMAGE, RECEIVER AND RESULT PROOF PASS',flush=True)
assert len(all_proofs)==11
(w/'initial-image-proofs.json').write_text(json.dumps({'changed_functions':len(all_proofs),'records':all_proofs,'all_template_bytes_and_argument_bindings_verified':True,'all_exact_writer_profiles_verified':True,'scope':'Independent parent raw operands and separately decoded complete body profiles bind every initial recovered call. Full breadth and instruction mapping classification remain separate.'},indent=2)+'\n')
