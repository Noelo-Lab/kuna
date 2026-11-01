from pathlib import Path
import subprocess,os,json,hashlib,re
w=Path('/tmp/kuna-rust-panic');r=Path('/home/mahaloz/kwt/angr-rust-panic');base=Path('/home/mahaloz/kwt/angr-rust-signed-bytes')
versions=['1.90.0','1.93.1','1.99.1']
names=['panic_literal','panic_empty','panic_decimal','panic_signed','panic_hex','panic_reorder','panic_narrow','panic_escaped']
prefix=(r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_panic.rs').read_text().split('fn main()')[0]
loop='for x in (0u64..256).chain([u64::MAX, u64::MAX-1, 1<<63, (1<<63)-1, 0xffff_ffff, 0xffff, 0x123456789abcdef0]) { let x=black_box(x);'
original='''fn main() { std::panic::set_hook(Box::new(|_| {})); capture(|| panic_literal()); capture(|| panic_empty());
'''+loop+'''
capture(|| panic_decimal(x)); capture(|| panic_signed(x as i64)); capture(|| panic_hex(x as u32));
capture(|| panic_reorder(x as u32, (x.wrapping_mul(37) ^ (x >> 16)) as u16));
capture(|| panic_narrow(x as i8)); capture(|| panic_escaped(x as u8));
}
if black_box(false) {capture(|| panic_dynamic(black_box(7),black_box(8)));capture(|| panic_generic(black_box("text")));capture(|| panic_float(black_box(1.25)));capture(|| panic_forwarded(black_box(7)));}
}
'''
source=w/'native-oracle.rs';source.write_text(prefix+original)
def compiler(version):return ['rustc']+([] if version=='1.90.0' else ['+'+version])
def build(version,source,binary):
    p=subprocess.run(['taskset','-c','0-78']+compiler(version)+['--edition=2021','--crate-name','panic_oracle','-C','opt-level=2','-C','debuginfo=1','-C','strip=debuginfo',str(source),'-o',str(binary)],capture_output=True,text=True)
    binary.with_suffix('.compile.log').write_text(p.stdout+p.stderr)
    assert p.returncode==0,p.stderr
native=w/'native-exhaustive-1931';build('1.93.1',source,native)
oracle=subprocess.run([str(native)],capture_output=True,check=True);assert len(oracle.stdout.splitlines())==1580 and not oracle.stderr
(w/'native-payloads.txt').write_bytes(oracle.stdout)
bodies={};controls=[]
for side,tree in [('before',base),('after',r)]:
    bodies[side]={}
    for name in names+['panic_dynamic','panic_generic','panic_float','panic_forwarded']:
        p=subprocess.run(['taskset','-c','0-78',str(tree/'decompiler/target/release/kuna'),'decompile',str(native),name,'--language','rust'],env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True)
        bodies[side][name]=p.stdout;(w/f'{name}-{side}.rs').write_text(p.stdout)
    assert all('panic_fmt(' in bodies[side][name] for name in names) if side=='before' else all('panic!(' in bodies[side][name] for name in names)
for name in ['panic_generic','panic_float','panic_forwarded']:
    assert bodies['before'][name]==bodies['after'][name];controls.append(name)
support=prefix[prefix.index('fn capture('):]
code='use std::hint::black_box;\n'+support+bodies['after']['panic_literal']+bodies['after']['panic_empty']
code+='fn main() {std::panic::set_hook(Box::new(|_| {}));capture(|| unsafe {panic_literal()});capture(|| unsafe {panic_empty()});\n'+loop+'\n'
for name in names[2:]:
    body=bodies['after'][name];start=body.index('panic!(');expression=body[start:body.index(');',start)+2]
    pointers=dict(re.findall(r'\(\((v[0-9]+) as \*const _\) as \*const ([ui](?:8|16|32|64))\)',expression));assert pointers
    code+='capture(|| unsafe {\n'
    for var,ty in pointers.items():
        value='(x.wrapping_mul(37) ^ (x >> 16))' if name=='panic_reorder' and ty=='u16' else 'x'
        code+=f'let mut {var}_value={value} as {ty};let {var}=std::ptr::addr_of_mut!({var}_value);\n'
    code+=expression+'\n});\n'
code+='} }\n';recovered=w/'recovered-execution.rs';recovered.write_text(code)
records=[]
for version in versions:
    executable=w/f'recovered-{version}';build(version,recovered,executable)
    result=subprocess.run([str(executable)],capture_output=True,check=True)
    assert result.stdout==oracle.stdout and result.stderr==oracle.stderr,version
    records.append({'executor':version,'panic_payloads':1580,'literal_bodies_unchanged':True,'primitive_macro_expressions_unchanged':True,'explicit_primitive_input_bindings':True,'full_primitive_body_compilation_claimed':False,'original_location_equivalence_claimed':False})
    print(version,'1580 exact payloads',flush=True)
for version in ['1.90.0','1.99.1']:
    other=w/f'native-control-{version}';build(version,source,other)
    for name in ['panic_literal','panic_decimal','panic_hex']:
        outputs=[]
        for tree in [base,r]:
            outputs.append(subprocess.check_output(['taskset','-c','0-78',str(tree/'decompiler/target/release/kuna'),'decompile',str(other),name,'--language','rust'],env=os.environ|{'KUNA_SPECS':str(r/'specs')}))
        assert outputs[0]==outputs[1] and b'panic!(' not in outputs[1]
        controls.append(version+':'+name)
(w/'native-execution.json').write_text(json.dumps({'producer':'1.93.1','input_sha256':hashlib.sha256(native.read_bytes()).hexdigest(),'functions':names,'inputs_per_primitive':263,'six_integer_functions':True,'all_byte_patterns_and_wide_boundaries':True,'executions':records,'refusal_controls_unchanged':controls,'payload_sha256':hashlib.sha256(oracle.stdout).hexdigest()},indent=2)+'\n')
