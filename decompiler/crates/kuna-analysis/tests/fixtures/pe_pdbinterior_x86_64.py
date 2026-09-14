#!/usr/bin/env python3
"""Generate `pe_pdbinterior_x86_64.exe` and its matching sidecar
`pe_pdbinterior_x86_64.pdb` -- a PE32+ whose PDB is the only thing that knows
where one of its functions starts and ends.

The shape of the witness (GH-468, MSVC `/O2` `ab_o2.exe`): `cascade_switch` is a
leaf the compiler also inlined into its only caller, so the out-of-line copy has
no caller, no `.pdata` record and no export.  Only the PDB names it.  Recursive
descent never decodes it, so the aggressive gap walk (`aif`) probes its bytes as
an undiscovered hole and accepts a 4-byte-aligned interior instruction whose
two-mnemonic prologue (`mov eax,imm ; mov edx,imm`) matches the prologue most of
the image's discovered functions share.  That start lands on a fall-through
inside the leaf, so `funcboundflow` truncates the leaf there and one case of the
switch vanishes from the emitted C.

Beside it sits the same leaf as a `static` function: an `S_LPROC32` record and
no public, so nothing but the gap walk's interior entry represents its code and
that entry must survive.  Three more public procedures hold entries their own
flow never reaches: `handler_leaf` has a block after its last `ret` (where a
jump-table case or an exception handler sits), `caller_leaf` only calls its
inner routine, and `noreturn_leaf` puts its block after a call to `abort`.
Those entries are the only functions that code has and must survive too.
Three more `S_LPROC32` records carry extents no image can have (an overflowing
length, one running past `.text`, one in `.rdata`) for the executable-range
guard.

Layout (ImageBase 0x140000000, no `.pdata`, no exports):

  .text   RVA 0x1000, executable
    0x140001000  _start        sub rsp,0x28 ; call pick_00 .. pick_19
                               add rsp,0x28 ; ret     <- AddressOfEntryPoint
    0x140001080  pick_00 .. pick_09, 0x20 apart:
                               mov eax,imm ; mov edx,-1 ; cmp ecx,imm
                               cmovnz eax,edx ; ret
    0x1400011c0  cascade_leaf  reached by NOTHING but the PDB:
        +0x00  cmp  ecx,0x2003
        +0x06  jg   high
        +0x08  jz   ret_21
        +0x0a  cmp  ecx,0x1000
        +0x10  mov  eax,10          <- INTERIOR: a fall-through at an aligned
        +0x15  mov  edx,-1             address whose prologue matches pick_NN
        +0x1a  cmovnz eax,edx
        +0x1d  ret
        +0x1e  ret_21: mov eax,0x21 ; ret
        +0x24  high:   cmp ecx,0x3007
                       mov eax,0x4d ; mov edx,-1 ; cmovnz eax,edx ; ret
    0x140001200  pick_10 .. pick_19, 0x20 apart
    0x140001340  static_leaf   cascade_leaf's shape, S_LPROC32 only:
        +0x10  mov  eax,20          <- INTERIOR, the code's only entry
    0x140001380  handler_leaf  cmp ecx,0x9001 ; jz two ; mov eax,1 ; ret
                               two: mov eax,2 ; ret
        +0x20  mov  eax,0x33 ...    <- INTERIOR, after the last ret: unreached
    0x1400013c0  caller_leaf   call inner ; ret
        +0x10  inner: mov eax,0x44  <- INTERIOR, reached only by the call
    0x1400013f0  abort         ud2
    0x140001400  noreturn_leaf cmp ecx,0x9009 ; mov edx,-1 ; call abort
        +0x10  mov  eax,0x55 ...    <- INTERIOR, only past a no-return call
    0x140001428  absurd_length S_LPROC32, length 0xfffffff0 (int3 padding)
    0x140001430  past_text     S_LPROC32, length 0x20, .text ends at 0x140001438
  .rdata  RVA 0x2000  IMAGE_DEBUG_DIRECTORY + the RSDS CodeView record naming
                      `pe_pdbinterior_x86_64.pdb`
    0x140002008  in_rdata      S_LPROC32, length 8, not executable

The PDB is an MSF 7.00 container with 512-byte blocks carrying only what a PDB
reader needs to place a procedure: the info stream (GUID/age matching the RSDS
record), a DBI stream with one module and the section-headers debug stream, the
module's `S_GPROC32`/`S_LPROC32` records (each with its code length) and an
`S_PUB32` per global function in the symbol-record stream.

No Windows toolchain on this host, so both files are assembled byte by byte
(same pattern as `pe_reloctable_x86_64.py`).  Regenerate with:

    python3 pe_pdbinterior_x86_64.py
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000

START = 0x1000
PICKS_LO = 0x1080
LEAF = 0x11C0
PICKS_HI = 0x1200
PICK_COUNT = 20
STATIC_LEAF = 0x1340
HANDLER_LEAF = 0x1380
CALLER_LEAF = 0x13C0
ABORT = 0x13F0
NORETURN_LEAF = 0x1400
INDIRECT_LEAF = 0x1440
FASTFAIL_LEAF = 0x1480
CALLED_LEAF = 0x14C0
PUBINSIDE_LEAF = 0x1500
ABSURD_LENGTH = 0x1540
PAST_TEXT = 0x1548
TEXT_SIZE = 0x550
IN_RDATA = 0x2008

GUID = bytes.fromhex("4b554e41504442494e544552494f5231")
AGE = 1
BASENAME = "pe_pdbinterior_x86_64"
PDB_NAME = BASENAME + ".pdb"


def pick_rvas():
    return ([PICKS_LO + 0x20 * i for i in range(PICK_COUNT // 2)]
            + [PICKS_HI + 0x20 * i for i in range(PICK_COUNT // 2)])


class Text:
    def __init__(self, size):
        self.buf = bytearray(b"\xCC" * size)

    def put(self, rva, b):
        off = rva - TEXT_RVA
        self.buf[off:off + len(b)] = b
        return rva + len(b)


def mov_eax(imm):
    return b"\xB8" + struct.pack("<I", imm & 0xFFFFFFFF)


def mov_edx(imm):
    return b"\xBA" + struct.pack("<I", imm & 0xFFFFFFFF)


def cmp_ecx(imm):
    return b"\x81\xF9" + struct.pack("<I", imm)


CMOVNZ_EAX_EDX = b"\x0F\x45\xC2"
RET = b"\xC3"


def put_pick(t, at, value, key):
    """`mov eax,value ; mov edx,-1 ; cmp ecx,key ; cmovnz eax,edx ; ret`, the
    helpers' shape, 20 bytes.  Returns the end RVA."""
    p = t.put(at, mov_eax(value))
    p = t.put(p, mov_edx(-1))
    p = t.put(p, cmp_ecx(key))
    p = t.put(p, CMOVNZ_EAX_EDX)
    return t.put(p, RET)


