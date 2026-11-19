#!/usr/bin/env python3
"""Build the linked kernel-collection fixtures for the `kernelcache-boot` option.

linked/BootKernelExtensions.kc       an arm64 MH_FILESET boot collection: one entry,
                                     com.apple.kernel, defining kernel_add and
                                     kernel_log. Its LC_UUID and _PrelinkKCID are
                                     the UUID the Auxiliary collection names.
linked/AuxiliaryKernelExtensions.kc  an arm64 MH_FILESET at another base: one entry,
                                     com.example.driver.Gadget. Its plist carries
                                     _BootKCID, and its DYLD_CHAINED_PTR_64_KERNEL_CACHE
                                     chain holds two cache-level-0 slots (kernel_add,
                                     authenticated, and kernel_log) and one level-3
                                     slot of its own. gadget_start and gadget_log
                                     call through the two level-0 slots; the kext
                                     has no symbol for either kernel function.

Pure Python, no toolchain. Run from this directory: python3 gen_linked.py
"""
import base64
import os
import struct

PAGE = 0x1000
BOOT_BASE = 0xFFFFFE0007004000
AUX_BASE = 0xFFFFFE0017004000
BOOT_UUID = bytes.fromhex("6b756e61626f6f744b43000000000001")
AUX_UUID = bytes.fromhex("6b756e616175784b4300000000000003")

def u32(*v): return struct.pack("<" + "I" * len(v), *v)
def u64(*v): return struct.pack("<" + "Q" * len(v), *v)

def adrp(rd, pc, target):
    delta = (target >> 12) - (pc >> 12)
    return 0x90000000 | ((delta & 3) << 29) | (((delta >> 2) & 0x7FFFF) << 5) | rd

