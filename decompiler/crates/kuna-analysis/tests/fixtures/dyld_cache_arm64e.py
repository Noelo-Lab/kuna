#!/usr/bin/env python3
"""Generate `dyld_cache_arm64e/` -- a two-image arm64e dyld shared cache, split
the way iOS 16 splits one: a main file, a `.01` sub-cache holding a
`__TEXT_STUBS` stub island, and a `.symbols` file holding the local symbols.

`libcaller.dylib` calls `libcallee.dylib` through the island and loads one of its
globals through a coalesced `__got` whose slots carry v3 slide-info chains (a
plain pointer chained to an authenticated one). Its one local function is a
`<redacted>` placeholder in its own symbol table and is named only by
`.symbols`. No Apple toolchain is needed; the files are assembled here byte by
byte from the public `dyld_cache_format.h` layouts and the A64 bodies are
hand-encoded. Regenerate with:

    python3 dyld_cache_arm64e.py

Layout (cache base 0x180000000):

  main  0x180000000  __TEXT   file 0x0000  cache header + tables
        0x180001000           libcaller.dylib  __TEXT (Mach-O header, __text @+0x800)
            0x180001800  caller_entry(a, b) = callee_add(helper_local(a), b) + callee_counter
            0x180001828  helper_local(a) = a + 1      <- named by .symbols only
            0x180001830  caller_indirect: tail call through __got[1]
        0x180002000           libcallee.dylib  __TEXT (__text @+0x800)
            0x180002800  callee_add(a, b) = a + b
  .01   0x180008000  __TEXT_STUBS  island @0x180008800: adrp/add/br x16 -> callee_add
  main  0x180010000  __DATA   file 0x3000, slide info v3 @ file 0x4000
            0x180010000  libcaller __DATA_CONST,__got: [0] -> callee_counter (plain,
                         chained to [1]), [1] -> callee_add (auth, IA, addr-diversified)
            0x180010100  libcallee __DATA,__data: callee_counter = 5
  main  0x180020000  __LINKEDIT  file 0x5000  both images' nlists + one string pool
"""
import os
import struct

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "dyld_cache_arm64e")
NAME = "dyld_shared_cache_arm64e"

BASE = 0x180000000
CALLER, CALLEE = 0x180001000, 0x180002000
CALLER_TEXT, CALLEE_TEXT = CALLER + 0x800, CALLEE + 0x800
ISLANDS, ISLAND = 0x180008000, 0x180008800
DATA = 0x180010000
GOT, COUNTER = DATA, DATA + 0x100
LINKEDIT = 0x180020000
DATA_FILE, SLIDE_FILE, LINKEDIT_FILE, MAIN_SIZE = 0x3000, 0x4000, 0x5000, 0x6000

UUID_MAIN = bytes(range(0x10, 0x20))
UUID_01 = bytes(range(0x20, 0x30))
UUID_SYMBOLS = bytes(range(0x30, 0x40))

MH_MAGIC_64, CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64E = 0xFEEDFACF, 0x0100000C, 2
MH_DYLIB, MH_FLAGS = 6, 0x80000000 | 0x80 | 0x4
LC_SEGMENT_64, LC_SYMTAB, LC_DYSYMTAB, LC_ID_DYLIB = 0x19, 0x2, 0xB, 0xD
S_ATTR_CODE = 0x80000400
TEXT_STUBS = 1 << 3


def bl(pc, target):
    return 0x94000000 | (((target - pc) >> 2) & 0x3FFFFFF)


def b(pc, target):
    return 0x14000000 | (((target - pc) >> 2) & 0x3FFFFFF)


def adrp(rd, pc, target):
    pages = ((target >> 12) - (pc >> 12)) & 0x1FFFFF
    return 0x90000000 | ((pages & 3) << 29) | ((pages >> 2) << 5) | rd


def add_imm(rd, rn, imm):
    return 0x91000000 | (imm << 10) | (rn << 5) | rd


