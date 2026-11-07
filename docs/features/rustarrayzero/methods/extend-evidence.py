from common import *
p=w/'finalize.py';s=p.read_text();s=s.replace("'main-budget-integration-plan.json']", "'main-budget-integration-plan.json','angr-reference.json']")
marker="(out/'record.json').write_text(json.dumps(summary,indent=2)+'\\n')"
insert="""witness=next(row for row in speed['records']if row['case']=='enabled-rust-zero')
summary.update(opportunity='TestRustStoreWidth::zero_unsigned',test_name='ghangr-rust-array-zero',binary='decompiler/crates/kuna-analysis/tests/fixtures/array_constant_x86_64',selector='zero_unsigned',flag='rust_array_zero',change_kind='correctness-fix',source_decompiler='angr',inspiration='Wide-store parity audit exposed invalid scalar zero assigned to a recovered integer byte array; source-native memory is the oracle.',angr_version=read('angr-reference.json')['angr_version'],angr_commit=read('angr-reference.json')['angr_commit'],ablation_changed=0,ablation_scope='Initial default-on 675-assertion corpus; full evidence retained under failed-default-on/',parity='OK',demo_extra_options=[['outlang','rust'],['ctypes','off'],['realtypes','off']],speed_off_ms=witness['medians']['parent']['wall_ms'],speed_on_ms=witness['medians']['candidate']['wall_ms'],speed_delta_pct=witness['median_delta_pct']['wall_ms'],speed_samples=speed['plan']['samples_per_arm'],speed_budget_pct=speed['plan']['budget_pct'],speed_within_budget=witness['median_delta_pct']['wall_ms']<=speed['plan']['budget_pct'],speed_method='Fixed balanced paired final-binary on/off wall medians for zero_unsigned; full CPU/wall comparisons in timing-result.json. Initial failed default-on budget controls the off default.')
(out/'record.json').write_text(json.dumps(summary,indent=2)+'\\n')
(out/'plan.md').write_text('''# Rust zero byte-array stores

Preserve the recovered byte-array destination and its alignment. For a constant-zero STORE, require identical value/destination byte width, a positive array length, and one-byte integer elements of alignment one. Emit a contextually typed `[0; N]` only for that bounded case. Keep the existing STORE address, evaluation count and cast-suppression behavior.

Ship `rustarrayzero` off in every preset after the initial default-on timing failure. Compare complete native bodies with the source C at both optimization levels, including unaligned byte arrays, all guard bytes and pointer callback counts. Retain the wider six-function bundle with carry and alias cases. Compare fresh parent/off/on captures across the pinned corpus, then classify every enabled change using recursive AST comparison. Run the four repository checks and the fixed speed plan on the integrated main tree before publication.

Nonzero constants, arbitrary element types, scalar STORE-width recovery and source-wide Rust parity are separate work. `analysis.md` and `record.json` contain the measured outcome; failed and superseded evidence remains archived.
''')
reference=read('angr-reference.json');parent_functions={f['name']:f for f in read('parent-rust.json')['functions']};candidate_functions={f['name']:f for f in read('candidate-rust.json')['functions']}
side=['Pinned angr Rust-flavor capture of the authored C-source fixture, then complete Kuna parent/enabled output. These reference bodies are not claimed compilable or native-equivalent. The source C is the native oracle.']
for f in reference['functions']:
 name=f['name'];side.extend(['\\nFUNCTION '+name,'ANGR\\n'+f.get('code',f.get('error','')),'KUNA PARENT\\n'+parent_functions[name]['code'],'KUNA RUSTARRAYZERO ON\\n'+candidate_functions[name]['code']])
(out/'angr-vs-kuna.txt').write_text('\\n'.join(side)+'\\n')"""
assert marker in s;s=s.replace(marker,insert)
s=s.replace('Complete parent and candidate bodies, compiler diagnostics and native output are retained without body rewrites.', 'Complete parent and candidate bodies, compiler diagnostics and native output are retained without body rewrites. The pinned angr STORE-width tests motivated the audit. A fresh reference capture also retains all 13 functions from this authored C-source fixture using its Rust flavor; that output contains C-style member syntax and unresolved types, so it is not claimed compilable or native-equivalent. The source C is the semantic oracle, and `angr-vs-kuna.txt` retains the exact three-way text.')
p.write_text(s)
compile(s,str(p),'exec')
print('Extended concrete evidence plan and exact reference comparison')