def put_leaf(t, at, keys, values):
    """cascade_leaf's shape at `at`: a three-way compare whose middle arm falls
    through to an aligned `mov eax,imm ; mov edx,-1`.  Returns the end RVA."""
    p = at
    p = t.put(p, cmp_ecx(keys[1]))
    jg_at = p
    p = t.put(p, b"\x7F\x00")                            # jg   high
    jz_at = p
    p = t.put(p, b"\x74\x00")                            # jz   ret_mid
    p = t.put(p, cmp_ecx(keys[0]))
    assert p == at + 0x10 and (IMAGE_BASE + p) % 4 == 0, hex(p)
    p = t.put(p, mov_eax(values[0]))
    p = t.put(p, mov_edx(-1))
    p = t.put(p, CMOVNZ_EAX_EDX)
    p = t.put(p, RET)
    ret_mid = p
    p = t.put(p, mov_eax(values[1]))
    p = t.put(p, RET)
    high = p
    p = t.put(p, cmp_ecx(keys[2]))
    p = t.put(p, mov_eax(values[2]))
    p = t.put(p, mov_edx(-1))
    p = t.put(p, CMOVNZ_EAX_EDX)
    p = t.put(p, RET)
    t.put(jg_at + 1, struct.pack("<b", high - (jg_at + 2)))
    t.put(jz_at + 1, struct.pack("<b", ret_mid - (jz_at + 2)))
    return p


