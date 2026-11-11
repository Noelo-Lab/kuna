from common import *
import shutil,datetime
f=w/'native';dest=r/'decompiler/crates/kuna-analysis/tests/fixtures';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
record=json.loads((w/'native-record.json').read_text());assert record['status']=='PASS' and record['complete_bodies']==57 and record['candidate_cli_sha256']==sha(r/'decompiler/target/release/kuna')
for name in ['pointer_index.c','pointer_index.kuna','pointer_index_helpers.c','pointer_index_x86_64']:assert not(dest/name).exists();shutil.copy2(f/name,dest/name)
for ext in ['c','rs']:shutil.copy2(f/('harness.'+ext),dest/('pointer_index_harness.'+ext))
provenance={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'authored':True,'command':['cc','-O2','-g','-fno-inline','-fno-pie','-no-pie',str(f/'pointer_index.c'),str(f/'pointer_index_helpers.c'),'-o',str(f/'pointer_index_x86_64')],'compiler':subprocess.check_output(['cc','--version'],text=True).splitlines()[0],'files':{name:sha(dest/name)for name in ['pointer_index.c','pointer_index.kuna','pointer_index_helpers.c','pointer_index_x86_64','pointer_index_harness.c','pointer_index_harness.rs']},'cases':json.loads((f/'cases.json').read_text()),'scope':record['scope'],'complete_unmodified_bodies':True}
(dest/'pointer_index.build.txt').write_text(json.dumps(provenance,indent=2)+'\n')
p=r/'decompiler/crates/kuna-cli/tests/rust_store_value.rs';s=p.read_text().replace('complete_rust_integer_stores_match_native_bytes_and_calls','complete_rust_pointer_indices_match_native_memory_addresses_and_calls').replace('go_store_value','pointer_index').replace('rust_store_value_harness','pointer_index_harness').replace('rust_store_','rust_pointer_index_').replace(' && *name != "array_store"','').replace('names.len(), 33','names.len(), 57').replace('33 * 324','54 * 270 + 3 * 64')
(r/'decompiler/crates/kuna-cli/tests/rust_pointer_index.rs').write_text(s)
p=r/'decompiler/crates/kuna-cli/tests/integration.rs';p.write_text(p.read_text()+'\n#[path = "rust_pointer_index.rs"]\nmod rust_pointer_index;\n')
p=r/'decompiler/crates/kuna-base/src/xml.rs';s=p.read_text();assert s.count('assert_eq!(count, 686, "corpus file count drifted");')==1;p.write_text(s.replace('assert_eq!(count, 686, "corpus file count drifted");','assert_eq!(count, 687, "corpus file count drifted");'))
p=r/'docs/spec/09-emission.md';p.write_text(p.read_text()+'''

### Rust raw-pointer indices

`kuna_rustptradd.rs` renders a raw-pointer PTRADD as
`base.wrapping_byte_offset((index as isize).wrapping_mul(scale))`.
The byte scale comes from the instruction, so it does not depend on the
recovered pointee's Rust layout. Index conversion and multiplication retain
modular address arithmetic. Literal-dependent index expressions keep their
recovered integer width before the `isize` conversion. A value use adds one
dereference; an address use leaves a raw pointer without creating a reference.
The base and index each appear once, in their existing order. A different
output pointee receives an explicit pointer cast.

This path requires matching 32- or 64-bit raw-pointer and integer-index
widths, byte-addressed pointers and a constant scale. Array-valued sources,
constant bases, relative pointers and implied PTRSUB/COPY bases retain their
existing paths. These exclusions avoid treating an array value as a pointer
receiver. `nocastprinting` retains its previous diagnostic output. This
corrects raw-pointer index syntax; it does not infer array layouts or repair
unrelated pointer arithmetic and memory widths.
''')
print('Promoted 57 native bodies; stage XML still to write',flush=True)
