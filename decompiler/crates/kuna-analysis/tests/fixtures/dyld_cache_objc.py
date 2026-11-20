#!/usr/bin/env python3
"""Generate `dyld_cache_objc/` and `dyld_cache_objc_v16/` -- a two-image arm64e
dyld shared cache whose `libgreeter.dylib` carries Objective-C metadata the way
a cache lays it out: its selector and type strings live in libobjc's `__OBJC_RO`
region, outside every segment of libgreeter; its method list is small with
`0x40000000` (selectors are offsets from the cache's selector base),
`0x20000000` (so are the type strings) and the two low flag bits the cache
builder sets (uniqued, sorted); and `class_ro_t.baseMethods` has its low bit
set, a relative list-of-lists (the preattached-category shape) naming that list
and a second one that lives in libobjc with its IMP there. An `__objc_stubs`
entry loads a selector reference whose string is in libobjc too. No Apple
toolchain is needed; the files are assembled byte by byte from the public
`dyld_cache_format.h` / objc4 layouts. Regenerate with:

    python3 dyld_cache_objc.py

Layout (cache base 0x180000000), one main file, no sub-caches:

  0x180000000  __TEXT  file 0x0000   cache header + tables
    0x180001000  libgreeter.dylib  __TEXT (Mach-O header; __text @+0x800)
      0x180001800  -[Greeter greet:](self, _cmd, n) = n * 3 + 7
      0x180001810  -[Greeter count](self, _cmd)     = 42
      0x180001820  b objc_msgSend$count        (the stub below)
      0x180001830  b -[Greeter shout]          (the IMP in libobjc)
    0x180001900  __objc_methlist: the small method list (2 entries)
    0x180001A00  __objc_stubs: objc_msgSend$count (adrp/ldr x1 from the selref, braa)
    0x180002000  libobjc.A.dylib   __TEXT
      0x180002800  objc_msgSend
      0x180002810  -[Greeter shout], a category method preattached from this image
  0x180010000  __DATA_CONST  file 0x4000  libgreeter __objc_classlist / __objc_data / __objc_const
    0x180010000  __objc_classlist -> class_t @0x180010100
    0x180010080  __objc_selrefs: one slot -> "count" in libobjc's __OBJC_RO
    0x180010100  class_t (isa -> metaclass @0x180010140, data -> class_ro_t @0x180010200)
    0x180010200  class_ro_t: name "Greeter", baseMethods = list-of-lists @0x180010300 | 1
    0x180010300  relative_list_list_t: entsize 8, count 2, entries -> __objc_methlist
                 and the list @0x180030400
    0x180010400  __auth_got: one slot -> objc_msgSend
  0x180030000  __OBJC_RO  file 0x5000  libobjc: __objc_opt_ro (objc_opt_t v16 @0x180030000),
                                       __objc_methname (selector base 0x180030100), type strings
    0x180030400                         the preattached list: direct selectors, self-relative types
  0x180030800                           ObjCOptimizationHeader (the dyld-1042+ cache header
                                       names it; the `_v16` variant omits it so the
                                       objc_opt_t fallback is what finds the base)
  0x180020000  __LINKEDIT  file 0x6000  an empty string pool (both images are stripped)
"""
import os
import struct

HERE = os.path.dirname(os.path.abspath(__file__))
NAME = "dyld_shared_cache_arm64e"

BASE = 0x180000000
GREETER, OBJC = 0x180001000, 0x180002000
GREETER_TEXT, METHLIST, STUBS = GREETER + 0x800, GREETER + 0x900, GREETER + 0xA00
IMP_GREET, IMP_COUNT = GREETER_TEXT, GREETER_TEXT + 0x10
OBJC_TEXT = OBJC + 0x800
MSGSEND, IMP_SHOUT = OBJC_TEXT, OBJC_TEXT + 0x10
DATA = 0x180010000
CLASSLIST, CLASS, META, CLASS_RO, META_RO, LISTLIST, CLASSNAME = (
    DATA, DATA + 0x100, DATA + 0x140, DATA + 0x200, DATA + 0x260, DATA + 0x300, DATA + 0x380)
