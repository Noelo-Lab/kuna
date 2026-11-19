#!/usr/bin/env python3
"""Build the pre-fileset (prelinked) kernelcache fixtures.

prelinked_merged.macho  an iOS 12-14 style arm64 kernel: one MH_EXECUTE whose
                        __TEXT_EXEC holds the kernel's code and the header and
                        code of one merged kext (com.example.driver.Widget),
                        found through __PRELINK_INFO,__kmod_start (ModuleIndex 0
                        in the __info plist). Every pointer is threaded through
                        __TEXT,__thread_starts: a plain one to kernel_add, an
                        authenticated one to the kext's widget_get, the four
                        handlers of an IOExternalMethodDispatch table, and the
                        __kmod_info / __kmod_start entries. No symbols.
prelinked_split.macho   an iOS 11 style kernel: the kext is a whole Mach-O in
                        __PRELINK_TEXT, its code in __PLK_TEXT_EXEC and its
                        __got in __PLK_DATA_CONST, its segment offsets relative
                        to its own header (one deliberately wrong, as kxld
                        leaves them), __got and __stubs untyped and no symbols.
                        The kernel exports kernel_add; the plist has no <plist>
                        wrapper, as in these caches.
chainstarts.macho       a static arm64e MH_EXECUTE with __TEXT,__chain_starts
                        (DYLD_CHAINED_PTR_ARM64E_KERNEL, one chain: an
                        authenticated and a plain pointer), as SPTM / TXM ship.

Run from this directory: python3 gen_prelinked.py
"""
import struct

BASE = 0xFFFFFFF007004000
PAGE = 0x1000

def u32(*v): return struct.pack("<" + "I" * len(v), *v)
def u64(*v): return struct.pack("<" + "Q" * len(v), *v)

def adrp(rd, pc, target):
    delta = ((BASE + target) >> 12) - ((BASE + pc) >> 12)
    return 0x90000000 | ((delta & 3) << 29) | (((delta >> 2) & 0x7FFFF) << 5) | rd
