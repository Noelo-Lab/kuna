#!/usr/bin/env python3
"""Author static int/error contracts and rejection controls; never run the images."""
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent
CHECK = """cmp $-2,%rax
je 1f
test %rax,%rax
js 2f
inc %rax
xor %ebx,%ebx
xor %ecx,%ecx
ret
1: mov $17,%eax
lea main.table(%rip),%rbx
xor %ecx,%ecx
ret
2: mov $23,%eax
lea main.table(%rip),%rbx
lea main.fault(%rip),%rcx
ret
"""
CONSUME = """call main.forward
test %rbx,%rbx
je 1f
lea main.table(%rip),%rdx
cmp %rdx,%rbx
jne 2f
test %rcx,%rcx
jne 2f
add $100,%rax
ret
2: add $200,%rax
1: ret
"""
FUNCTIONS = [
    ("check", CHECK, ["int", "error"]),
    ("forward", "call main.check; ret", ["int", "error"]),
    ("consume", CONSUME, ["int"]),
    ("modernForward", "call main.check; ret", ["int", "modern_error"]),
    ("wrongFirst", CHECK, ["i64", "error"]),
    ("badKind", CHECK, ["int", "bad_kind"]),
    ("badOffset", CHECK, ["int", "bad_offset"]),
    ("singleRegister", CHECK, ["int", "error"]),
    ("missingLocation", CHECK, ["int", "error"]),
    ("thirdResult", CHECK, ["int", "error", "int"]),
    ("interfaceInput", "ret", ["int", "error"]),
    ("wrongProducer", CHECK, ["int", "error"]),
    ("duplicate", CHECK, ["int", "error"]),
    ("forward.abi0", CHECK, ["int", "error"]),
]
lines = [".text", ".global _start", "_start: ret"]


def emit(*items):
    lines.extend(items)


def string(value):
    return '.asciz "' + value + '"'


def ref(name):
    return ".long type_" + name + "-debug_info"


def abbrev(number, tag, children, attrs):
    emit(f".uleb128 {number}", f".uleb128 {tag}", f".byte {children}")
    for attribute, form in attrs:
        emit(f".uleb128 {attribute}", f".uleb128 {form}")
    emit(".byte 0,0")


for name, body, _ in FUNCTIONS:
    emit(".p2align 4", f".global main.{name}", f".type main.{name},@function",
         f"main.{name}:", body, f".size main.{name},.-main.{name}")
emit(".data", ".balign 8", ".global main.table", "main.table: .quad 0,0,0,0",
     ".global main.fault", "main.fault: .quad 3", '.section .go.buildinfo,"a"',
     ".balign 16", '.ascii "\\377 Go buildinf:"', ".byte 8,2", ".zero 16",
     ".byte 9", '.ascii "go1.22.10"', ".byte 0", '.section .debug_abbrev,"",@progbits')
abbrev(1, 0x11, 1, [(3, 8), (0x13, 5), (0x25, 8)])
abbrev(2, 0x2e, 1, [(3, 8), (0x11, 1)])
abbrev(3, 5, 0, [(3, 8), (0x4b, 0xc), (0x49, 0x10), (2, 0x18)])
abbrev(4, 0x24, 0, [(3, 8), (0x3e, 0xb), (0xb, 0xb), (0x2900, 0xb)])
abbrev(5, 0x16, 0, [(3, 8), (0x49, 0x10)])
abbrev(6, 0x16, 0, [(3, 8), (0x49, 0x10), (0x2900, 0xb)])
abbrev(7, 0x13, 1, [(3, 8), (0xb, 0xb), (0x2900, 0xb)])
abbrev(8, 0xd, 0, [(3, 8), (0x49, 0x10), (0x38, 0xb)])
abbrev(9, 0xf, 0, [(3, 8), (0x49, 0x10), (0x2900, 0xb)])
abbrev(10, 0xf, 0, [(3, 8)])
abbrev(11, 5, 0, [(3, 8), (0x4b, 0xc), (0x49, 0x10)])
emit(".byte 0", '.section .debug_info,"",@progbits', "debug_info:")