SELREFS, GOT = DATA + 0x80, DATA + 0x400
OBJC_RO = 0x180030000
OPT_T, SELBASE, CATLIST, OPTS_HDR = OBJC_RO, OBJC_RO + 0x100, OBJC_RO + 0x400, OBJC_RO + 0x800
LINKEDIT = 0x180020000
DATA_FILE, RO_FILE, LINKEDIT_FILE, MAIN_SIZE = 0x4000, 0x5000, 0x6000, 0x7000
UUID_MAIN = bytes(range(0x40, 0x50))

MH_MAGIC_64, CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64E = 0xFEEDFACF, 0x0100000C, 2
MH_DYLIB, MH_FLAGS = 6, 0x80000000 | 0x80 | 0x4
LC_SEGMENT_64, LC_SYMTAB, LC_DYSYMTAB, LC_ID_DYLIB = 0x19, 0x2, 0xB, 0xD
S_ATTR_CODE = 0x80000400

SELECTORS = ["greet:", "count"]
TYPES = ["i24@0:8i16", "i16@0:8"]
CAT_SELECTOR, CAT_TYPES = "shout", "v16@0:8"


def words(ws):
    return b"".join(struct.pack("<I", w) for w in ws)


def greeter_code():
    greet = [0x0B020440, 0x11001C00, 0xD65F03C0, 0xD503201F]   # add w0,w2,w2,lsl#1; add w0,w0,#7; ret; nop
    count = [0x52800540, 0xD65F03C0, 0xD503201F, 0xD503201F]   # mov w0,#42; ret; nop; nop
    nop = 0xD503201F
    send = [branch(GREETER_TEXT + 0x20, STUBS), nop, nop, nop]
    shout = [branch(GREETER_TEXT + 0x30, IMP_SHOUT), nop, nop, nop]
    return words(greet + count + send + shout)


def branch(pc, target):
    return 0x14000000 | (((target - pc) >> 2) & 0x3FFFFFF)


def adrp(pc, reg, target):
    pages = ((target >> 12) - (pc >> 12)) & 0x1FFFFF
    return 0x90000000 | ((pages & 3) << 29) | ((pages >> 2) << 5) | reg


