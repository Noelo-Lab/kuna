#!/usr/bin/env python3
"""Build fileset_calls.macho: an arm64 MH_FILESET whose kext entry calls one of
its own functions from five `bl` sites, at arm64e-kernelcache addresses.

Entries: com.apple.kernel (MH_EXECUTE: kernel_add, kernel_main, which calls
kernel_add) and com.example.driver.PMGR (MH_KEXT_BUNDLE: pmgr_set_perf_state
calls pmgr_wait_pending from four `bl` sites and kernel_add from one,
pmgr_idle_enter calls pmgr_wait_pending with a backward `bl`). Each entry has a
__TEXT,__cstring string of its own. No fixup table, one shared LINKEDIT, file
offset == vmaddr - BASE. Pure Python, no toolchain.
Run from this directory: python3 gen_fileset_calls.py
"""
import struct

BASE = 0xFFFFFE0007004000
PAGE = 0x1000
KERNEL_HDR, KEXT_HDR = 0x1000, 0x2000
KERNEL_CSTRING, KEXT_CSTRING = 0x1400, 0x2400
KERNEL_TEXT, KEXT_TEXT = 0x3000, 0x3800
LINKEDIT = 0x4000


def u32(*v): return struct.pack("<" + "I" * len(v), *v)
def u64(*v): return struct.pack("<" + "Q" * len(v), *v)
def bl(pc, target): return 0x94000000 | (((target - pc) // 4) & 0x3FFFFFF)


RET, FRAME_IN, FRAME_SP, FRAME_OUT = 0xD65F03C0, 0xA9BF7BFD, 0x910003FD, 0xA8C17BFD

kernel_add = KERNEL_TEXT
kernel_main = KERNEL_TEXT + 8
kernel_code = [0x0B010000, RET,
               FRAME_IN, FRAME_SP, 0x52800061, bl(kernel_main + 12, kernel_add), FRAME_OUT, RET]

set_perf_state = KEXT_TEXT
wait_pending = KEXT_TEXT + 0x40
idle_enter = KEXT_TEXT + 0x60
perf_body = [FRAME_IN, FRAME_SP, "wait", 0x52800021, "wait", "wait", 0x52800041, "wait", "kernel",
             FRAME_OUT, RET]
perf_code = []
for i, word in enumerate(perf_body):
    pc = set_perf_state + 4 * i
    perf_code.append({"wait": bl(pc, wait_pending), "kernel": bl(pc, kernel_add)}.get(word, word))
wait_code = [0x52800020, RET]
idle_code = [FRAME_IN, FRAME_SP, bl(idle_enter + 8, wait_pending), FRAME_OUT, RET]
kext_text_len = idle_enter + 4 * len(idle_code) - KEXT_TEXT

kernel_string, kext_string = b"xnu-example-kernel\0", b"AppleExamplePMGR\0"

names = [b"_kernel_add", b"_kernel_main", b"_pmgr_set_perf_state", b"_pmgr_wait_pending",
         b"_pmgr_idle_enter"]
strtab = b"\0"
stroff_of = {}
for n in names:
    stroff_of[n] = len(strtab)
    strtab += n + b"\0"
strtab = strtab.ljust((len(strtab) + 7) & ~7, b"\0")

N_SECT_EXT, N_UNDF_EXT = 0x0F, 0x01
def nlist(name, ntype, sect, value):
    return u32(stroff_of[name]) + bytes([ntype, sect]) + struct.pack("<H", 0) + u64(value)

kernel_syms = [nlist(b"_kernel_add", N_SECT_EXT, 2, BASE + kernel_add),
               nlist(b"_kernel_main", N_SECT_EXT, 2, BASE + kernel_main)]
kext_syms = [nlist(b"_pmgr_set_perf_state", N_SECT_EXT, 2, BASE + set_perf_state),
             nlist(b"_pmgr_wait_pending", N_SECT_EXT, 2, BASE + wait_pending),
             nlist(b"_pmgr_idle_enter", N_SECT_EXT, 2, BASE + idle_enter),
             nlist(b"_kernel_add", N_UNDF_EXT, 0, 0)]

linkedit = b""
def put(blob):
    global linkedit
    off = LINKEDIT + len(linkedit)
    linkedit += blob
    return off
kernel_symoff = put(b"".join(kernel_syms))
kext_symoff = put(b"".join(kext_syms))
stroff = put(strtab)
linkedit_size = len(linkedit)


def section(seg, name, addr, size, flags=0):
    return (name.ljust(16, b"\0") + seg.ljust(16, b"\0") + u64(BASE + addr, size)
            + u32(addr, 0, 0, 0, flags, 0, 0, 0))

def segment(name, addr, size, sections=(), prot=5):
    body = name.ljust(16, b"\0") + u64(BASE + addr, size, addr, size)
    body += u32(prot, prot, len(sections), 0) + b"".join(sections)
    return u32(0x19, 8 + len(body)) + body

def symtab(symoff, nsyms): return u32(0x2, 24, symoff, nsyms, stroff, len(strtab))
def dysymtab(ndef, nundef):
    return u32(0xB, 80, 0, 0, 0, ndef, ndef, nundef, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)

def header(filetype, cmds, flags=0):
    return u32(0xFEEDFACF, 0x0100000C, 0, filetype, len(cmds), sum(map(len, cmds)), flags, 0)

TEXT_FLAGS = 0x80000400
CSTRING_FLAGS = 0x2
MH_DYLIB_IN_CACHE = 0x80000000
linkedit_seg = lambda: segment(b"__LINKEDIT", LINKEDIT, linkedit_size, prot=1)
kernel_cmds = [
    segment(b"__TEXT", KERNEL_HDR, PAGE,
            [section(b"__TEXT", b"__cstring", KERNEL_CSTRING, len(kernel_string), CSTRING_FLAGS)], prot=1),
    segment(b"__TEXT_EXEC", KERNEL_TEXT, 0x800,
            [section(b"__TEXT_EXEC", b"__text", KERNEL_TEXT, 4 * len(kernel_code), TEXT_FLAGS)]),
    linkedit_seg(),
    symtab(kernel_symoff, len(kernel_syms)),
    dysymtab(len(kernel_syms), 0),
]
kext_cmds = [
    segment(b"__TEXT", KEXT_HDR, PAGE,
            [section(b"__TEXT", b"__cstring", KEXT_CSTRING, len(kext_string), CSTRING_FLAGS)], prot=1),
    segment(b"__TEXT_EXEC", KEXT_TEXT, 0x800,
            [section(b"__TEXT_EXEC", b"__text", KEXT_TEXT, kext_text_len, TEXT_FLAGS)]),
    linkedit_seg(),
    symtab(kext_symoff, len(kext_syms)),
    dysymtab(len(kext_syms) - 1, 1),
]

def fileset_entry(addr, name):
    body = u64(BASE + addr, addr) + u32(32, 0) + name + b"\0"
    body = body.ljust((len(body) + 8 + 7) & ~7, b"\0")
    return u32(0x80000035, 8 + len(body)) + body

top_cmds = [
    segment(b"__TEXT", 0, KERNEL_TEXT, prot=1),
    segment(b"__TEXT_EXEC", KERNEL_TEXT, PAGE),
    segment(b"__LINKEDIT", LINKEDIT, linkedit_size, prot=1),
    fileset_entry(KERNEL_HDR, b"com.apple.kernel"),
    fileset_entry(KEXT_HDR, b"com.example.driver.PMGR"),
]

image = bytearray(LINKEDIT + linkedit_size)
def place(off, blob): image[off:off + len(blob)] = blob
place(0, header(0xC, top_cmds) + b"".join(top_cmds))
place(KERNEL_HDR, header(0x2, kernel_cmds, MH_DYLIB_IN_CACHE | 0x1) + b"".join(kernel_cmds))
place(KEXT_HDR, header(0xB, kext_cmds, MH_DYLIB_IN_CACHE | 0x85) + b"".join(kext_cmds))
place(KERNEL_CSTRING, kernel_string)
place(KEXT_CSTRING, kext_string)
place(KERNEL_TEXT, u32(*kernel_code))
place(set_perf_state, u32(*perf_code))
place(wait_pending, u32(*wait_code))
place(idle_enter, u32(*idle_code))
place(LINKEDIT, linkedit)
open("fileset_calls.macho", "wb").write(image)
