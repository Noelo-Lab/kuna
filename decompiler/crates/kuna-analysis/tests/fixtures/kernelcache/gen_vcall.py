#!/usr/bin/env python3
"""Build `fileset_vcall.macho`, the kernel-collection fixture for `arm64evcall`.

An arm64 MH_FILESET with two entries sharing one DYLD_CHAINED_PTR_64_KERNEL_CACHE
table. The kernel (MH_EXECUTE) has a vtable `{0, 0, Base_m0, Base_m1}` and two
virtual call sites; the kext (MH_KEXT_BUNDLE) a vtable `{0, 0, Base_m0,
Widget_m1}` that overrides slot 1. A `__PRELINK_INFO` plist with only
`_PrelinkKCID` marks it a boot collection (cache level 0). Every slot is signed key IA,
address-diversified: slot 0 with 0x1000, slot 1 with 0x2001.

    kernel_vcall_m0   ldr x8, [x16, #0]!          -> Base_m0 alone (both vtables)
    kernel_vcall_m1   mov x17, #8; add x16, x16, x17 -> Base_m1 or Widget_m1

Plain Python 3. Run from this directory: python3 gen_vcall.py
"""
import struct

BASE = 0xFFFFFFF007004000
PAGE = 0x1000

KERNEL_HDR, KEXT_HDR = 0x1000, 0x2000
KERNEL_TEXT, KEXT_TEXT = 0x3000, 0x3800
KERNEL_CONST, KEXT_CONST = 0x4000, 0x4800
LINKEDIT = 0x5000

def u32(*v): return struct.pack("<" + "I" * len(v), *v)
def u64(*v): return struct.pack("<" + "Q" * len(v), *v)

RET = 0xD65F03C0
def movz_w0(v): return 0x52800000 | (v << 5)
def movk48(rd, v): return 0xF2E00000 | (v << 5) | rd

def vcall_prologue():
    return [0xF9400010,              # ldr   x16, [x0]
            0xAA0003F1,              # mov   x17, x0
            movk48(17, 0x6AE1),      # movk  x17, #0x6ae1, lsl #48
            0xDAC11A30]              # autda x16, x17

BRAA_X8_X17 = 0xD71F0911
base_m0, base_m1 = KERNEL_TEXT, KERNEL_TEXT + 8
kernel_vcall_m0 = KERNEL_TEXT + 16
kernel_vcall_m1 = kernel_vcall_m0 + 4 * 8
kernel_code = [movz_w0(1), RET, movz_w0(2), RET,
               *vcall_prologue(),
               0xF8400E08,           # ldr   x8, [x16, #0]!
               0xAA1003F1,           # mov   x17, x16
               movk48(17, 0x1000), BRAA_X8_X17,
               *vcall_prologue(),
               0xD2800111,           # mov   x17, #8
               0x8B110210,           # add   x16, x16, x17
               0xF9400208,           # ldr   x8, [x16]
               0xAA1003F1,           # mov   x17, x16
               movk48(17, 0x2001), BRAA_X8_X17]
assert kernel_vcall_m1 == KERNEL_TEXT + 4 * 12, hex(kernel_vcall_m1)
widget_m1 = KEXT_TEXT
kext_code = [movz_w0(3), RET]

def section(seg, name, addr, size, offset, flags=0):
    return (name.ljust(16, b"\0") + seg.ljust(16, b"\0") + u64(BASE + addr, size)
            + u32(offset, 0, 0, 0, flags, 0, 0, 0))

def segment(name, addr, size, sections=(), prot=5):
    body = name.ljust(16, b"\0") + u64(BASE + addr, size, addr, size)
    body += u32(prot, prot, len(sections), 0) + b"".join(sections)
    return u32(0x19, 8 + len(body)) + body

def header(filetype, cmds):
    return u32(0xFEEDFACF, 0x0100000C, 0, filetype, len(cmds), sum(map(len, cmds)), 0, 0)

names = [b"_Base_m0", b"_Base_m1", b"_kernel_vcall_m0", b"_kernel_vcall_m1", b"_Widget_m1"]
strtab = b"\0"
stroff_of = {}
for n in names:
    stroff_of[n] = len(strtab)
    strtab += n + b"\0"
strtab = strtab.ljust((len(strtab) + 7) & ~7, b"\0")

N_SECT_EXT = 0x0F
def nlist(name, value):
    return u32(stroff_of[name]) + bytes([N_SECT_EXT, 1]) + struct.pack("<H", 0) + u64(BASE + value)

kernel_syms = [nlist(b"_Base_m0", base_m0), nlist(b"_Base_m1", base_m1),
               nlist(b"_kernel_vcall_m0", kernel_vcall_m0), nlist(b"_kernel_vcall_m1", kernel_vcall_m1)]
kext_syms = [nlist(b"_Widget_m1", widget_m1)]

KEY_IA = 0
def kc_auth(target, nxt, diversity):
    return target | (diversity << 32) | (1 << 48) | (KEY_IA << 49) | (nxt << 51) | (1 << 63)