def stub_code():
    """The arm64e `objc_msgSend$count` stub: the selector from its selref into
    x1, then an authenticated jump through the `objc_msgSend` slot."""
    return words([
        adrp(STUBS, 1, SELREFS),
        0xF9400021 | (((SELREFS & 0xFFF) // 8) << 10),          # ldr x1, [x1, #selref]
        adrp(STUBS + 8, 17, GOT),
        0x91000231 | ((GOT & 0xFFF) << 10),                     # add x17, x17, #got
        0xF9400230,                                             # ldr x16, [x17]
        0xD71F0A11,                                             # braa x16, x17
        0xD4200020, 0xD4200020,                                 # brk #1
    ])


def objc_code():
    ret, nop = 0xD65F03C0, 0xD503201F
    return words([ret, nop, nop, nop, ret, nop, nop, nop])


def strings_region():
    """`__OBJC_RO` after the objc_opt_t: the selector base, then the strings."""
    pool = bytearray()
    offsets = {}
    for s in SELECTORS + TYPES + [CAT_SELECTOR, CAT_TYPES]:
        offsets[s] = len(pool)
        pool += s.encode() + b"\0"
    return bytes(pool), offsets


def method_list(offsets):
    """Small, direct selectors, relative types, uniqued and sorted: header then
    3 x i32 per entry."""
    flags = 12 | 0x80000000 | 0x40000000 | 0x20000000 | 3
    out = struct.pack("<II", flags, len(SELECTORS))
    for i, (sel, ty, imp) in enumerate(zip(SELECTORS, TYPES, [IMP_GREET, IMP_COUNT])):
        at = METHLIST + 8 + 12 * i
        out += struct.pack("<iii", offsets[sel], offsets[ty], imp - (at + 8))
    return out


def category_list(offsets):
    """The list preattached from libobjc: direct selector, self-relative types."""
    at = CATLIST + 8
    return struct.pack("<IIiii", 12 | 0x80000000 | 0x40000000 | 3, 1, offsets[CAT_SELECTOR],
                       SELBASE + offsets[CAT_TYPES] - (at + 4), IMP_SHOUT - (at + 8))


def name16(s):
    return s.encode().ljust(16, b"\0")


def segment(name, vmaddr, vmsize, fileoff, filesize, prot, sections):
    cmd = struct.pack("<II16sQQQQIIII", LC_SEGMENT_64, 72 + 80 * len(sections), name16(name),
                      vmaddr, vmsize, fileoff, filesize, prot, prot, len(sections), 0)
    for sect, addr, size, offset, flags in sections:
        cmd += struct.pack("<16s16sQQIIIIIIII", name16(sect), name16(name), addr, size, offset,
                           3, 0, 0, flags, 0, 0, 0)
    return cmd


def dylib(path):
    raw = path.encode() + b"\0"
    raw = raw.ljust((len(raw) + 7) // 8 * 8, b"\0")
    return struct.pack("<IIIIII", LC_ID_DYLIB, 24 + len(raw), 24, 0, 0x10000, 0x10000) + raw


def macho(segments, path, symtab):
    cmds = b"".join(segments) + dylib(path)
    cmds += struct.pack("<IIIIII", LC_SYMTAB, 24, *symtab)
    cmds += struct.pack("<II18I", LC_DYSYMTAB, 80, *([0] * 18))
    header = struct.pack("<IIIIIIII", MH_MAGIC_64, CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64E, MH_DYLIB,
                         len(segments) + 3, len(cmds), MH_FLAGS, 0)
    return header + cmds


def cache_header(mapping_count, header_len, extra):
    h = bytearray(0x200)
    h[0:16] = b"dyld_v1  arm64e\0"
    struct.pack_into("<II", h, 16, header_len, mapping_count)
    h[88:104] = UUID_MAIN
    for off, fmt, *vals in extra:
        struct.pack_into(fmt, h, off, *vals)
    return h


def main_file(with_opts_header):
    f = bytearray(MAIN_SIZE)
    mappings = [  # address, size, file offset, prot
        (BASE, 0x4000, 0, 5),
        (DATA, 0x1000, DATA_FILE, 3),
        (OBJC_RO, 0x1000, RO_FILE, 1),
        (LINKEDIT, 0x1000, LINKEDIT_FILE, 1),
    ]
    paths = ["/usr/lib/libgreeter.dylib", "/usr/lib/libobjc.A.dylib"]
    old_maps, slide_maps, images = 0x200, 0x2A0, 0x3C0
    path_pool = images + 32 * len(paths)
    # A dyld-1042+ header carries `objcOptsOffset`/`objcOptsSize` at 464; the
    # older variant ends its header before them (`mappingOffset` 0x1D0).
    header_len = 0x200 if with_opts_header else 0x1D0
    extra = [
        (312, "<II", slide_maps, len(mappings)),
        (448, "<II", images, len(paths)),
    ]
    if with_opts_header:
        extra.append((464, "<QQ", OPTS_HDR - BASE, 0x38))
    f[:0x200] = cache_header(len(mappings), header_len, extra)
    for i, (addr, size, off, prot) in enumerate(mappings):
        struct.pack_into("<QQQII", f, old_maps + i * 32, addr, size, off, prot, prot)
        struct.pack_into("<QQQQQQII", f, slide_maps + i * 56, addr, size, off, 0, 0, 0, prot, prot)
    at = path_pool
    for i, (path, addr) in enumerate(zip(paths, [GREETER, OBJC])):
        struct.pack_into("<QQQII", f, images + i * 32, addr, 0, 0, at, 0)
        f[at:at + len(path) + 1] = path.encode() + b"\0"
        at += len(path) + 1

    pool, offsets = strings_region()
    strings = 0x100
    greeter = macho([
        segment("__TEXT", GREETER, 0x1000, GREETER - BASE, 0x1000, 5, [
            ("__text", GREETER_TEXT, len(greeter_code()), GREETER_TEXT - BASE, S_ATTR_CODE),
            ("__objc_methlist", METHLIST, len(method_list(offsets)), METHLIST - BASE, 0),
            ("__objc_stubs", STUBS, len(stub_code()), STUBS - BASE, S_ATTR_CODE),
        ]),
        segment("__DATA_CONST", DATA, 0x1000, DATA_FILE, 0x1000, 3, [
            ("__objc_classlist", CLASSLIST, 8, DATA_FILE, 0),
            ("__objc_selrefs", SELREFS, 8, DATA_FILE + 0x80, 0),
            ("__objc_data", CLASS, 0x80, DATA_FILE + 0x100, 0),
            ("__objc_const", CLASS_RO, 0x200, DATA_FILE + 0x200, 0),
            ("__auth_got", GOT, 8, DATA_FILE + 0x400, 0),
        ]),
        segment("__LINKEDIT", LINKEDIT, 0x1000, LINKEDIT_FILE, 0x1000, 1, []),
    ], paths[0], (LINKEDIT_FILE, 0, LINKEDIT_FILE + strings, 32))
    objc = macho([
        segment("__TEXT", OBJC, 0x1000, OBJC - BASE, 0x1000, 5, [
            ("__text", OBJC_TEXT, len(objc_code()), OBJC_TEXT - BASE, S_ATTR_CODE),
        ]),
        segment("__OBJC_RO", OBJC_RO, 0x1000, RO_FILE, 0x1000, 1, [
            ("__objc_opt_ro", OPT_T, 0x30, RO_FILE, 0),
            ("__objc_methname", SELBASE, len(pool), RO_FILE + 0x100, 2),
        ]),
        segment("__LINKEDIT", LINKEDIT, 0x1000, LINKEDIT_FILE, 0x1000, 1, []),
    ], paths[1], (LINKEDIT_FILE, 0, LINKEDIT_FILE + strings, 32))
    for addr, blob in [(GREETER, greeter), (OBJC, objc), (GREETER_TEXT, greeter_code()),
                       (METHLIST, method_list(offsets)), (STUBS, stub_code()), (OBJC_TEXT, objc_code())]:
        f[addr - BASE:addr - BASE + len(blob)] = blob

    # libgreeter's metadata in __DATA_CONST (raw, unslid pointers: no slide info).
    struct.pack_into("<Q", f, DATA_FILE, CLASS)
    struct.pack_into("<QQQQQ", f, DATA_FILE + (CLASS - DATA), META, 0, 0, 0, CLASS_RO)
    struct.pack_into("<QQQQQ", f, DATA_FILE + (META - DATA), META, 0, 0, 0, META_RO)
    struct.pack_into("<IIIIQQQQQQQ", f, DATA_FILE + (CLASS_RO - DATA),
                     0, 8, 8, 0, 0, CLASSNAME, LISTLIST | 1, 0, 0, 0, 0)
    struct.pack_into("<IIIIQQQQQQQ", f, DATA_FILE + (META_RO - DATA),
                     1, 40, 40, 0, 0, CLASSNAME, 0, 0, 0, 0, 0)
    struct.pack_into("<IIqq", f, DATA_FILE + (LISTLIST - DATA), 8, 2,
                     ((METHLIST - (LISTLIST + 8)) << 16) | 0, ((CATLIST - (LISTLIST + 16)) << 16) | 1)
    struct.pack_into("<Q", f, DATA_FILE + (SELREFS - DATA), SELBASE + offsets["count"])
    struct.pack_into("<Q", f, DATA_FILE + (GOT - DATA), MSGSEND)
    f[DATA_FILE + (CLASSNAME - DATA):DATA_FILE + (CLASSNAME - DATA) + 8] = b"Greeter\0"

    # libobjc's __OBJC_RO: objc_opt_t v16 (the pre-dyld-1042 carrier of the
    # selector base, relative to the record), then the strings at the base.
    struct.pack_into("<IIiiiiiiiiQ", f, RO_FILE, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, SELBASE - OPT_T)
    f[RO_FILE + 0x100:RO_FILE + 0x100 + len(pool)] = pool
    cat = category_list(offsets)
    f[RO_FILE + (CATLIST - OBJC_RO):RO_FILE + (CATLIST - OBJC_RO) + len(cat)] = cat
    if with_opts_header:
        struct.pack_into("<IIQQQQQQ", f, RO_FILE + (OPTS_HDR - OBJC_RO), 1, 0, 0, 0, 0, 0, 0, SELBASE - BASE)

    # No symbols: the image is stripped, the metadata names everything.
    f[LINKEDIT_FILE + strings:LINKEDIT_FILE + strings + 32] = b" \0".ljust(32, b"\0")
    return f


def build():
    for folder, with_opts in [("dyld_cache_objc", True), ("dyld_cache_objc_v16", False)]:
        out = os.path.join(HERE, folder)
        os.makedirs(out, exist_ok=True)
        with open(os.path.join(out, NAME), "wb") as fh:
            fh.write(main_file(with_opts))


if __name__ == "__main__":
    build()
