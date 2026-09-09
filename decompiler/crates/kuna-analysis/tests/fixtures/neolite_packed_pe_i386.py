#!/usr/bin/env python3
"""Build `neolite_packed_pe_i386.exe`, a minimal NEOLite-packed 32-bit PE.

NEOLite (NeoWorx) is a commercial packer from around 1999 and no compressor for
it is available, so this script carries one: a reference encoder for the format
`kuna_analysis::neolite` decodes, written from the loader stub of a real packed
image and validated against it.  The fixture it produces is deliberately not a
recording of that image -- it is the smallest file that exercises every branch
the decoder has:

  * `.text`   one verbatim block and one aligned-offset block, long matches
              through the extended-length tree, and a repeated-offset (R0) match
  * `.rdata`  the original import descriptor array, so import recovery has
              something to find
  * `.data`   a short payload followed by the section's zero tail, encoded the
              way the packer encodes one -- as matches, out to the virtual size
  * `.rsrc`   plaintext the packer left alone, which must survive as it stands
  * `.NEOpack`/`.NEOdata` the loader sections, and a `push OEP; ret` hand-over

Run with no arguments to rewrite the .exe next to this file.
"""

import os
import struct
import sys

LEN_BASE = [0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 20, 24, 28, 32, 40, 48, 56,
            64, 80, 96, 112, 128, 160, 192, 224]
LEN_EXTRA = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5]
DIST_EXTRA = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10,
              11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 17, 17, 17, 17, 17,
              17, 17, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 18, 18, 18]
DIST_BASE = []
_acc = 0
for _e in DIST_EXTRA:
    DIST_BASE.append(_acc)
    _acc += 1 << _e

NUM_MAIN, NUM_LEN, NUM_ALIGN = 721, 28, 8
NUM_LENS = NUM_MAIN + NUM_LEN + NUM_ALIGN


class BitWriter:
    def __init__(self):
        self.out = bytearray()
        self.acc = 0
        self.n = 0

    def put(self, value, bits):
        for i in range(bits - 1, -1, -1):
            self.acc = (self.acc << 1) | ((value >> i) & 1)
            self.n += 1
            if self.n == 8:
                self.out.append(self.acc)
                self.acc = 0
                self.n = 0

    def finish(self):
        if self.n:
            self.put(0, 8 - self.n)
        # The reader keeps a four-byte accumulator and may look past the last
        # symbol it needs.
        return bytes(self.out) + b"\0" * 8


def canonical(lens):
    """Symbol -> (code, length), in the order the decoder's tables assume."""
    codes = {}
    code = 0
    for k in range(1, 16):
        for sym, l in enumerate(lens):
            if l == k:
                codes[sym] = (code, k)
                code += 1
        code <<= 1
    return codes


def huffman_lengths(freqs, limit=15):
    """Code lengths for a complete canonical code over the used symbols."""
    used = [(f, s) for s, f in enumerate(freqs) if f]
    if not used:
        raise ValueError("no symbol is used")
    if len(used) == 1:
        # A one-symbol code cannot fill the space; give it a partner so the
        # table stays complete, which is what the decoder insists on.
        spare = next(s for s in range(len(freqs)) if s != used[0][1])
        used.append((1, spare))
    nodes = [(f, [s]) for f, s in used]
    lens = [0] * len(freqs)
    while len(nodes) > 1:
        nodes.sort(key=lambda n: (n[0], min(n[1])))
        a, b = nodes.pop(0), nodes.pop(0)
        for s in a[1] + b[1]:
            lens[s] += 1
        nodes.append((a[0] + b[0], a[1] + b[1]))
    if max(lens) > limit:
        raise ValueError("code longer than %d bits" % limit)
    return lens


