"""Compile authored data-only fixtures; never execute them."""
from pathlib import Path
from hashlib import sha256
import argparse, json, re, struct, subprocess, shutil
parser = argparse.ArgumentParser()
parser.add_argument('output', type=Path)
parser.add_argument('--rustc', default='rustc')
args = parser.parse_args()
D = args.output.resolve()
D.mkdir(parents=True, exist_ok=True)
version = subprocess.check_output([args.rustc, '--version', '--verbose'], text=True)
assert '\nrelease: 1.93.1\n' in version, version
source = '''use std::sync::atomic::{AtomicU64, Ordering};
static CALLS: AtomicU64 = AtomicU64::new(0);
#[no_mangle]
#[inline(never)]
pub fn r13_tick(value: u64) -> u64 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    value.wrapping_add(1)
}
#[no_mangle]
#[inline(never)]
pub fn r13_display_stderr(value: u64) {
    eprintln!("R13 display={} braces={{ok}}", r13_tick(value));
}
#[no_mangle]
#[inline(never)]
pub fn r13_display_stdout(value: u64) {
    println!("R13 display={} braces={{ok}}", r13_tick(value));
}
fn main() {
    r13_display_stderr(std::hint::black_box(41));
    r13_display_stdout(std::hint::black_box(97));
}
'''
(D / 'display.rs').write_text(source)
commands = []
flags = ['--edition=2021', '--crate-name', 'r13_display', '-C', 'lto=off', '-C', 'debuginfo=2', '-C', 'panic=abort', '-C', 'relocation-model=static', '-C', 'link-arg=-no-pie']

def compile(name, src, opt, extra=()):
    image = D / (name + '.elfdata')
    cmd = [args.rustc, *flags, '-C', 'opt-level=' + str(opt), *extra, str(src), '-o', str(image)]
    commands.append(cmd)
    p = subprocess.run(cmd, capture_output=True)
    (D / (name + '.compile.log')).write_bytes(p.stdout + p.stderr)
    assert p.returncode == 0, p.stderr
    image.chmod(292)
    return image

def symbols(image):
    return [line.split() for line in subprocess.check_output(['nm', '-S', '--defined-only', str(image)], text=True).splitlines() if len(line.split()) == 4]
seed = compile('seed', D / 'display.rs', 0)
rows = symbols(seed)

def unique(pattern):
    found = [row[3] for row in rows if re.search(pattern, row[3])]
    assert len(found) == 1, (pattern, found)
    return found[0]
ctor = unique('^_ZN4core3fmt9Arguments3new')
stderr = unique('^_ZN3std2io5stdio7_eprint')
stdout = unique('^_ZN3std2io5stdio6_print')
demangled = subprocess.check_output(['nm', '-S', '-C', '--defined-only', str(seed)], text=True).splitlines()
display = [line.split()[0] for line in demangled if line.endswith(' core::fmt::num::imp::<impl core::fmt::Display for u64>::fmt')]
assert len(display) == 1
fmt = next((row[3] for row in rows if row[0] == display[0]))
literal = b'R13 display='
tail = b' braces={ok}\n'
template = bytes([len(literal)]) + literal + b'\xc0' + bytes([len(tail)]) + tail + b'\x00'
assert len(template) == 29
assembly = ''
for (name, sink) in [('r13_constructor_stderr', stderr), ('r13_constructor_stdout', stdout)]:
    assembly += f'.section .text.{name},"ax",@progbits\n.globl {name}\n.type {name},@function\n{name}:\n sub rsp,40\n call r13_tick\n mov [rsp],rax\n lea rax,[rsp]\n mov [rsp+8],rax\n lea rax,[rip+{fmt}]\n mov [rsp+16],rax\n lea rdi,[rip+r13_template]\n lea rsi,[rsp+8]\n call {ctor}\n mov rdi,rax\n mov rsi,rdx\n call {sink}\n add rsp,40\n ret\n.size {name},.-{name}\n'
assembly += '.section .rodata.r13_template,"a",@progbits\nr13_template:\n.byte ' + ','.join((hex(b) for b in template)) + '\n.section .note.GNU-stack,"",@progbits\n'
(D / 'route.s').write_text(assembly)
(D / 'route.rs').write_text(source + '\ncore::arch::global_asm!(include_str!("route.s"));\n')
route = compile('route', D / 'route.rs', 0, ['-C', 'link-arg=-Wl,-u,r13_constructor_stderr', '-C', 'link-arg=-Wl,-u,r13_constructor_stdout'])
ordinary = compile('ordinary', D / 'display.rs', 3)
raw = route.read_bytes()
assert raw[:6] == b'\x7fELF\x02\x01'
phoff = struct.unpack_from('<Q', raw, 32)[0]
(phsize, phnum) = struct.unpack_from('<HH', raw, 54)

def offset(address, size):
    matches = []
    for i in range(phnum):
        (typ, flags, start, vma, _, filesize, _, _) = struct.unpack_from('<IIQQQQQQ', raw, phoff + i * phsize)
        if typ == 1 and vma <= address and (address + size <= vma + filesize):
            matches.append(start + address - vma)
    assert len(matches) == 1
    return matches[0]
row = next((row for row in symbols(route) if row[3] == ctor))
address = int(row[0], 16)
size = int(row[1], 16)
start = offset(address, size)
code = raw[start:start + size]
assert code == bytes.fromhex('4889f24889f848894424f048895424f8c3'), code.hex()
wrong = bytearray(raw)
wrong[start + 2] = 250
(D / 'wrong-constructor.elfdata').write_bytes(wrong)
(D / 'wrong-constructor.elfdata').chmod(292)
shoff = struct.unpack_from('<Q', raw, 40)[0]
(shsize, shnum, shnames) = struct.unpack_from('<HHH', raw, 58)
sections = [struct.unpack_from('<IIQQQQIIQQ', raw, shoff + i * shsize) for i in range(shnum)]
names = sections[shnames]
table = raw[names[4]:names[4] + names[5]]
comment = next((s for s in sections if table[s[0]:].split(b'\x00', 1)[0] == b'.comment'))
(lo, hi) = (comment[4], comment[4] + comment[5])
assert raw[lo:hi].count(b'rustc version 1.93.1') == 1
bad = raw[:lo] + raw[lo:hi].replace(b'rustc version 1.93.1', b'rustc version 1.94.1') + raw[hi:]
(D / 'unsupported-version.elfdata').write_bytes(bad)
(D / 'unsupported-version.elfdata').chmod(292)
proof = {'compiler': version, 'compiler_sha256': sha256(Path(shutil.which(args.rustc)).read_bytes()).hexdigest(), 'commands': commands, 'source_sha256': sha256(source.encode()).hexdigest(), 'constructor': {'symbol': ctor, 'address': address, 'size': size, 'bytes': code.hex(), 'effect': 'rdi/rsi -> rax/rdx; two exact RSP-relative stores; caller disjointness still required; return'}, 'formatter': fmt, 'sink': stderr, 'template_hex': template.hex(), 'artifacts': {p.name: sha256(p.read_bytes()).hexdigest() for p in sorted(D.iterdir()) if p.is_file()}, 'specimen_execution': False}
(D / 'build-proof.json').write_text(json.dumps(proof, indent=2) + '\n')
print(D / 'route.elfdata')
