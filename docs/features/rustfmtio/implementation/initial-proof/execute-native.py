from pathlib import Path
import hashlib
import json
import os
import re
import socket
import subprocess

w = Path('/tmp/kuna-rust-io-proposal')
r = Path('/home/mahaloz/kwt/angr-rust-io-proposal')
records = {x['name']: x for x in json.loads((w / 'native-after.json').read_text())['functions']}
primitive = re.compile(r'&\*\(\((\w+) as \*const _\) as \*const (u32|i64)\)')
strings = re.compile(r'from_raw_parts\((\w+) as \*const u8,(\w+) as usize\)')
functions = [
    ('write_unsigned', 'out:&mut &std::io::Stdout,value:u32'),
    ('write_signed', 'out:&mut &std::io::Stdout,value:i64'),
    ('write_flags', 'out:&mut &std::io::Stdout,value:u32'),
    ('write_literal', 'out:&mut &std::io::Stdout'),
    ('write_string', 'out:&mut &std::io::Stdout,text:&str'),
    ('write_pair', 'out:&mut &std::io::Stdout,value:u32,text:&str'),
]
recovered = ''
sites = []
for name, parameters in functions:
    body = records[name]['code']
    lines = [line.strip() for line in body.splitlines() if 'use std::io::Write as _;' in line]
    assert len(lines) == 1 and lines[0].startswith('return '), (name, body)
    expression = lines[0][len('return '):].split('; //', 1)[0].removesuffix(';')
    assert 'std::mem::transmute::<std::io::Error, *mut ()>' in expression
    writer_type = re.search(r'fn ' + re.escape(name) + r'\(mut a0: (.*?)(?:, mut a1:|\) ->)', body)[1]
    bindings = f'let a0=out as *mut &std::io::Stdout as {writer_type};'
    for pointer, ty in primitive.findall(expression):
        bindings += f'let {pointer}=&value as *const {ty};'
    for data, length in strings.findall(expression):
        bindings += f'let {data}=text.as_ptr() as u64;let {length}=text.len() as u64;'
    recovered += f'fn {name}({parameters})->std::io::Result<()>{{unsafe{{{bindings}let raw={expression};if raw==0{{Ok(())}}else{{Err(std::mem::transmute::<*mut (),std::io::Error>(raw as usize as *mut ()))}}}}}}\n'
    sites.append({'name': name, 'expression': expression, 'bindings': bindings, 'unchanged_emitted_expression': True})

texts = []
for text in ['', 'é🙂🦀', 'a\0b\n{}', '中', 'e\u0301'] + [chr(i) for i in range(128)] + [a+b for a in ['é', '🙂', '🦀', '中', '\0', '\n', '{', '}'] for b in ['a', 'é', '🙂', '\0', '\n']]:
    if text not in texts:
        texts.append(text)
texts = texts[:156]
assert len(texts) == 156
unsigned = list(range(256)) + [256, 257, 65535, 65536, 2**31-1, 2**31, 2**32-1]
signed = list(range(-128, 128)) + [-2**63, 2**63-1, -257, 256, -65537, 65536, 2**32, -2**32]
values = [0, 1, 2, 23, 255, 256, 65535, 2**31, 2**32-1]
main = r'''
fn observe(name:&str,result:std::io::Result<()>) {
    match result {
        Ok(())=>eprintln!("{} ok 0",name),
        Err(error)=>unsafe {
            let pointer=std::mem::transmute::<std::io::Error,*mut ()>(error);
            let bits=pointer as usize;
            let error=std::mem::transmute::<*mut (),std::io::Error>(pointer);
            eprintln!("{} err {} {:?} {:?} {}",name,bits,error.raw_os_error(),error.kind(),error);
        },
    }
}
fn main() {
    let texts:Vec<String>=vec![TEXTS];
    let stdout=std::io::stdout();let mut out=&stdout;
    for pass in 0..3 {
        if pass==1 {let _=std::io::Write::write_all(&mut out,b"prefix:\xc3\xa9\xf0\x9f\x99\x82\0");}
        for value in [UNSIGNED] {
            if pass==2 {let _=std::io::Write::flush(&mut out);}
            observe("unsigned",write_unsigned(&mut out,value));
            observe("flags",write_flags(&mut out,value));
        }
        for value in [SIGNED] {observe("signed",write_signed(&mut out,value));}
        for text in &texts {
            if pass==2 {let _=std::io::Write::flush(&mut out);}
            observe("string",write_string(&mut out,text));
            for value in [VALUES] {observe("pair",write_pair(&mut out,value,text));}
        }
        observe("literal",write_literal(&mut out));
    }
}
'''
main = main.replace('TEXTS', ','.join('String::from_utf8(vec![' + ','.join(map(str, t.encode())) + ']).unwrap()' for t in texts)).replace('UNSIGNED', ','.join(str(x)+'u32' for x in unsigned)).replace('SIGNED', ','.join(str(x)+'i64' for x in signed)).replace('VALUES', ','.join(str(x)+'u32' for x in values))
original = (r / 'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io.rs').read_text()
original = original[:original.index('fn main() {')]
(w / 'native-oracle.rs').write_text(original + main)
(w / 'recovered-execution.rs').write_text(recovered + main)