def build_text():
    """Returns (bytes, [(name, rva, length)], [(name, rva, length)]): the global
    functions, then the local procedures."""
    t = Text(TEXT_SIZE)
    funcs = []

    p = START
    p = t.put(p, b"\x48\x83\xEC\x28")                    # sub  rsp,0x28
    for rva in pick_rvas():
        p = t.put(p, b"\xE8" + struct.pack("<i", rva - (p + 5)))  # call pick_NN
    p = t.put(p, b"\xE8" + struct.pack("<i", CALLED_LEAF - (p + 5)))  # call called_leaf
    p = t.put(p, b"\xE8" + struct.pack("<i", (CALLED_LEAF + 0x10) - (p + 5)))  # call called_leaf+0x10 (its interior)
    p = t.put(p, b"\x48\x83\xC4\x28")                    # add  rsp,0x28
    p = t.put(p, RET)
    funcs.append(("_start", START, p - START))
    assert p <= PICKS_LO, hex(p)

    for i, rva in enumerate(pick_rvas()):
        p = put_pick(t, rva, i + 1, 0x100 + i * 0x11)
        funcs.append(("pick_%02d" % i, rva, p - rva))

    p = put_leaf(t, LEAF, (0x1000, 0x2003, 0x3007), (10, 0x21, 0x4D))
    funcs.append(("cascade_leaf", LEAF, p - LEAF))
    assert p <= PICKS_HI, hex(p)
    assert pick_rvas()[-1] + 0x14 <= STATIC_LEAF

    p = put_leaf(t, STATIC_LEAF, (0x5000, 0x6003, 0x7007), (20, 0x41, 0x8D))
    locals_ = [("static_leaf", STATIC_LEAF, p - STATIC_LEAF)]
    assert p <= HANDLER_LEAF

    p = t.put(HANDLER_LEAF, cmp_ecx(0x9001))
    jz_at = p
    p = t.put(p, b"\x74\x00")                            # jz   two
    p = t.put(p, mov_eax(1))
    p = t.put(p, RET)
    t.put(jz_at + 1, struct.pack("<b", p - (jz_at + 2)))
    p = t.put(p, mov_eax(2))
    p = t.put(p, RET)
    unreached = HANDLER_LEAF + 0x20
    assert p <= unreached and (IMAGE_BASE + unreached) % 4 == 0
    p = put_pick(t, unreached, 0x33, 0x9003)
    funcs.append(("handler_leaf", HANDLER_LEAF, p - HANDLER_LEAF))
    assert p <= CALLER_LEAF

    inner = CALLER_LEAF + 0x10
    p = t.put(CALLER_LEAF, b"\xE8" + struct.pack("<i", inner - (CALLER_LEAF + 5)))
    p = t.put(p, RET)
    assert (IMAGE_BASE + inner) % 4 == 0
    p = put_pick(t, inner, 0x44, 0x9007)
    funcs.append(("caller_leaf", CALLER_LEAF, p - CALLER_LEAF))
    assert p <= ABORT

    p = t.put(ABORT, b"\x0F\x0B")                        # ud2
    funcs.append(("abort", ABORT, p - ABORT))

    after = NORETURN_LEAF + 0x10
    p = t.put(NORETURN_LEAF, cmp_ecx(0x9009))
    p = t.put(p, mov_edx(-1))
    p = t.put(p, b"\xE8" + struct.pack("<i", ABORT - (p + 5)))
    assert p == after and (IMAGE_BASE + after) % 4 == 0
    p = put_pick(t, after, 0x55, 0x900B)
    funcs.append(("noreturn_leaf", NORETURN_LEAF, p - NORETURN_LEAF))
    assert p <= INDIRECT_LEAF

    # indirect_leaf: an indirect CALL through a memory slot -- a call with no
    # static target (the s1b IAT no-return shape, without an import table). The
    # block after it is a gap-walk interior entry.
    p = t.put(INDIRECT_LEAF, cmp_ecx(0x200A))
    jz_at = p
    p = t.put(p, b"\x74\x00")                            # jz ok
    slot = IMAGE_BASE + RDATA_RVA
    p = t.put(p, b"\xFF\x15" + struct.pack("<i", slot - (IMAGE_BASE + p + 6)))  # call [rip+d]
    ind_interior = INDIRECT_LEAF + 0x10
    while p < ind_interior:
        p = t.put(p, b"\x90")                            # nop to alignment
    assert (IMAGE_BASE + ind_interior) % 4 == 0
    p = put_pick(t, ind_interior, 0x66, 0x900D)
    ok = p
    p = t.put(p, mov_eax(0x21))
    p = t.put(p, RET)
    t.put(jz_at + 1, struct.pack("<b", ok - (jz_at + 2)))
    funcs.append(("indirect_leaf", INDIRECT_LEAF, p - INDIRECT_LEAF))
    assert p <= FASTFAIL_LEAF

    # fastfail_leaf: int 0x29, the fastfail userop-call (no-return by default),
    # then a gap-walk interior entry.
    p = t.put(FASTFAIL_LEAF, cmp_ecx(0x200B))
    jz_at = p
    p = t.put(p, b"\x74\x00")                            # jz ok
    p = t.put(p, b"\xCD\x29")                            # int 0x29
    ff_interior = FASTFAIL_LEAF + 0x10
    while p < ff_interior:
        p = t.put(p, b"\x90")
    assert (IMAGE_BASE + ff_interior) % 4 == 0
    p = put_pick(t, ff_interior, 0x77, 0x900E)
    ok = p
    p = t.put(p, mov_eax(0x21))
    p = t.put(p, RET)
    t.put(jz_at + 1, struct.pack("<b", ok - (jz_at + 2)))
    funcs.append(("fastfail_leaf", FASTFAIL_LEAF, p - FASTFAIL_LEAF))
    assert p <= CALLED_LEAF

    # called_leaf: reached by a direct call from _start, so its start is a
    # committed function even with `pdb off`; cascade_leaf's shape, with an
    # aligned interior fall-through the gap walk mints.
    p = put_leaf(t, CALLED_LEAF, (0x200C, 0x6003, 0x7007), (0x88, 0x41, 0x8D))
    funcs.append(("called_leaf", CALLED_LEAF, p - CALLED_LEAF))
    assert p <= PUBINSIDE_LEAF

    # pubinside_leaf: a procedure carrying a second S_PUB32 at an interior label
    # (no procedure there). The public marks a function boundary inside the
    # extent, so the extent must not be used.
    p = put_leaf(t, PUBINSIDE_LEAF, (0x200D, 0x6003, 0x7007), (0x99, 0x41, 0x8D))
    funcs.append(("pubinside_leaf", PUBINSIDE_LEAF, p - PUBINSIDE_LEAF))
    extra_publics = [("pub_inside", PUBINSIDE_LEAF + 0x10)]
    assert p <= ABSURD_LENGTH

    locals_ += [("absurd_length", ABSURD_LENGTH, 0xFFFFFFF0),
                ("past_text", PAST_TEXT, 0x20),
                ("in_rdata", IN_RDATA, 8)]
    assert PAST_TEXT < TEXT_RVA + TEXT_SIZE < PAST_TEXT + 0x20
    return bytes(t.buf), funcs, locals_, extra_publics