def cu_start(label, producer="Go cmd/compile go1.22.10; regabi"):
    emit(label + ":", f".long {label}_end-{label}-4", ".short 4", ".long 0", ".byte 8,1",
         string("main"), ".short 22", string(producer))


def cu_end(label):
    emit(".byte 0", label + "_end:")


def function(name, results):
    emit(".byte 2", string("main." + name), ".quad main." + name,
         ".byte 3", string("n"), ".byte 0",
         ref("error" if name == "interfaceInput" else "int"), ".byte 1,0x50")
    for index, ty in enumerate(results):
        missing = name == "missingLocation" and index == 1
        emit(".byte " + ("11" if missing else "3"), string("~r" + str(index)), ".byte 1", ref(ty))
        if not missing:
            emit(".byte 1,0x53" if name == "singleRegister" and index == 1 else ".byte 0")
    emit(".byte 0")


cu_start("functions")
for name, _, results in FUNCTIONS:
    if name != "wrongProducer":
        function(name, results)
cu_end("functions")
cu_start("duplicates")
function("duplicate", ["int", "error"])
cu_end("duplicates")
cu_start("old", "Go cmd/compile go1.16.15")
function("wrongProducer", ["int", "error"])
cu_end("old")
cu_start("types")
emit("type_int:", ".byte 4", string("int"), ".byte 5,8,2",
     "type_i64:", ".byte 4", string("int64"), ".byte 5,8,6",
     "type_unsafe:", ".byte 10", string("unsafe.Pointer"))
for label, table, kind, data_offset in [
    ("error", "runtime.itab", 20, 8),
    ("modern_error", "internal/abi.ITab", 20, 8),
    ("bad_kind", "runtime.itab", 25, 8),
    ("bad_offset", "runtime.itab", 20, 0),
]:
    emit("type_" + label + ":", ".byte 5", string("error"), ref(label + "_kind"),
         "type_" + label + "_kind:", ".byte 6", string("error"), ref(label + "_iface"), f".byte {kind}",
         "type_" + label + "_iface:", ".byte 5", string("runtime.iface"), ref(label + "_storage"),
         "type_" + label + "_storage:", ".byte 7", string("runtime.iface"), ".byte 16,25",
         ".byte 8", string("tab"), ref(label + "_tabptr"), ".byte 0",
         ".byte 8", string("data"), ref("unsafe"), f".byte {data_offset}", ".byte 0",
         "type_" + label + "_tabptr:", ".byte 9", string("*" + table), ref(label + "_tabalias"), ".byte 22",
         "type_" + label + "_tabalias:", ".byte 5", string(table), ref(label + "_tab"),
         "type_" + label + "_tab:", ".byte 7", string(table), ".byte 32,25", ".byte 0")
cu_end("types")
emit('.section .note.GNU-stack,"",@progbits')
with tempfile.TemporaryDirectory(prefix="kuna-go-int-error-") as directory:
    directory = Path(directory)
    source, obj = directory / "fixture.S", directory / "fixture.o"
    source.write_text("\n".join(lines) + "\n")
    for name in ["contracts-plain", "contracts"]:
        if (ROOT / name).exists():
            (ROOT / name).chmod(0o644)
    subprocess.run(["as", "--64", str(source), "-o", str(obj)], check=True)
    subprocess.run(["ld", "-m", "elf_x86_64", "-e", "_start", "-Ttext=0x401000", "--build-id=none",
                    "-o", str(ROOT / "contracts-plain"), str(obj)], check=True)
    subprocess.run(["objcopy", "--compress-debug-sections=zlib", str(ROOT / "contracts-plain"),
                    str(ROOT / "contracts")], check=True)
    for name in ["contracts-plain", "contracts"]:
        (ROOT / name).chmod(0o444)