class Encoder:
    """One block's worth of tokens, then the bits for it."""

    def __init__(self, aligned):
        self.aligned = aligned
        self.tokens = []

    def literal(self, byte):
        self.tokens.append(("lit", byte))

    def match(self, length, formatted):
        self.tokens.append(("mat", length, formatted))

    def _parts(self, length, formatted):
        len_slot = min(length - 2, 7)
        ext = None
        if len_slot == 7:
            want = length - 9
            i = max(j for j in range(28) if LEN_BASE[j] <= want)
            extra = want - LEN_BASE[i]
            if extra >= (1 << LEN_EXTRA[i]):
                raise ValueError("length %d is not encodable" % length)
            ext = (i, extra, LEN_EXTRA[i])
        slot = max(j for j in range(58) if DIST_BASE[j] <= formatted)
        dist_extra = formatted - DIST_BASE[slot]
        if dist_extra >= (1 << DIST_EXTRA[slot]):
            raise ValueError("offset %d is not encodable" % formatted)
        return len_slot, ext, slot, dist_extra

    def emit(self, w):
        main_f = [0] * NUM_MAIN
        len_f = [0] * NUM_LEN
        plan = []
        for t in self.tokens:
            if t[0] == "lit":
                main_f[t[1]] += 1
                plan.append(("lit", t[1]))
            else:
                len_slot, ext, slot, dist_extra = self._parts(t[1], t[2])
                sym = 256 + slot * 8 + len_slot
                main_f[sym] += 1
                if ext:
                    len_f[ext[0]] += 1
                plan.append(("mat", sym, ext, slot, dist_extra))
        main_f[720] += 1  # the end-of-block symbol always carries a code
        main_lens = huffman_lengths(main_f)
        len_lens = huffman_lengths(len_f) if any(len_f) else [0] * NUM_LEN
        if not any(len_lens):
            len_lens = [1, 1] + [0] * (NUM_LEN - 2)
        align_lens = [2, 2, 3, 3, 4, 4, 4, 4] if self.aligned else [3] * NUM_ALIGN

        lens = main_lens + len_lens + align_lens
        assert len(lens) == NUM_LENS
        # The code-length alphabet: symbol N means "this length is N", because
        # the previous block's lengths are zeroed by the reset bit below.
        cl_f = [0] * 19
        for l in lens:
            cl_f[l] += 1
        cl_lens = huffman_lengths(cl_f)
        cl_codes = canonical(cl_lens)

        w.put(0, 1)  # reset: decode lengths against an all-zero previous block
        for l in cl_lens:
            w.put(l, 4)
        for l in lens:
            code, bits = cl_codes[l]
            w.put(code, bits)

        main_codes = canonical(main_lens)
        len_codes = canonical(len_lens)
        align_codes = canonical(align_lens)
        for step in plan:
            if step[0] == "lit":
                code, bits = main_codes[step[1]]
                w.put(code, bits)
                continue
            _, sym, ext, slot, dist_extra = step
            code, bits = main_codes[sym]
            w.put(code, bits)
            if ext:
                i, extra, nbits = ext
                c, b = len_codes[i]
                w.put(c, b)
                w.put(extra, nbits)
            nbits = DIST_EXTRA[slot]
            if self.aligned and nbits >= 3:
                w.put(dist_extra >> 3, nbits - 3)
                c, b = align_codes[dist_extra & 7]
                w.put(c, b)
            else:
                w.put(dist_extra, nbits)
        return main_codes[720]


def compress(blocks):
    """`blocks` is a list of Encoder; all but the last end with symbol 720."""
    w = BitWriter()
    for i, enc in enumerate(blocks):
        end = enc.emit(w)
        if i + 1 < len(blocks):
            w.put(end[0], end[1])
    return w.finish()


# --- the fixture's payloads ------------------------------------------------

SNIPPET = bytes.fromhex("558bec8b4508034d0c5dc39090909090")  # 16 bytes, x86
TEXT = SNIPPET * 256                                          # 4096 bytes
DATA = b"kuna neolite fixture payload\0"
RSRC = b"stored plaintext the packer left alone\0" * 4

IMAGE_BASE = 0x400000
OEP_RVA = 0x1000
STUB_RVA = 0x6000


def build_rdata():
    """An import directory at RVA 0x2000 + 0x40, with two DLLs."""
    base = 0x2000
    body = bytearray(0x400)
    iat = 0x00        # thunk arrays live at the front of the section
    descr = 0x40
    names = 0x100
    strings = bytearray()

    def add_string(s):
        off = names + len(strings)
        strings.extend(s + b"\0")
        if len(strings) & 1:
            strings.append(0)
        return base + off

    entries = [(b"KERNEL32.dll", [b"ExitProcess", b"GetTickCount"]),
               (b"USER32.dll", [b"MessageBoxA"])]
    thunk = iat
    for i, (dll, funcs) in enumerate(entries):
        dll_rva = add_string(dll)
        first_thunk = base + thunk
        for f in funcs:
            hint = base + names + len(strings)
            strings.extend(struct.pack("<H", 0) + f + b"\0")
            if len(strings) & 1:
                strings.append(0)
            struct.pack_into("<I", body, thunk, hint)
            thunk += 4
        struct.pack_into("<I", body, thunk, 0)
        thunk += 4
        struct.pack_into("<IIIII", body, descr + i * 20, 0, 0, 0, dll_rva, first_thunk)
    struct.pack_into("<IIIII", body, descr + len(entries) * 20, 0, 0, 0, 0, 0)
    body[names:names + len(strings)] = strings
    return bytes(body), base + descr


