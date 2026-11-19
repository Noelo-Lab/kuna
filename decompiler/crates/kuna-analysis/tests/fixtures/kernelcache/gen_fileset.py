#!/usr/bin/env python3
"""Build the synthetic kernel-collection fixtures for the `kernelcache` option.

fileset.macho          an arm64 MH_FILESET with two entries: com.apple.kernel
                       (MH_EXECUTE) and com.example.driver.Widget (MH_KEXT_BUNDLE),
                       one shared LC_DYLD_CHAINED_FIXUPS table in
                       DYLD_CHAINED_PTR_64_KERNEL_CACHE format, one shared LINKEDIT.
kernelcache.img4       IMG4 { IM4P 'krnl' { LZFSE(fileset.macho) }, IM4M stub }.
kernelcache.lzss.im4p  a bare IM4P whose payload is a complzss container.

Needs the reference `lzfse` CLI (github.com/lzfse/lzfse) on PATH.
Run from this directory: python3 gen_fileset.py
"""
import struct
import subprocess
import zlib

BASE = 0xFFFFFFF007004000
PAGE = 0x1000

# File offset == vmaddr - BASE throughout, as in a real collection.
KERNEL_HDR, KEXT_HDR = 0x1000, 0x2000
KEXT_CSTRING = 0x2400
KERNEL_TEXT, KEXT_TEXT = 0x3000, 0x3800
KERNEL_CONST, KEXT_GOT, KEXT_CONST = 0x4000, 0x4800, 0x4808
LINKEDIT = 0x5000

def u32(*v): return struct.pack("<" + "I" * len(v), *v)
def u64(*v): return struct.pack("<" + "Q" * len(v), *v)

def adrp(rd, pc, target):
    delta = ((BASE + target) >> 12) - ((BASE + pc) >> 12)
    return 0x90000000 | ((delta & 3) << 29) | (((delta >> 2) & 0x7FFFF) << 5) | rd