def ldr(rt, rn, off): return 0xF9400000 | ((off // 8) << 10) | (rn << 5) | rt

RET, BLR_X16 = 0xD65F03C0, 0xD63F0200
FRAME_IN, FRAME_SP, FRAME_OUT = 0xA9BF7BFD, 0x910003FD, 0xA8C17BFD
TEXT_FLAGS = 0x80000400
MH_DYLIB_IN_CACHE = 0x80000000
N_SECT_EXT = 0x0F

class Collection:
    """A fileset whose file offsets equal vmaddr - base."""
    def __init__(self, base):
        self.base = base
        self.blobs = {}

    def place(self, off, blob): self.blobs[off] = blob

    def section(self, seg, name, addr, size, flags=0, r1=0, r2=0):
        return (name.ljust(16, b"\0") + seg.ljust(16, b"\0") + u64(self.base + addr, size)
                + u32(addr, 0, 0, 0, flags, r1, r2, 0))

    def segment(self, name, addr, size, sections=(), prot=5):
        body = name.ljust(16, b"\0") + u64(self.base + addr, size, addr, size)
        body += u32(prot, prot, len(sections), 0) + b"".join(sections)
        return u32(0x19, 8 + len(body)) + body

    def fileset_entry(self, addr, name):
        body = u64(self.base + addr, addr) + u32(32, 0) + name + b"\0"
        body = body.ljust((len(body) + 8 + 7) & ~7, b"\0")
        return u32(0x80000035, 8 + len(body)) + body

    def write(self, path):
        image = bytearray(max(off + len(blob) for off, blob in self.blobs.items()))
        for off, blob in self.blobs.items():
            image[off:off + len(blob)] = blob
        open(path, "wb").write(image)

def header(filetype, cmds, flags=0):
    return u32(0xFEEDFACF, 0x0100000C, 0, filetype, len(cmds), sum(map(len, cmds)), flags, 0)

def strings(names):
    pool, at = b"\0", {}
    for n in names:
        at[n] = len(pool)
        pool += n + b"\0"
    return pool.ljust((len(pool) + 7) & ~7, b"\0"), at

def nlist(strx, sect, value): return u32(strx) + bytes([N_SECT_EXT, sect]) + struct.pack("<H", 0) + u64(value)
def symtab(symoff, nsyms, stroff, strsize): return u32(0x2, 24, symoff, nsyms, stroff, strsize)
def dysymtab(ndef, indoff=0, nind=0):
    return u32(0xB, 80, 0, 0, 0, ndef, ndef, 0, 0, 0, 0, 0, 0, 0, indoff, nind, 0, 0, 0, 0)
def uuid_cmd(uuid): return u32(0x1B, 24) + uuid

def plist(keys):
    body = b"".join(b"\t<key>%s</key>\n\t<data>%s</data>\n" % (k, base64.b64encode(v)) for k, v in keys)
    return (b'<?xml version="1.0" encoding="UTF-8"?>\n<plist version="1.0">\n<dict>\n'
            b'\t<key>_PrelinkInfoDictionary</key>\n\t<array/>\n' + body + b'</dict>\n</plist>\n\0')

os.makedirs("linked", exist_ok=True)

# The boot collection.
KERNEL_HDR, KERNEL_TEXT, LINKEDIT, PRELINK = 0x1000, 0x3000, 0x4000, 0x5000
kernel_add, kernel_log = KERNEL_TEXT, KERNEL_TEXT + 8
boot = Collection(BOOT_BASE)
pool, at = strings([b"_kernel_add", b"_kernel_log"])
syms = nlist(at[b"_kernel_add"], 2, BOOT_BASE + kernel_add) + nlist(at[b"_kernel_log"], 2, BOOT_BASE + kernel_log)
linkedit = syms + pool
kernel_code = [0x0B010000, RET, 0x4B010000, RET]        # w0 + w1; w0 - w1
kernel_cmds = [
    boot.segment(b"__TEXT", KERNEL_HDR, PAGE, prot=1),
    boot.segment(b"__TEXT_EXEC", KERNEL_TEXT, 0x800,
                 [boot.section(b"__TEXT_EXEC", b"__text", KERNEL_TEXT, 4 * len(kernel_code), TEXT_FLAGS)]),
    boot.segment(b"__LINKEDIT", LINKEDIT, len(linkedit), prot=1),
    symtab(LINKEDIT, 2, LINKEDIT + len(syms), len(pool)),
    dysymtab(2),
]
boot_plist = plist([(b"_PrelinkKCID", BOOT_UUID)])
boot_cmds = [
    boot.segment(b"__TEXT", 0, KERNEL_TEXT, prot=1),
    boot.segment(b"__TEXT_EXEC", KERNEL_TEXT, PAGE),
    boot.segment(b"__LINKEDIT", LINKEDIT, len(linkedit), prot=1),
    boot.segment(b"__PRELINK_INFO", PRELINK, len(boot_plist), prot=1),
    uuid_cmd(BOOT_UUID),
    boot.fileset_entry(KERNEL_HDR, b"com.apple.kernel"),
]
boot.place(0, header(0xC, boot_cmds) + b"".join(boot_cmds))
boot.place(KERNEL_HDR, header(0x2, kernel_cmds, MH_DYLIB_IN_CACHE | 0x1) + b"".join(kernel_cmds))
boot.place(KERNEL_TEXT, u32(*kernel_code))
boot.place(LINKEDIT, linkedit)
boot.place(PRELINK, boot_plist)
boot.write("linked/BootKernelExtensions.kc")

# The Auxiliary collection.
KEXT_HDR, KEXT_CSTRING, KEXT_TEXT = 0x1000, 0x1400, 0x2000
KEXT_GOT, KEXT_CONST, LINKEDIT, PRELINK = 0x3000, 0x3010, 0x4000, 0x5000
aux = Collection(AUX_BASE)
A = AUX_BASE
gadget_start, gadget_log, gadget_name = (KEXT_TEXT + o for o in (0, 0x20, 0x40))

def through_got(pc, slot):
    return [adrp(16, A + pc, A + slot), ldr(16, 16, slot % PAGE), BLR_X16]

code = {
    gadget_start: [FRAME_IN, FRAME_SP, 0x528000E1] + through_got(gadget_start + 12, KEXT_GOT) + [FRAME_OUT, RET],
    gadget_log: [FRAME_IN, FRAME_SP, 0x52800061] + through_got(gadget_log + 12, KEXT_GOT + 8) + [FRAME_OUT, RET],
    gadget_name: [adrp(0, A + gadget_name, A + KEXT_CONST), ldr(0, 0, KEXT_CONST % PAGE), RET],
}
text_len = gadget_name + 12 - KEXT_TEXT

def kc_ptr(target, level, nxt, auth=False, diversity=0):
    return target | (level << 30) | (diversity << 32) | (nxt << 51) | (int(auth) << 63)

slots = {KEXT_GOT: kc_ptr(kernel_add, 0, 2, auth=True, diversity=0x8F1C),
         KEXT_GOT + 8: kc_ptr(kernel_log, 0, 2),
         KEXT_CONST: kc_ptr(KEXT_CSTRING, 3, 0)}

starts_seg = u32(24) + struct.pack("<HH", PAGE, 8) + u64(KEXT_GOT) + u32(0) + struct.pack("<HH", 1, 0)
seg_count = 4
starts_image = u32(seg_count, 0, 0, 4 + 4 * seg_count, 0) + starts_seg
fixups = u32(0, 32, 32 + len(starts_image), 32 + len(starts_image), 0, 1, 0, 0) + starts_image
fixups = fixups.ljust((len(fixups) + 7) & ~7, b"\0")

names = [b"_gadget_start", b"_gadget_log", b"_gadget_name"]
pool, at = strings(names)
syms = b"".join(nlist(at[n], 2, A + addr)
                for n, addr in zip(names, (gadget_start, gadget_log, gadget_name)))
INDIRECT_SYMBOL_LOCAL = 0x80000000
indirect = u32(INDIRECT_SYMBOL_LOCAL) * 2
symoff = LINKEDIT + len(fixups)
indoff = symoff + len(syms)
stroff = indoff + len(indirect)
linkedit = fixups + syms + indirect + pool

kext_cmds = [
    aux.segment(b"__TEXT", KEXT_HDR, PAGE,
                [aux.section(b"__TEXT", b"__cstring", KEXT_CSTRING, 7, 0x2)], prot=1),
    aux.segment(b"__TEXT_EXEC", KEXT_TEXT, 0x800,
                [aux.section(b"__TEXT_EXEC", b"__text", KEXT_TEXT, text_len, TEXT_FLAGS)]),
    aux.segment(b"__DATA_CONST", KEXT_GOT, 0x800,
                [aux.section(b"__DATA_CONST", b"__got", KEXT_GOT, 16, 0x6, 0),
                 aux.section(b"__DATA_CONST", b"__const", KEXT_CONST, 8)], prot=1),
    aux.segment(b"__LINKEDIT", LINKEDIT, len(linkedit), prot=1),
    symtab(symoff, len(names), stroff, len(pool)),
    dysymtab(len(names), indoff, 2),
]
aux_plist = plist([(b"_PrelinkKCID", AUX_UUID), (b"_BootKCID", BOOT_UUID)])
aux_cmds = [
    aux.segment(b"__TEXT", 0, KEXT_TEXT, prot=1),
    aux.segment(b"__TEXT_EXEC", KEXT_TEXT, PAGE),
    aux.segment(b"__DATA_CONST", KEXT_GOT, PAGE, prot=1),
    aux.segment(b"__LINKEDIT", LINKEDIT, len(linkedit), prot=1),
    aux.segment(b"__PRELINK_INFO", PRELINK, len(aux_plist), prot=1),
    uuid_cmd(AUX_UUID),
    u32(0x80000034, 16, LINKEDIT, len(fixups)),
    aux.fileset_entry(KEXT_HDR, b"com.example.driver.Gadget"),
]
aux.place(0, header(0xC, aux_cmds) + b"".join(aux_cmds))
aux.place(KEXT_HDR, header(0xB, kext_cmds, MH_DYLIB_IN_CACHE | 0x85) + b"".join(kext_cmds))
aux.place(KEXT_CSTRING, b"Gadget\0")
for addr, words in code.items():
    aux.place(addr, u32(*words))
for off, raw in slots.items():
    aux.place(off, u64(raw))
aux.place(LINKEDIT, linkedit)
aux.place(PRELINK, aux_plist)
aux.write("linked/AuxiliaryKernelExtensions.kc")