def pack_text():
    """One verbatim block, then an aligned-offset block, over `TEXT`."""
    a = Encoder(aligned=False)
    for b in SNIPPET:
        a.literal(b)
    # distance 16 -> offset 15 -> formatted 18; then the same offset again
    # through the repeated-offset slot 0.
    a.match(264, 18)
    for _ in range(3):
        a.match(264, 0)
    b = Encoder(aligned=True)
    remaining = len(TEXT) - 16 - 264 * 4
    while remaining > 0:
        n = min(264, remaining)
        b.match(n, 18 if n == 264 else 0)
        remaining -= n
    return compress([a, b])


def pack_bytes(payload, total=None):
    """Encode `payload`, then zeros out to `total` -- the packer sizes a stream
    by the section's virtual size, so the zero tail is part of it."""
    e = Encoder(aligned=False)
    for byte in payload:
        e.literal(byte)
    remaining = (total or len(payload)) - len(payload)
    if remaining:
        e.literal(0)
        remaining -= 1
        while remaining > 0:
            n = min(264, remaining)
            e.match(n, 3)  # distance 1
            remaining -= n
    return compress([e])


def build():
    rdata, import_rva = build_rdata()
    stub = bytearray(b"\x90" * 0x40)
    stub[0:1] = b"\x90"
    stub[1:6] = b"\xe8\x00\x00\x00\x00"
    stub[6:11] = b"\x68" + struct.pack("<I", IMAGE_BASE + OEP_RVA) + b"\xc3"

    sections = [
        # name, vaddr, vsize, raw bytes, characteristics
        (b".text", 0x1000, 0x1000, pack_text(), 0xC0000040),
        (b".rdata", 0x2000, 0x0400, pack_bytes(rdata), 0xC0000040),
        (b".data", 0x3000, 0x2000, pack_bytes(DATA, 0x2000), 0xC0000040),
        (b".rsrc", 0x5000, 0x1000, RSRC, 0xC0000040),
        (b".NEOpack", STUB_RVA, 0x1000, bytes(stub), 0xC0000040),
        (b".NEOdata", 0x7000, 0x1000, b"", 0xC0000040),
    ]

    file_align, sect_align = 0x200, 0x1000
    headers = 0x400
    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3c, 0x40)

    pe = bytearray()
    pe += b"PE\0\0"
    pe += struct.pack("<HHIIIHH", 0x14c, len(sections), 0, 0, 0, 0xE0, 0x0102)
    opt = bytearray(0xE0)
    struct.pack_into("<HBBIIIIII", opt, 0, 0x10b, 6, 0, 0x1000, 0x1000, 0x1000,
                     STUB_RVA + 1, 0x1000, 0x2000)
    struct.pack_into("<IIIHHHHHHIIIIHH", opt, 28, IMAGE_BASE, sect_align, file_align,
                     4, 0, 0, 0, 4, 0, 0, 0x8000, headers, 0, 2, 0)
    struct.pack_into("<IIIIII", opt, 72, 0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    # data directory 1 (import) points at the stub's own table, as a packed
    # image's does; unpacking is what moves it back.
    struct.pack_into("<II", opt, 96 + 8, STUB_RVA + 0x100, 40)
    pe += opt

    table = bytearray()
    body = bytearray()
    offset = headers
    for name, va, vsize, raw, ch in sections:
        padded = (len(raw) + file_align - 1) // file_align * file_align
        ptr = offset if raw else 0
        table += name.ljust(8, b"\0")
        table += struct.pack("<IIII", vsize, va, padded if raw else 0, ptr)
        table += struct.pack("<IIHHI", 0, 0, 0, 0, ch)
        if raw:
            body += raw + b"\0" * (padded - len(raw))
            offset += padded

    out = bytearray(dos + pe + table)
    assert len(out) <= headers, "headers do not fit in %#x bytes" % headers
    out += b"\0" * (headers - len(out))
    out += body
    return bytes(out), import_rva


if __name__ == "__main__":
    image, import_rva = build()
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                        "neolite_packed_pe_i386.exe")
    with open(path, "wb") as f:
        f.write(image)
    print("wrote %s (%d bytes); original import directory at %#x"
          % (path, len(image), import_rva), file=sys.stderr)