def build(source, version, name):
    binary = w / name
    p = subprocess.run(['rustc', '+'+version, '--edition=2021', '--crate-name', 'stdout_oracle', str(w / source), '-O', '-o', str(binary)], capture_output=True)
    (w / (name+'.compile.log')).write_bytes(p.stderr)
    assert p.returncode == 0, p.stderr.decode()
    return binary


def execute(binary, mode):
    if mode == 'pipe':
        return subprocess.run([str(binary)], capture_output=True, check=True)
    if mode == 'broken-pipe':
        reader, writer = socket.socketpair()
        reader.close()
        with writer:
            return subprocess.run([str(binary)], stdout=writer, stderr=subprocess.PIPE, check=True)
    assert mode == 'device-full'
    with open('/dev/full', 'wb', buffering=0) as output:
        return subprocess.run([str(binary)], stdout=output, stderr=subprocess.PIPE, check=True)


count = 3 * (len(unsigned)*2 + len(signed) + len(texts)*(len(values)+1) + 1)
assert count == 7053
native = build('native-oracle.rs', '1.93.1', 'native-io-exhaustive-1931')
expected = {}
for mode in ['pipe', 'broken-pipe', 'device-full']:
    result = execute(native, mode)
    assert len(result.stderr.splitlines()) == count
    if mode == 'pipe':
        assert b' err ' not in result.stderr
    else:
        assert b' err ' in result.stderr
    expected[mode] = result
    (w / ('native-io-'+mode+'.stdout')).write_bytes(result.stdout or b'')
    (w / ('native-io-'+mode+'.stderr')).write_bytes(result.stderr)
results = []
for version in ['1.90.0', '1.93.1', '1.99.1']:
    binary = build('recovered-execution.rs', version, 'recovered-io-'+version)
    for mode, reference in expected.items():
        actual = execute(binary, mode)
        assert actual.stdout == reference.stdout and actual.stderr == reference.stderr, (version, mode)
        results.append({'compiler': version, 'mode': mode, 'calls': count, 'stdout_sha256': hashlib.sha256(actual.stdout or b'').hexdigest(), 'stderr_sha256': hashlib.sha256(actual.stderr).hexdigest(), 'original_output_bytes_and_owned_result_words_match': True})
        print(version, mode, count, 'NATIVE IO COMPARISONS PASS', flush=True)
(w / 'native-execution.json').write_text(json.dumps({'native_producer': '1.93.1', 'texts': len(texts), 'u32_values': len(unsigned), 'i64_values': len(signed), 'mixed_u32_values': len(values), 'buffering_sequences': 3, 'stream_modes': 3, 'total_comparisons': count*len(results), 'sites': sites, 'results': results, 'explicit_typed_input_bindings': True, 'complete_function_recompilation_claimed': False, 'scope': 'Six recovered reference-cell fixture expressions. Empty calls remain explicit and are tested separately by exact on/off equality; their source is not recompiled into this execution sequence. Promoted-receiver and reference-image expressions require their separate review.'}, indent=2) + '\n')