def ldr_x(rt, rn, off):
    return 0xF9400000 | ((off // 8) << 10) | (rn << 5) | rt


def words(ws):
    return b"".join(struct.pack("<I", w) for w in ws)


def caller_code():
    pc = CALLER_TEXT
    entry = [
        0xA9BF7BFD,                                    # stp x29, x30, [sp, #-16]!
        0x910003FD,                                    # mov x29, sp
        bl(pc + 8, pc + 0x28),                         # bl helper_local
        bl(pc + 12, ISLAND),                           # bl island -> callee_add
        adrp(8, pc + 16, GOT),                         # adrp x8, __got
        ldr_x(8, 8, GOT & 0xFFF),                      # ldr x8, [x8, __got[0]]
        0xB9400109,                                    # ldr w9, [x8]
        0x0B090000,                                    # add w0, w0, w9
        0xA8C17BFD,                                    # ldp x29, x30, [sp], #16
        0xD65F03C0,                                    # ret
    ]
    helper = [0x11000400, 0xD65F03C0]                  # add w0, w0, #1; ret
    indirect = [
        adrp(16, pc + 0x30, GOT),                      # adrp x16, __got
        ldr_x(16, 16, (GOT & 0xFFF) + 8),              # ldr x16, [x16, __got[1]]
        0xD61F0200,                                    # br x16
    ]
    return words(entry + helper + indirect)


def callee_code():
    return words([0x0B010000, 0xD65F03C0])            # add w0, w0, w1; ret


def island():
    return words([
        adrp(16, ISLAND, CALLEE_TEXT),
        add_imm(16, 16, CALLEE_TEXT & 0xFFF),
        0xD61F0200,                                    # br x16
        0xD4200020,                                    # brk #1
    ])


def name16(s):
    return s.encode().ljust(16, b"\0")


def segment(name, vmaddr, vmsize, fileoff, filesize, prot, sections):
    cmd = struct.pack("<II16sQQQQIIII", LC_SEGMENT_64, 72 + 80 * len(sections), name16(name),
                      vmaddr, vmsize, fileoff, filesize, prot, prot, len(sections), 0)
    for sect, addr, size, offset, flags in sections:
        cmd += struct.pack("<16s16sQQIIIIIIII", name16(sect), name16(name), addr, size, offset,
                           2, 0, 0, flags, 0, 0, 0)
    return cmd


def id_dylib(path):
    raw = path.encode() + b"\0"
    raw = raw.ljust((len(raw) + 7) // 8 * 8, b"\0")
    return struct.pack("<IIIIII", LC_ID_DYLIB, 24 + len(raw), 24, 0, 0x10000, 0x10000) + raw


def macho(text_addr, segments, path, symtab, dysymtab):
    cmds = b"".join(segments) + id_dylib(path)
    cmds += struct.pack("<IIIIII", LC_SYMTAB, 24, *symtab)
    cmds += struct.pack("<II18I", LC_DYSYMTAB, 80, *dysymtab, *([0] * 12))
    ncmds = len(segments) + 3
    header = struct.pack("<IIIIIIII", MH_MAGIC_64, CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64E, MH_DYLIB,
                         ncmds, len(cmds), MH_FLAGS, 0)
    return header + cmds


class Pool:
    def __init__(self):
        self.data = b" \0"

    def add(self, s):
        at = len(self.data)
        self.data += s.encode() + b"\0"
        return at


def nlist(strx, n_type, n_sect, value):
    return struct.pack("<IBBHQ", strx, n_type, n_sect, 0, value)


def cache_header(path_uuid, mapping_count, extra):
    h = bytearray(0x200)
    h[0:16] = b"dyld_v1  arm64e\0"
    struct.pack_into("<II", h, 16, 0x200, mapping_count)
    h[88:104] = path_uuid
    for off, fmt, *vals in extra:
        struct.pack_into(fmt, h, off, *vals)
    return h


def main_file(pool, linkedit):
    f = bytearray(MAIN_SIZE)
    mappings = [  # address, size, file offset, slide offset, slide size, flags, prot
        (BASE, 0x3000, 0, 0, 0, 0, 5),
        (DATA, 0x1000, DATA_FILE, SLIDE_FILE, 32, 0, 3),
        (LINKEDIT, 0x1000, LINKEDIT_FILE, 0, 0, 0, 1),
    ]
    paths = ["/usr/lib/libcaller.dylib", "/usr/lib/libcallee.dylib"]
    old_maps, slide_maps, subcache, images = 0x200, 0x260, 0x320, 0x360
    path_pool = images + 32 * len(paths)
    f[:0x200] = cache_header(UUID_MAIN, len(mappings), [
        (312, "<II", slide_maps, len(mappings)),
        (392, "<II", subcache, 1),
        (448, "<II", images, len(paths)),
    ])
    f[400:416] = UUID_SYMBOLS
    for i, (addr, size, off, soff, ssize, flags, prot) in enumerate(mappings):
        struct.pack_into("<QQQII", f, old_maps + i * 32, addr, size, off, prot, prot)
        struct.pack_into("<QQQQQQII", f, slide_maps + i * 56, addr, size, off, soff, ssize, flags, prot, prot)
    struct.pack_into("<16sQ32s", f, subcache, UUID_01, ISLANDS - BASE, b".01")
    at = path_pool
    for i, (path, addr) in enumerate(zip(paths, [CALLER, CALLEE])):
        struct.pack_into("<QQQII", f, images + i * 32, addr, 0, 0, at, 0)
        f[at:at + len(path) + 1] = path.encode() + b"\0"
        at += len(path) + 1

    sym_caller, sym_callee, strings = 0x0, 0x30, 0x60
    caller = macho(CALLER, [
        segment("__TEXT", CALLER, 0x1000, CALLER - BASE, 0x1000, 5,
                [("__text", CALLER_TEXT, len(caller_code()), CALLER_TEXT - BASE, S_ATTR_CODE)]),
        segment("__DATA_CONST", GOT, 0x100, DATA_FILE, 0x100, 3,
                [("__got", GOT, 16, DATA_FILE, 0)]),
        segment("__LINKEDIT", LINKEDIT, 0x1000, LINKEDIT_FILE, 0x1000, 1, []),
    ], paths[0], (LINKEDIT_FILE + sym_caller, 3, LINKEDIT_FILE + strings, len(pool.data)),
        (0, 1, 1, 2, 3, 0))
    callee = macho(CALLEE, [
        segment("__TEXT", CALLEE, 0x1000, CALLEE - BASE, 0x1000, 5,
                [("__text", CALLEE_TEXT, len(callee_code()), CALLEE_TEXT - BASE, S_ATTR_CODE)]),
        segment("__DATA", COUNTER, 0x100, DATA_FILE + 0x100, 0x100, 3,
                [("__data", COUNTER, 4, DATA_FILE + 0x100, 0)]),
        segment("__LINKEDIT", LINKEDIT, 0x1000, LINKEDIT_FILE, 0x1000, 1, []),
    ], paths[1], (LINKEDIT_FILE + sym_callee, 2, LINKEDIT_FILE + strings, len(pool.data)),
        (0, 0, 0, 2, 2, 0))
    for addr, blob in [(CALLER, caller), (CALLEE, callee),
                       (CALLER_TEXT, caller_code()), (CALLEE_TEXT, callee_code())]:
        f[addr - BASE:addr - BASE + len(blob)] = blob

    plain = (1 << 51) | COUNTER
    auth = (1 << 63) | (1 << 48) | (0x1234 << 32) | (CALLEE_TEXT - BASE)
    struct.pack_into("<QQ", f, DATA_FILE, plain, auth)
    struct.pack_into("<I", f, DATA_FILE + 0x100, 5)
    struct.pack_into("<IIIIQH", f, SLIDE_FILE, 3, 0x1000, 1, 0, BASE, 0)

    f[LINKEDIT_FILE:LINKEDIT_FILE + len(linkedit)] = linkedit
    f[LINKEDIT_FILE + strings:LINKEDIT_FILE + strings + len(pool.data)] = pool.data
    return f


def subcache_file():
    f = bytearray(0x1000)
    f[:0x200] = cache_header(UUID_01, 1, [(312, "<II", 0x220, 1)])
    struct.pack_into("<QQQII", f, 0x200, ISLANDS, 0x1000, 0, 5, 5)
    struct.pack_into("<QQQQQQII", f, 0x220, ISLANDS, 0x1000, 0, 0, 0, TEXT_STUBS, 5, 5)
    f[ISLAND - ISLANDS:ISLAND - ISLANDS + 16] = island()
    return f


def symbols_file():
    local_pool = b" \0_helper_local\0"
    nlists = nlist(2, 0x0E, 1, CALLER_TEXT + 0x28)
    entries = struct.pack("<QII", CALLER - BASE, 0, 1)
    info_size = 24
    nlist_off, str_off = info_size, info_size + len(nlists)
    entries_off = str_off + len(local_pool)
    info = struct.pack("<IIIIII", nlist_off, 1, str_off, len(local_pool), entries_off, 1)
    blob = info + nlists + local_pool + entries
    return bytes(cache_header(UUID_SYMBOLS, 0, [(72, "<QQ", 0x200, len(blob))])) + blob


def build():
    pool = Pool()
    redacted = pool.add("<redacted>")
    names = {n: pool.add(n) for n in
             ["_caller_entry", "_caller_indirect", "_callee_add", "_callee_counter"]}
    linkedit = b"".join([
        nlist(redacted, 0x0E, 1, CALLER_TEXT + 0x28),
        nlist(names["_caller_entry"], 0x0F, 1, CALLER_TEXT),
        nlist(names["_caller_indirect"], 0x0F, 1, CALLER_TEXT + 0x30),
        nlist(names["_callee_add"], 0x0F, 1, CALLEE_TEXT),
        nlist(names["_callee_counter"], 0x0F, 2, COUNTER),
    ])
    os.makedirs(OUT, exist_ok=True)
    for suffix, data in [("", main_file(pool, linkedit)), (".01", subcache_file()),
                         (".symbols", symbols_file())]:
        with open(os.path.join(OUT, NAME + suffix), "wb") as out:
            out.write(data)


if __name__ == "__main__":
    build()