slots = [(KERNEL_CONST + 0x10, base_m0, 0x1000), (KERNEL_CONST + 0x18, base_m1, 0x2001),
         (KEXT_CONST + 0x10, base_m0, 0x1000), (KEXT_CONST + 0x18, widget_m1, 0x2001)]
fixups_at = {}
for i, (at, target, div) in enumerate(slots):
    nxt = (slots[i + 1][0] - at) // 4 if i + 1 < len(slots) else 0
    fixups_at[at] = kc_auth(target, nxt, div)

starts_seg = u32(24) + struct.pack("<HH", PAGE, 8) + u64(KERNEL_CONST) + u32(0) + struct.pack("<HH", 1, 0x10)
seg_count = 4
starts_image = u32(seg_count, 0, 0, 4 + 4 * seg_count, 0) + starts_seg
fx_header_len = 32
fixups = u32(0, fx_header_len, fx_header_len + len(starts_image), fx_header_len + len(starts_image), 0, 1, 0, 0)
fixups += starts_image
fixups = fixups.ljust((len(fixups) + 7) & ~7, b"\0")

linkedit = b""
def put(blob):
    global linkedit
    off = LINKEDIT + len(linkedit)
    linkedit += blob
    return off
fixups_off = put(fixups)
kernel_symoff = put(b"".join(kernel_syms))
kext_symoff = put(b"".join(kext_syms))
stroff = put(strtab)
linkedit_size = len(linkedit)

def symtab(symoff, nsyms): return u32(0x2, 24, symoff, nsyms, stroff, len(strtab))
def dysymtab(ndef):
    return u32(0xB, 80, 0, 0, 0, ndef, ndef, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)

linkedit_seg = lambda: segment(b"__LINKEDIT", LINKEDIT, linkedit_size, prot=1)
TEXT_FLAGS = 0x80000400
def entry_cmds(hdr, text, code, const, syms, nsyms):
    return [segment(b"__TEXT", hdr, PAGE, prot=5),
            segment(b"__TEXT_EXEC", text, 0x800,
                    [section(b"__TEXT_EXEC", b"__text", text, 4 * len(code), text, TEXT_FLAGS)]),
            segment(b"__DATA_CONST", const, 0x800,
                    [section(b"__DATA_CONST", b"__const", const, 0x20, const)], prot=1),
            linkedit_seg(), symtab(syms, nsyms), dysymtab(nsyms)]
kernel_cmds = entry_cmds(KERNEL_HDR, KERNEL_TEXT, kernel_code, KERNEL_CONST, kernel_symoff, len(kernel_syms))
kext_cmds = entry_cmds(KEXT_HDR, KEXT_TEXT, kext_code, KEXT_CONST, kext_symoff, len(kext_syms))

def fileset_entry(addr, name):
    body = u64(BASE + addr, addr) + u32(32, 0) + name + b"\0"
    body = body.ljust((len(body) + 8 + 7) & ~7, b"\0")
    return u32(0x80000035, 8 + len(body)) + body

PRELINK = (LINKEDIT + linkedit_size + PAGE - 1) & ~(PAGE - 1)
prelink_plist = (b'<?xml version="1.0" encoding="UTF-8"?>\n<plist version="1.0">\n<dict>\n'
                 b'\t<key>_PrelinkInfoDictionary</key>\n\t<array/>\n'
                 b'\t<key>_PrelinkKCID</key>\n\t<data>AAAAAAAAAAAAAAAAAAAAAA==</data>\n</dict>\n</plist>\n\0')

top_cmds = [
    segment(b"__TEXT", 0, 0x3000, prot=1),
    segment(b"__TEXT_EXEC", KERNEL_TEXT, PAGE),
    segment(b"__DATA_CONST", KERNEL_CONST, PAGE, prot=1),
    segment(b"__LINKEDIT", LINKEDIT, linkedit_size, prot=1),
    segment(b"__PRELINK_INFO", PRELINK, len(prelink_plist), prot=1),
    u32(0x80000034, 16, fixups_off, len(fixups)),
    fileset_entry(KERNEL_HDR, b"com.apple.kernel"),
    fileset_entry(KEXT_HDR, b"com.example.driver.Widget"),
]

image = bytearray(PRELINK + len(prelink_plist))
def place(off, blob): image[off:off + len(blob)] = blob
place(0, header(0xC, top_cmds) + b"".join(top_cmds))
place(KERNEL_HDR, header(0x2, kernel_cmds) + b"".join(kernel_cmds))
place(KEXT_HDR, header(0xB, kext_cmds) + b"".join(kext_cmds))
place(KERNEL_TEXT, u32(*kernel_code))
place(KEXT_TEXT, u32(*kext_code))
for off, raw in fixups_at.items():
    place(off, u64(raw))
place(LINKEDIT, linkedit)
place(PRELINK, prelink_plist)
open("fileset_vcall.macho", "wb").write(image)