def ldr(rt, rn, off): return 0xF9400000 | ((off // 8) << 10) | (rn << 5) | rt
def bl(pc, target): return 0x94000000 | (((target - pc) // 4) & 0x3FFFFFF)

RET, BR_X8, BR_X16 = 0xD65F03C0, 0xD61F0100, 0xD61F0200

kernel_add = KERNEL_TEXT
kernel_dispatch = KERNEL_TEXT + 8
kernel_code = [0x0B010000, RET,                                  # w0 = w0 + w1
               adrp(8, kernel_dispatch, KERNEL_CONST),
               ldr(8, 8, KERNEL_CONST % PAGE), BR_X8]             # tail call table[0]
widget_start = KEXT_TEXT
widget_name = KEXT_TEXT + 24
widget_via_stub = KEXT_TEXT + 36
widget_kernel_table = KEXT_TEXT + 60
stub = KEXT_TEXT + 0x100
kext_code = [0xA9BF7BFD, 0x910003FD, 0x528000E1,                  # frame; w1 = 7
             bl(widget_start + 12, kernel_add), 0xA8C17BFD, RET,   # direct kernel_add(w0, 7)
             adrp(0, widget_name, KEXT_CONST), ldr(0, 0, KEXT_CONST % PAGE), RET,
             0xA9BF7BFD, 0x910003FD, 0x52800101,                  # frame; w1 = 8
             bl(widget_via_stub + 12, stub), 0xA8C17BFD, RET,      # kernel_add(w0, 8) via __stubs
             adrp(0, widget_kernel_table, KERNEL_CONST),          # &kernel_table, a kernel data object
             0x91000000 | ((KERNEL_CONST % PAGE) << 10), RET]
stub_code = [adrp(16, stub, KEXT_GOT), ldr(16, 16, KEXT_GOT % PAGE), BR_X16]

def section(seg, name, addr, size, offset, flags=0, r1=0, r2=0):
    return (name.ljust(16, b"\0") + seg.ljust(16, b"\0") + u64(BASE + addr, size)
            + u32(offset, 0, 0, 0, flags, r1, r2, 0))

def segment(name, addr, size, sections=(), prot=5):
    body = name.ljust(16, b"\0") + u64(BASE + addr if size else BASE + addr, size, addr, size)
    body += u32(prot, prot, len(sections), 0) + b"".join(sections)
    return u32(0x19, 8 + len(body)) + body

MH_DYLIB_IN_CACHE = 0x80000000

def header(filetype, cmds, subtype=0, flags=0):
    return u32(0xFEEDFACF, 0x0100000C, subtype, filetype, len(cmds), sum(map(len, cmds)), flags, 0)

# Shared LINKEDIT: string pool, two symbol tables, the indirect table, fixups.
names = [b"_kernel_add", b"_kernel_dispatch", b"_kernel_table", b"_widget_start", b"_widget_name",
         b"_widget_via_stub", b"_widget_kernel_table"]
strtab = b"\0"
stroff_of = {}
for n in names:
    stroff_of[n] = len(strtab)
    strtab += n + b"\0"
strtab = strtab.ljust((len(strtab) + 7) & ~7, b"\0")

N_SECT_EXT, N_UNDF_EXT = 0x0F, 0x01
def nlist(name, ntype, sect, value):
    return u32(stroff_of[name]) + bytes([ntype, sect]) + struct.pack("<H", 0) + u64(value)

kernel_syms = [nlist(b"_kernel_add", N_SECT_EXT, 1, BASE + kernel_add),
               nlist(b"_kernel_dispatch", N_SECT_EXT, 1, BASE + kernel_dispatch),
               nlist(b"_kernel_table", N_SECT_EXT, 2, BASE + KERNEL_CONST)]
kext_syms = [nlist(b"_widget_start", N_SECT_EXT, 2, BASE + widget_start),
             nlist(b"_widget_name", N_SECT_EXT, 2, BASE + widget_name),
             nlist(b"_widget_via_stub", N_SECT_EXT, 2, BASE + widget_via_stub),
             nlist(b"_widget_kernel_table", N_SECT_EXT, 2, BASE + widget_kernel_table),
             nlist(b"_kernel_add", N_UNDF_EXT, 0, 0)]

def kc_ptr(target, nxt, auth=False, diversity=0):
    return target | (diversity << 32) | (nxt << 51) | (int(auth) << 63)

fixups_at = {KERNEL_CONST: kc_ptr(kernel_add, (KEXT_GOT - KERNEL_CONST) // 4),
             KEXT_GOT: kc_ptr(kernel_add, 2, auth=True, diversity=0x8F1C),
             KEXT_CONST: kc_ptr(KEXT_CSTRING, 0)}

starts_seg = u32(24, ) + struct.pack("<HH", PAGE, 8) + u64(KERNEL_CONST) + u32(0) + struct.pack("<HH", 1, 0)
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
indirect_off = put(u32(4, 4))
stroff = put(strtab)
linkedit_size = len(linkedit)

def symtab(symoff, nsyms): return u32(0x2, 24, symoff, nsyms, stroff, len(strtab))
def dysymtab(ndef, nundef, indoff=0, nind=0):
    return u32(0xB, 80, 0, 0, 0, ndef, ndef, nundef, 0, 0, 0, 0, 0, 0, indoff, nind, 0, 0, 0, 0)

linkedit_seg = lambda: segment(b"__LINKEDIT", LINKEDIT, linkedit_size, prot=1)
TEXT_FLAGS = 0x80000400
kernel_cmds = [
    segment(b"__TEXT", KERNEL_HDR, PAGE, prot=5),
    segment(b"__TEXT_EXEC", KERNEL_TEXT, 0x800,
            [section(b"__TEXT_EXEC", b"__text", KERNEL_TEXT, 4 * len(kernel_code), KERNEL_TEXT, TEXT_FLAGS)]),
    segment(b"__DATA_CONST", KERNEL_CONST, 0x800,
            [section(b"__DATA_CONST", b"__const", KERNEL_CONST, 8, KERNEL_CONST)], prot=1),
    linkedit_seg(),
    symtab(kernel_symoff, len(kernel_syms)),
    dysymtab(3, 0),
]
kext_cmds = [
    segment(b"__TEXT", KEXT_HDR, PAGE,
            [section(b"__TEXT", b"__cstring", KEXT_CSTRING, 7, KEXT_CSTRING, 0x2)]),
    segment(b"__TEXT_EXEC", KEXT_TEXT, 0x800,
            [section(b"__TEXT_EXEC", b"__text", KEXT_TEXT, 4 * len(kext_code), KEXT_TEXT, TEXT_FLAGS),
             section(b"__TEXT_EXEC", b"__stubs", stub, 12, stub, TEXT_FLAGS | 0x8, 0, 12)]),
    segment(b"__DATA_CONST", KEXT_GOT, 0x800,
            [section(b"__DATA_CONST", b"__got", KEXT_GOT, 8, KEXT_GOT, 0x6, 1),
             section(b"__DATA_CONST", b"__const", KEXT_CONST, 8, KEXT_CONST)], prot=1),
    linkedit_seg(),
    symtab(kext_symoff, len(kext_syms)),
    dysymtab(4, 1, indirect_off, 2),
]

def fileset_entry(addr, name):
    body = u64(BASE + addr, addr) + u32(32, 0) + name + b"\0"
    body = body.ljust((len(body) + 8 + 7) & ~7, b"\0")
    return u32(0x80000035, 8 + len(body)) + body

PRELINK = (LINKEDIT + linkedit_size + PAGE - 1) & ~(PAGE - 1)
prelink_plist = (b'<?xml version="1.0" encoding="UTF-8"?>\n<plist version="1.0">\n<dict>\n'
                 b'\t<key>_PrelinkInfoDictionary</key>\n\t<array/>\n'
                 b'\t<!--' + b"-" * 56 + b'-->\n'
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
place(KERNEL_HDR, header(0x2, kernel_cmds, flags=MH_DYLIB_IN_CACHE | 0x1) + b"".join(kernel_cmds))
place(KEXT_HDR, header(0xB, kext_cmds, flags=MH_DYLIB_IN_CACHE | 0x85) + b"".join(kext_cmds))
place(KEXT_CSTRING, b"Widget\0")
place(KERNEL_TEXT, u32(*kernel_code))
place(KEXT_TEXT, u32(*kext_code))
place(stub, u32(*stub_code))
for off, raw in fixups_at.items():
    place(off, u64(raw))
place(LINKEDIT, linkedit)
place(PRELINK, prelink_plist)
open("fileset.macho", "wb").write(image)

def der(tag, value):
    n = len(value)
    length = bytes([n]) if n < 0x80 else bytes([0x84]) + n.to_bytes(4, "big")
    return bytes([tag]) + length + value

def im4p(payload):
    body = der(0x16, b"IM4P") + der(0x16, b"krnl") + der(0x16, b"KernelManagement_host-test")
    body += der(0x04, payload)
    body += der(0x30, der(0x02, b"\x01") + der(0x02, len(image).to_bytes(4, "big")))
    return der(0x30, body)

subprocess.run(["lzfse", "-encode", "-i", "fileset.macho", "-o", "fileset.lzfse"], check=True)
lzfse = open("fileset.lzfse", "rb").read()
manifest = der(0xA0, der(0x30, der(0x16, b"IM4M") + der(0x02, b"\x00")))
open("kernelcache.img4", "wb").write(der(0x30, der(0x16, b"IMG4") + im4p(lzfse) + manifest))

def lzss_encode(data, window=4096, max_len=18, min_len=3):
    """Greedy LZSS over the decoder's 4 KiB ring, which starts writing at 0xFEE."""
    out, flags, group, chains = bytearray(), 0, bytearray(), {}
    i, bit = 0, 0
    while i < len(data):
        best_len, best_pos = 0, 0
        for cand in reversed(chains.get(data[i:i + min_len], [])[-32:]):
            if i - cand >= window - max_len:
                break
            n = 0
            while n < max_len and i + n < len(data) and data[cand + n] == data[i + n]:
                n += 1
            if n > best_len:
                best_len, best_pos = n, cand
        if best_len >= min_len:
            ring = (best_pos + window - max_len) & (window - 1)
            group += bytes([ring & 0xFF, ((ring >> 4) & 0xF0) | (best_len - min_len)])
            step = best_len
        else:
            flags |= 1 << bit
            group.append(data[i])
            step = 1
        for k in range(i, i + step):
            chains.setdefault(data[k:k + min_len], []).append(k)
        i += step
        bit += 1
        if bit == 8:
            out += bytes([flags]) + group
            flags, group, bit = 0, bytearray(), 0
    if bit:
        out += bytes([flags]) + group
    return bytes(out)

stream = lzss_encode(bytes(image))
lzss = b"complzss" + struct.pack(">IIII", zlib.adler32(bytes(image)), len(image), len(stream), 1)
lzss = lzss.ljust(0x180, b"\0") + stream
open("kernelcache.lzss.im4p", "wb").write(im4p(lzss))