def add_lo(rd, rn, target): return 0x91000000 | ((target & 0xFFF) << 10) | (rn << 5) | rd
def ldr(rt, rn, target): return 0xF9400000 | (((target & 0xFFF) // 8) << 10) | (rn << 5) | rt
def bl(pc, target): return 0x94000000 | (((target - pc) // 4) & 0x3FFFFFF)

RET, BR_X8, BR_X16, BLR_X8 = 0xD65F03C0, 0xD61F0100, 0xD61F0200, 0xD63F0100
FRAME, UNFRAME, MOVFP = 0xA9BF7BFD, 0xA8C17BFD, 0x910003FD
TEXT_FLAGS = 0x80000400

def section(seg, name, addr, size, offset, flags=0, r1=0, r2=0):
    return (name.ljust(16, b"\0") + seg.ljust(16, b"\0") + u64(BASE + addr, size)
            + u32(offset, 0, 0, 0, flags, r1, r2, 0))

def segment(name, addr, size, sections=(), prot=5, fileoff=None, flags=0):
    fileoff = addr if fileoff is None else fileoff
    body = name.ljust(16, b"\0") + u64(BASE + addr, size, fileoff, size)
    body += u32(prot, prot, len(sections), flags) + b"".join(sections)
    return u32(0x19, 8 + len(body)) + body

def header(filetype, cmds, cpusub=0):
    return u32(0xFEEDFACF, 0x0100000C, cpusub, filetype, len(cmds), sum(map(len, cmds)), 0x1, 0)

def symtab(symoff=0, nsyms=0, stroff=0, strsize=0): return u32(0x2, 24, symoff, nsyms, stroff, strsize)
def dysymtab(nextdef=0): return u32(0xB, 80, 0, 0, 0, nextdef, nextdef, 0, *([0] * 12))
def uuid(n): return u32(0x1B, 24) + bytes([n]) * 16

def threaded_plain(target, nxt):
    target += BASE
    return (target & 0x7FF_FFFF_FFFF) | (((target >> 56) & 0xFF) << 43) | (nxt << 51)
def threaded_auth(target, nxt, diversity=0x1234):
    return (target & 0xFFFFFFFF) | (diversity << 32) | (nxt << 51) | (1 << 63)

def image_of(size, parts):
    image = bytearray(size)
    for off, blob in parts:
        image[off:off + len(blob)] = blob
    return image

def plist(kexts):
    body = "".join(f"<dict>{k}</dict>" for k in kexts)
    return f"<dict><key>_PrelinkInfoDictionary</key><array>{body}</array></dict>".encode() + b"\0"

# --- prelinked_merged.macho -------------------------------------------------
TEXT_EXEC, KLD, CONST, PRELINK, LINKEDIT = 0x2000, 0x3000, 0x4000, 0x5000, 0x6000
CSTRING, STARTS = 0x1000, 0x1800
kernel_add = TEXT_EXEC
kernel_call = TEXT_EXEC + 8
KEXT = TEXT_EXEC + 0x800
KEXT_TEXT = KEXT + 0x100
widget_get = KEXT_TEXT
widget_call = KEXT_TEXT + 12
SLOT_KERNEL, SLOT_KEXT, SLOT_NAME = CONST, CONST + 8, CONST + 16
KMOD_INFO, KMOD_START, KMOD, INFO = PRELINK, PRELINK + 8, PRELINK + 0x100, PRELINK + 0x200

kernel_code = [0x0B010000, RET,
               FRAME, MOVFP, adrp(8, kernel_call + 8, SLOT_KERNEL), ldr(8, 8, SLOT_KERNEL), BLR_X8, UNFRAME, RET]
DISPATCH = CONST + 0x40
ext = [KEXT_TEXT + 24 + 8 * k for k in range(4)]
kext_code = [adrp(0, widget_get, SLOT_NAME), ldr(0, 0, SLOT_NAME), RET,
             adrp(8, widget_call, SLOT_KEXT), ldr(8, 8, SLOT_KEXT), BR_X8]
for k in range(4):
    kext_code += [0x52800000 | ((0x10 + k) << 5), RET]
kext_cmds = [segment(b"__TEXT_EXEC", KEXT, 0x800, [section(b"__TEXT_EXEC", b"__text", KEXT_TEXT, 4 * len(kext_code), 0x100)],
                     fileoff=0), uuid(2)]
kmod = u64(0) + u32(1, 0) + b"com.example.driver.Widget".ljust(64, b"\0") + b"1.0".ljust(64, b"\0")
kmod = kmod.ljust(0xC8, b"\0")
info = plist(["<key>CFBundleIdentifier</key><string>com.example.driver.Widget</string>"
              "<key>CFBundleVersion</key><string>1.0</string>"
              "<key>ModuleIndex</key><integer size=\"32\">0x0</integer>"
              "<key>IOKitPersonalities</key><dict><key>Widget</key><dict><key>IOClass</key>"
              "<string>WidgetDriver</string></dict></dict>"])
starts = u32(0, CONST, KMOD_INFO, 0xFFFFFFFF)
cmds = [
    segment(b"__TEXT", 0, TEXT_EXEC, [section(b"__TEXT", b"__cstring", CSTRING, 7, CSTRING, 0x2),
                                      section(b"__TEXT", b"__thread_starts", STARTS, len(starts), STARTS)]),
    segment(b"__TEXT_EXEC", TEXT_EXEC, PAGE, [section(b"__TEXT_EXEC", b"__text", TEXT_EXEC, PAGE, TEXT_EXEC, TEXT_FLAGS)]),
    segment(b"__KLD", KLD, PAGE, [section(b"__KLD", b"__text", KLD, 4, KLD, TEXT_FLAGS)]),
    segment(b"__DATA_CONST", CONST, PAGE, [section(b"__DATA_CONST", b"__const", CONST, 0xA0, CONST)], prot=1, flags=0x10),
    segment(b"__PRELINK_INFO", PRELINK, PAGE, [section(b"__PRELINK_INFO", b"__kmod_info", KMOD_INFO, 8, KMOD_INFO),
                                              section(b"__PRELINK_INFO", b"__kmod_start", KMOD_START, 8, KMOD_START),
                                              section(b"__PRELINK_INFO", b"__info", INFO, len(info), INFO)], prot=3),
    segment(b"__LINKEDIT", LINKEDIT, 0, prot=1),
    symtab(), dysymtab(), uuid(1),
]
image = image_of(LINKEDIT, [
    (0, header(0x2, cmds, 0x2) + b"".join(cmds)),
    (CSTRING, b"Widget\0"), (STARTS, starts),
    (TEXT_EXEC, u32(*kernel_code)), (KEXT, header(0xB, kext_cmds, 0x2) + b"".join(kext_cmds)),
    (KEXT_TEXT, u32(*kext_code)), (KLD, u32(RET)),
    (SLOT_KERNEL, u64(threaded_plain(kernel_add, 2))),
    (SLOT_KEXT, u64(threaded_auth(widget_get, 2))),
    (SLOT_NAME, u64(threaded_plain(CSTRING, (DISPATCH - SLOT_NAME) // 4))),
    *[(DISPATCH + 0x18 * k, u64(threaded_auth(f, 6) if k % 2 else threaded_plain(f, 6 if k < 3 else 0)) + u32(*counts))
      for k, (f, counts) in enumerate(zip(ext, [(1, 0, 0, 0), (0, 0xFFFFFFFF, 0, 0), (2, 0, 1, 0), (0, 0x10, 0, 0x10)]))],
    (KMOD_INFO, u64(threaded_plain(KMOD, 2))), (KMOD_START, u64(threaded_plain(KEXT, 0))),
    (KMOD, kmod), (INFO, info),
])
open("prelinked_merged.macho", "wb").write(image)

# --- prelinked_split.macho --------------------------------------------------
TEXT_EXEC, KLD, CONST = 0x2000, 0x3000, 0x4000
PRELINK_TEXT, PLK_EXEC, PLK_CONST, PRELINK_INFO, LINKEDIT = 0x8000, 0x9000, 0xA000, 0xB000, 0xC000
kernel_add = TEXT_EXEC
KHDR, KCSTRING = PRELINK_TEXT, PRELINK_TEXT + 0x800
KTEXT, KSTUBS = PLK_EXEC, PLK_EXEC + 0x100
KGOT, KCONST = PLK_CONST, PLK_CONST + 8
widget_via_stub = KTEXT
kext_code = [FRAME, MOVFP, 0x52800101, bl(KTEXT + 12, KSTUBS), UNFRAME, RET]
stub_code = [adrp(16, KSTUBS, KGOT), ldr(16, 16, KGOT), BR_X16]

def rel(off): return off - KHDR
kext_cmds = [
    segment(b"__TEXT", KHDR, PAGE, [section(b"__TEXT", b"__cstring", KCSTRING, 7, rel(KCSTRING), 0x2)], fileoff=0),
    segment(b"__TEXT_EXEC", PLK_EXEC, PAGE, [section(b"__TEXT_EXEC", b"__text", KTEXT, 4 * len(kext_code), rel(KTEXT), TEXT_FLAGS),
                                             section(b"__TEXT_EXEC", b"__stubs", KSTUBS, 12, rel(KSTUBS), TEXT_FLAGS)],
            fileoff=rel(PLK_EXEC)),
    segment(b"__DATA_CONST", PLK_CONST, PAGE, [section(b"__DATA_CONST", b"__got", KGOT, 8, 0x77000),
                                               section(b"__DATA_CONST", b"__const", KCONST, 8, 0x77008)],
            prot=1, fileoff=0x77000),
    segment(b"__LINKEDIT", LINKEDIT + 0x1000, 0, prot=1, fileoff=0x99000),
    symtab(), dysymtab(), uuid(3),
]
strtab = b"\0_kernel_add\0_kernel_table\0".ljust(32, b"\0")
syms = (u32(1) + bytes([0x0F, 2]) + struct.pack("<H", 0) + u64(BASE + kernel_add)
        + u32(13) + bytes([0x0F, 3]) + struct.pack("<H", 0) + u64(BASE + CONST))
info = plist(["<key>CFBundleIdentifier</key><string>com.apple.kpi.mach</string>"
              f"<key>_PrelinkExecutableLoadAddr</key><integer size=\"64\">{hex(BASE + PRELINK_TEXT + 0xF00)}</integer>",
              "<key>CFBundleIdentifier</key><string>com.example.driver.Widget</string>"
              "<key>CFBundleVersion</key><string>1.0</string>"
              f"<key>_PrelinkExecutableLoadAddr</key><integer size=\"64\">{hex(BASE + KHDR)}</integer>"])
cmds = [
    segment(b"__TEXT", 0, TEXT_EXEC, prot=5),
    segment(b"__TEXT_EXEC", TEXT_EXEC, PAGE, [section(b"__TEXT_EXEC", b"__text", TEXT_EXEC, 8, TEXT_EXEC, TEXT_FLAGS)]),
    segment(b"__KLD", KLD, PAGE, [section(b"__KLD", b"__text", KLD, 4, KLD, TEXT_FLAGS)]),
    segment(b"__DATA_CONST", CONST, PAGE, [section(b"__DATA_CONST", b"__const", CONST, 8, CONST)], prot=3),
    segment(b"__PRELINK_TEXT", PRELINK_TEXT, PAGE, [section(b"__PRELINK_TEXT", b"__text", PRELINK_TEXT, PAGE, PRELINK_TEXT)], prot=1),
    segment(b"__PLK_TEXT_EXEC", PLK_EXEC, PAGE, [section(b"__PLK_TEXT_EXEC", b"__text", PLK_EXEC, PAGE, PLK_EXEC)]),
    segment(b"__PLK_DATA_CONST", PLK_CONST, PAGE, [section(b"__PLK_DATA_CONST", b"__data", PLK_CONST, PAGE, PLK_CONST)], prot=1),
    segment(b"__PRELINK_INFO", PRELINK_INFO, PAGE, [section(b"__PRELINK_INFO", b"__info", PRELINK_INFO, len(info), PRELINK_INFO)], prot=3),
    segment(b"__LINKEDIT", LINKEDIT, len(syms) + len(strtab), prot=1),
    symtab(LINKEDIT, 2, LINKEDIT + len(syms), len(strtab)), dysymtab(2), uuid(1),
]
image = image_of(LINKEDIT + len(syms) + len(strtab), [
    (0, header(0x2, cmds) + b"".join(cmds)),
    (TEXT_EXEC, u32(0x0B010000, RET)), (KLD, u32(RET)), (CONST, u64(BASE + kernel_add)),
    (KHDR, header(0xB, kext_cmds) + b"".join(kext_cmds)), (KCSTRING, b"Widget\0"),
    (KTEXT, u32(*kext_code)), (KSTUBS, u32(*stub_code)),
    (KGOT, u64(BASE + kernel_add)), (KCONST, u64(BASE + KCSTRING)),
    (PRELINK_INFO, info), (LINKEDIT, syms + strtab),
])
open("prelinked_split.macho", "wb").write(image)

# --- chainstarts.macho ------------------------------------------------------
TEXT_EXEC, CONST, LINKEDIT = 0x2000, 0x3000, 0x4000
STARTS = 0x1000
get_auth, target = TEXT_EXEC, TEXT_EXEC + 32
code = [FRAME, MOVFP, adrp(8, get_auth + 8, CONST), ldr(8, 8, CONST), BLR_X8, UNFRAME, RET, 0,
        0x0B010000, RET]
starts = u32(7, 1, CONST)
def kernel_auth(off, nxt): return (off & 0xFFFFFFFF) | (0x5678 << 32) | (nxt << 51) | (1 << 63)
def kernel_plain(off, nxt): return (off & 0x7FF_FFFF_FFFF) | (nxt << 51)
cmds = [
    segment(b"__TEXT", 0, TEXT_EXEC, [section(b"__TEXT", b"__chain_starts", STARTS, len(starts), STARTS)]),
    segment(b"__TEXT_EXEC", TEXT_EXEC, PAGE, [section(b"__TEXT_EXEC", b"__text", TEXT_EXEC, 4 * len(code), TEXT_EXEC, TEXT_FLAGS)]),
    segment(b"__DATA_CONST", CONST, PAGE, [section(b"__DATA_CONST", b"__const", CONST, 16, CONST)], prot=1, flags=0x10),
    segment(b"__LINKEDIT", LINKEDIT, 0, prot=1),
    symtab(), dysymtab(),
]
image = image_of(LINKEDIT, [
    (0, header(0x2, cmds, 0x80000002) + b"".join(cmds)), (STARTS, starts), (TEXT_EXEC, u32(*code)),
    (CONST, u64(kernel_auth(target, 2), kernel_plain(target, 0))),
])
open("chainstarts.macho", "wb").write(image)