def build_rdata(rdata_off):
    rsds = b"RSDS" + GUID + struct.pack("<I", AGE) + PDB_NAME.encode() + b"\0"
    dd_size = 28
    rec_rva = RDATA_RVA + dd_size
    rec_off = rdata_off + dd_size
    dd = struct.pack("<IIHHIIII", 0, 0, 0, 0, 2, len(rsds), rec_rva, rec_off)
    return dd + rsds, (RDATA_RVA, dd_size)


def section_headers(text_len, rdata_len, text_off, rdata_off):
    def align(n):
        return (n + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN

    def sect(name, vsz, rva, rsz, roff, chars):
        return (name.encode().ljust(8, b"\0")
                + struct.pack("<IIIIIIHHI", vsz, rva, rsz, roff, 0, 0, 0, 0, chars))

    return (sect(".text", text_len, TEXT_RVA, align(text_len), text_off, 0x60000020)
            + sect(".rdata", rdata_len, RDATA_RVA, align(rdata_len), rdata_off, 0x40000040))


def layout(text_len):
    nsec = 2
    opt_size = 240
    hdr_size = 0x40 + 4 + 20 + opt_size + 40 * nsec
    headers_sz = (hdr_size + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    text_off = headers_sz
    rdata_off = text_off + (text_len + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    return nsec, opt_size, headers_sz, text_off, rdata_off


def build_pe(text):
    nsec, opt_size, headers_sz, text_off, rdata_off = layout(len(text))
    rdata, debug_dir = build_rdata(rdata_off)
    image_sz = RDATA_RVA + SECT_ALIGN

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)
    b = bytearray(dos)
    b += b"PE\0\0"
    b += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)
    opt = bytearray()
    opt += struct.pack("<HBBIIIII", 0x20B, 14, 0, len(text), len(rdata), 0,
                       START, TEXT_RVA)
    opt += struct.pack("<Q", IMAGE_BASE)
    opt += struct.pack("<IIHHHHHHIIIIHHQQQQII",
                       SECT_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
                       image_sz, headers_sz, 0, 3, 0x8160,
                       0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = [(0, 0)] * 16
    dirs[6] = debug_dir                       # IMAGE_DIRECTORY_ENTRY_DEBUG
    for rva, sz in dirs:
        opt += struct.pack("<II", rva, sz)
    assert len(opt) == opt_size, len(opt)
    b += opt
    b += section_headers(len(text), len(rdata), text_off, rdata_off)
    b += bytes(headers_sz - len(b))
    for blob in (text, rdata):
        b += blob.ljust((len(blob) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN, b"\0")
    return bytes(b), section_headers(len(text), len(rdata), text_off, rdata_off)


def cv_record(kind, body):
    body = body + b"\0" * (-len(body) % 4)
    return struct.pack("<HH", len(body) + 2, kind) + body


S_END, S_PUB32, S_LPROC32, S_GPROC32 = 0x0006, 0x110E, 0x110F, 0x1110


def module_stream(funcs, locals_):
    out = bytearray(struct.pack("<I", 4))
    records = [(S_GPROC32, f) for f in funcs] + [(S_LPROC32, f) for f in locals_]
    for kind, (name, rva, length) in records:
        start = len(out)
        segment, offset = (2, rva - RDATA_RVA) if rva >= RDATA_RVA else (1, rva - TEXT_RVA)
        rec = cv_record(kind, struct.pack(
            "<IIIIIIIIHB", 0, 0, 0, length, 0, length, 0,
            offset, segment, 0) + name.encode() + b"\0")
        end = start + len(rec)
        rec = rec[:8] + struct.pack("<I", end) + rec[12:]
        out += rec
        out += cv_record(S_END, b"")
    return bytes(out)


def module_stream_with_refs(symbols):
    return symbols + struct.pack("<I", 0)                # GlobalRefs size


def symbol_records(funcs, extra_publics):
    out = bytearray()
    pubs = [(name, rva) for name, rva, _ in funcs] + list(extra_publics)
    for name, rva in pubs:
        out += cv_record(S_PUB32, struct.pack("<IIH", 2, rva - TEXT_RVA, 1)
                         + name.encode() + b"\0")
    return bytes(out)


def info_stream():
    return (struct.pack("<III", 20000404, 0, AGE) + GUID
            + struct.pack("<I", 0)                      # names buffer size
            + struct.pack("<IIII", 0, 1, 0, 0)          # hash table: size, cap, present, deleted
            + struct.pack("<I", 20140508))              # feature: VC140


def type_stream():
    return struct.pack("<IIIII HHII iI iI iI", 20040203, 56, 0x1000, 0x1000, 0,
                       0xFFFF, 0xFFFF, 4, 0x3FFFF, 0, 0, 0, 0, 0, 0)


def dbi_stream(mod_stream_idx, mod_symbols_size, symrec_idx, sechdr_idx, text_len):
    mod_name = b"cascade.obj\0"
    mi = struct.pack("<I", 0)
    mi += struct.pack("<HHIIIHHII", 1, 0, 0, text_len, 0x60000020, 0, 0, 0, 0)
    mi += struct.pack("<HHIIIHHIII", 0, mod_stream_idx, mod_symbols_size, 0, 0,
                      0, 0, 0, 0, 0)
    mi += mod_name + mod_name
    mi += b"\0" * (-len(mi) % 4)
    sec_contrib = struct.pack("<I", 0xEFFE0000 + 19970605)
    sec_map = struct.pack("<HH", 0, 0)
    file_info = struct.pack("<HH", 1, 0) + struct.pack("<HH", 0, 0)
    file_info += b"\0" * (-len(file_info) % 4)
    ec_names = struct.pack("<III", 0xEFFEEFFE, 1, 1) + b"\0" + struct.pack("<III", 1, 0, 0)
    dbg_hdr = struct.pack("<11H", 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, sechdr_idx,
                          0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF)
    hdr = struct.pack("<iIIHHHHHHIIIIIIIIHHI",
                      -1, 19990903, AGE, 0xFFFF, 0x8E1D, 0xFFFF, 0, symrec_idx, 0,
                      len(mi), len(sec_contrib), len(sec_map), len(file_info), 0, 0,
                      len(dbg_hdr), len(ec_names), 0, 0x8664, 0)
    assert len(hdr) == 64, len(hdr)
    return hdr + mi + sec_contrib + sec_map + file_info + ec_names + dbg_hdr


def msf(streams, block=512):
    """An MSF 7.00 container: block 0 superblock, 1-2 free-block maps, then the
    stream-directory block list, the directory, and each stream's blocks."""
    def nblocks(n):
        return (n + block - 1) // block

    next_block = 3
    stream_blocks = []
    for s in streams:
        stream_blocks.append(list(range(next_block, next_block + nblocks(len(s)))))
        next_block += nblocks(len(s))
    directory = struct.pack("<I", len(streams))
    directory += b"".join(struct.pack("<I", len(s)) for s in streams)
    directory += b"".join(struct.pack("<%dI" % len(bl), *bl) for bl in stream_blocks)
    dir_blocks = list(range(next_block, next_block + nblocks(len(directory))))
    next_block += len(dir_blocks)
    block_map_addr = next_block
    next_block += 1
    total = next_block

    image = bytearray(total * block)
    magic = b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0"
    sb = magic + struct.pack("<IIIIII", block, 1, total, len(directory), 0, block_map_addr)
    image[0:len(sb)] = sb
    fpm = bytearray(b"\xFF" * block)
    for i in range(total):
        fpm[i // 8] &= ~(1 << (i % 8)) & 0xFF
    image[block:2 * block] = fpm
    for s, bl in zip(streams, stream_blocks):
        for k, blk in enumerate(bl):
            chunk = s[k * block:(k + 1) * block]
            image[blk * block:blk * block + len(chunk)] = chunk
    for k, blk in enumerate(dir_blocks):
        chunk = directory[k * block:(k + 1) * block]
        image[blk * block:blk * block + len(chunk)] = chunk
    bm = struct.pack("<%dI" % len(dir_blocks), *dir_blocks)
    image[block_map_addr * block:block_map_addr * block + len(bm)] = bm
    if len(image) < 4096:
        raise AssertionError("pdb readers map a 4 KiB superblock page")
    return bytes(image)


def build_pdb(funcs, locals_, extra_publics, sechdrs, text_len):
    mod = module_stream(funcs, locals_)
    syms = symbol_records(funcs, extra_publics)
    MOD, SYMREC, SECHDR = 5, 6, 7
    streams = [b"", info_stream(), type_stream(),
               dbi_stream(MOD, len(mod), SYMREC, SECHDR, text_len),
               type_stream(), module_stream_with_refs(mod), syms, sechdrs]
    return msf(streams)


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    text, funcs, locals_, extra_publics = build_text()
    pe, sechdrs = build_pe(text)
    pdb = build_pdb(funcs, locals_, extra_publics, sechdrs, len(text))
    for ext, blob in ((".exe", pe), (".pdb", pdb)):
        out = os.path.join(here, BASENAME + ext)
        with open(out, "wb") as f:
            f.write(blob)
        print(f"wrote {out} ({len(blob)} bytes)")


if __name__ == "__main__":
    main()
