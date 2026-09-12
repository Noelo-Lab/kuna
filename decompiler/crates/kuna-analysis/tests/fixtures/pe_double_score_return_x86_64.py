#!/usr/bin/env python3
"""Generate the self-contained PE witness for `declaring-double-score-return`.

The internal score function mirrors the PolyMLP.exe seam: it accumulates a
declared-double `log` result in XMM6 through a loop and post-log volatile calls,
divides the sum, copies it to ABI return register XMM0, and then calls a
declared-void MSVC cookie checker. The cookie value crosses the loop, so its
stack slot has a loop-carried MULTIEQUAL. The checker's failure tail contains a
nested call before `int 0x29`, deliberately making the generic callee-body proof
incomplete; only the exact `(cookie ^ SP) ^ SP` proof may preserve XMM0.

The score/log/cookie VMAs match the authoritative round-5 two-assert probe. This
reuses the minimal PE import-data builder next door and emits sparse executable
sections around those VMAs. Regenerate with
`python3 pe_double_score_return_x86_64.py`.
"""
import os
import struct

import pe_cookiecheck_x86_64 as pe


SCORE_SECTION_RVA = 0x3000
COOKIE_SECTION_RVA = 0x8000
LOG_SECTION_RVA = 0x9000
ENTRY = 0x3050

# Keep the three addresses from the authoritative round-5 minimal probe so its
# command and expectations remain byte-identical when promoted to this fixture.
SCORE = 0x3150
LOG_HELPER = 0x904F
COOKIE = 0x8140
FAILURE = 0x8180

# Negative controls live beside score in the first executable section.
NESTED = 0x3300
CLOBBER = 0x3304
CLOBBER_CALLER = 0x3310
COOKIE_CLOBBER = 0x3320
COOKIE_CLOBBER_CALLER = 0x3340
WIDE_COOKIE_CALLER = 0x3370
LOOP_BARRIER = 0x33E0


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_text(cookie_rva, log_iat_rva, loop_barrier_iat_rva):
    text = bytearray(LOG_SECTION_RVA + 0x200 - SCORE_SECTION_RVA)
    text[ENTRY - SCORE_SECTION_RVA] = 0xC3
    body = bytearray()
    body += b"\x48\x83\xec\x38"                              # sub rsp,0x38
    body += b"\x48\x8b\x05" + rel32(SCORE + len(body), 7, cookie_rva)
    body += b"\x48\x33\xc4"                                  # xor rax,rsp
    body += b"\x48\x89\x44\x24\x28"                          # mov [rsp+0x28],rax
    body += b"\x0f\x29\x74\x24\x10"                          # save xmm6
    body += b"\x48\x89\x5c\x24\x20"                          # save rbx
    body += b"\x89\xcb"                                      # mov ebx,ecx
    one = cookie_rva + 8
    two = cookie_rva + 16
    body += b"\xf2\x0f\x10\x35" + rel32(SCORE + len(body), 8, one)
    body += b"\xf2\x0f\x10\x3d" + rel32(SCORE + len(body), 8, one)
    loop = SCORE + len(body)
    body += b"\xf2\x0f\x10\x05" + rel32(SCORE + len(body), 8, two)
    body += b"\xf2\x0f\x58\xc7"                              # addsd xmm0,xmm7
    body += b"\xe8" + rel32(SCORE + len(body), 5, LOG_HELPER)
    body += b"\xf2\x0f\x58\xf0"                              # addsd xmm6,xmm0
    # The real loop calls `free` after consuming log's result. Model that
    # volatile-XMM barrier explicitly so the loop's log return is distinct from
    # the final XMM0 value copied across the later cookie call.
    body += b"\xe8" + rel32(SCORE + len(body), 5, LOOP_BARRIER)  # call free thunk
    body += b"\xff\xcb"                                       # dec ebx
    body += b"\x75" + struct.pack("b", loop - (SCORE + len(body) + 2))
    body += b"\xf2\x0f\x10\x05" + rel32(SCORE + len(body), 8, two)
    body += b"\xf2\x0f\x5e\xf0"                              # divsd xmm6,xmm0
    body += b"\xe8" + rel32(SCORE + len(body), 5, LOOP_BARRIER)  # destructor/free
    body += b"\x0f\x28\xc6"                                  # movaps xmm0,xmm6
    body += b"\x48\x8b\x4c\x24\x28\x48\x33\xcc"            # cookie ^ rsp
    body += b"\xe8" + rel32(SCORE + len(body), 5, COOKIE)
    body += b"\x0f\x28\x74\x24\x10"                          # restore xmm6
    body += b"\x48\x8b\x5c\x24\x20"                          # restore rbx
    body += b"\x48\x83\xc4\x38\xc3"
    assert SCORE + len(body) <= NESTED, hex(SCORE + len(body))
    text[SCORE - SCORE_SECTION_RVA:SCORE - SCORE_SECTION_RVA + len(body)] = body

    log_thunk = b"\xff\x25" + rel32(LOG_HELPER, 6, log_iat_rva)
    text[LOG_HELPER - SCORE_SECTION_RVA:LOG_HELPER - SCORE_SECTION_RVA + len(log_thunk)] = log_thunk

    check = bytearray()
    check += b"\x48\x3b\x0d" + rel32(COOKIE + len(check), 7, cookie_rva)
    check += b"\x75\x10\x48\xc1\xc1\x10\x66\xf7\xc1\xff\xff\x75\x01\xc3"
    check += b"\x48\xc1\xc9\x10"
    check += b"\xe9" + rel32(COOKIE + len(check), 5, FAILURE)
    text[COOKIE - SCORE_SECTION_RVA:COOKIE - SCORE_SECTION_RVA + len(check)] = check

    fail = bytearray(b"\x48\x83\xec\x28")
    fail += b"\xe8" + rel32(FAILURE + len(fail), 5, NESTED)
    fail += b"\xb9\x02\x00\x00\x00\xcd\x29\x48\x83\xc4\x28\xc3"
    text[FAILURE - SCORE_SECTION_RVA:FAILURE - SCORE_SECTION_RVA + len(fail)] = fail
    loop_barrier = b"\xff\x25" + rel32(LOOP_BARRIER, 6, loop_barrier_iat_rva)
    text[
        LOOP_BARRIER - SCORE_SECTION_RVA:
        LOOP_BARRIER - SCORE_SECTION_RVA + len(loop_barrier)
    ] = loop_barrier
    text[NESTED - SCORE_SECTION_RVA:NESTED - SCORE_SECTION_RVA + 3] = b"\x31\xc0\xc3"
    text[CLOBBER - SCORE_SECTION_RVA:CLOBBER - SCORE_SECTION_RVA + 5] = b"\x66\x0f\xef\xc0\xc3"
    control = b"\xf2\x0f\x10\x05" + rel32(CLOBBER_CALLER, 8, cookie_rva + 8)
    control += b"\xe8" + rel32(CLOBBER_CALLER + len(control), 5, CLOBBER) + b"\xc3"
    text[CLOBBER_CALLER - SCORE_SECTION_RVA:CLOBBER_CALLER - SCORE_SECTION_RVA + len(control)] = control

    body_write_then_call = bytearray(b"\x66\x0f\xef\xc0")
    body_write_then_call += b"\xe8" + rel32(
        COOKIE_CLOBBER + len(body_write_then_call), 5, NESTED
    )
    body_write_then_call += b"\xc3"
    text[
        COOKIE_CLOBBER - SCORE_SECTION_RVA:
        COOKIE_CLOBBER - SCORE_SECTION_RVA + len(body_write_then_call)
    ] = body_write_then_call
    exact = bytearray(b"\x48\x83\xec\x28")
    exact += b"\x48\x8b\x05" + rel32(COOKIE_CLOBBER_CALLER + len(exact), 7, cookie_rva)
    exact += b"\x48\x33\xc4\x48\x89\x44\x24\x20"
    exact += b"\xf2\x0f\x10\x05" + rel32(
        COOKIE_CLOBBER_CALLER + len(exact), 8, cookie_rva + 8
    )
    exact += b"\x48\x8b\x4c\x24\x20\x48\x33\xcc"
    exact += b"\xe8" + rel32(COOKIE_CLOBBER_CALLER + len(exact), 5, COOKIE_CLOBBER)
    exact += b"\x48\x83\xc4\x28\xc3"
    text[
        COOKIE_CLOBBER_CALLER - SCORE_SECTION_RVA:
        COOKIE_CLOBBER_CALLER - SCORE_SECTION_RVA + len(exact)
    ] = exact

    # A real 16-byte XMM heritage range containing the 8-byte Win64 output.
    # The cookie proof may retain the low double, but the upper scratch lane
    # must remain killed by the call rather than inheriting its pre-call 2.0.
    wide = bytearray(b"\x48\x83\xec\x28")
    wide += b"\x48\x8b\x05" + rel32(WIDE_COOKIE_CALLER + len(wide), 7, cookie_rva)
    wide += b"\x48\x33\xc4\x48\x89\x44\x24\x20"
    wide += b"\x66\x0f\x10\x05" + rel32(
        WIDE_COOKIE_CALLER + len(wide), 8, cookie_rva + 8
    )
    wide += b"\x48\x8b\x4c\x24\x20\x48\x33\xcc"
    wide += b"\xe8" + rel32(WIDE_COOKIE_CALLER + len(wide), 5, COOKIE)
    wide += b"\x0f\x12\xc8"                              # movhlps xmm1,xmm0
    wide += b"\xf2\x0f\x58\xc1"                          # addsd xmm0,xmm1
    wide += b"\x48\x83\xc4\x28\xc3"
    text[
        WIDE_COOKIE_CALLER - SCORE_SECTION_RVA:
        WIDE_COOKIE_CALLER - SCORE_SECTION_RVA + len(wide)
    ] = wide
    return text


def append_exports(rdata):
    """Append exports so the promoted probe prints `log` and `cookie`."""
    export_rva = pe.RDATA_RVA + len(rdata)
    names_and_rvas = [("cookie", COOKIE), ("log", LOG_HELPER)]
    funcs_off = 40
    names_off = funcs_off + 4 * len(names_and_rvas)
    ordinals_off = names_off + 4 * len(names_and_rvas)
    strings_off = ordinals_off + 2 * len(names_and_rvas)
    strings = bytearray(b"double_score_fixture.dll\0")
    dll_rva = export_rva + strings_off
    name_rvas = []
    for name, _ in names_and_rvas:
        name_rvas.append(export_rva + strings_off + len(strings))
        strings += name.encode() + b"\0"

    exports = bytearray()
    exports += struct.pack(
        "<IIHHIIIIIII",
        0, 0, 0, 0, dll_rva, 1, len(names_and_rvas), len(names_and_rvas),
        export_rva + funcs_off, export_rva + names_off, export_rva + ordinals_off,
    )
    for _, function_rva in names_and_rvas:
        exports += struct.pack("<I", function_rva)
    for name_rva in name_rvas:
        exports += struct.pack("<I", name_rva)
    for ordinal in range(len(names_and_rvas)):
        exports += struct.pack("<H", ordinal)
    exports += strings
    rdata += exports
    return export_rva, len(exports)


def build():
    original_imports = pe.IMPORTS
    try:
        pe.IMPORTS = [*original_imports, "free"]
        rdata, iat_rvas, imp_rva, imp_size, iat_rva, iat_size, cookie_rva = pe.build_rdata()
    finally:
        pe.IMPORTS = original_imports
    rdata = bytearray(rdata)
    rdata += struct.pack("<dd", 1.5, 2.0)
    export_rva, export_size = append_exports(rdata)
    text = build_text(cookie_rva, cookie_rva + 24, iat_rvas[-1])
    sections_data = [
        (".rdata", bytes(rdata), pe.RDATA_RVA, 0x40000040),
        (".score", bytes(text[:0x500]), SCORE_SECTION_RVA, 0x60000020),
        (".cookie", bytes(text[COOKIE_SECTION_RVA - SCORE_SECTION_RVA:
                               COOKIE_SECTION_RVA - SCORE_SECTION_RVA + 0x200]),
         COOKIE_SECTION_RVA, 0x60000020),
        (".log", bytes(text[LOG_SECTION_RVA - SCORE_SECTION_RVA:
                            LOG_SECTION_RVA - SCORE_SECTION_RVA + 0x200]),
         LOG_SECTION_RVA, 0x60000020),
    ]

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)
    nsec = len(sections_data)
    opt_size = 240
    hdr_size = 0x40 + 4 + 20 + opt_size + 40 * nsec
    headers_sz = (hdr_size + pe.FILE_ALIGN - 1) // pe.FILE_ALIGN * pe.FILE_ALIGN

    laid_out = []
    raw_off = headers_sz
    for name, data, rva, chars in sections_data:
        raw_size = (len(data) + pe.FILE_ALIGN - 1) // pe.FILE_ALIGN * pe.FILE_ALIGN
        laid_out.append((name, data, rva, chars, raw_off, raw_size))
        raw_off += raw_size

    image_sz = LOG_SECTION_RVA + pe.SECT_ALIGN
    code_size = sum(len(data) for _, data, _, chars in sections_data if chars & 0x20)
    blob = bytearray(dos)
    blob += b"PE\0\0"
    blob += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)
    opt = bytearray()
    opt += struct.pack(
        "<HBBIIIII", 0x20B, 14, 0, code_size, len(rdata), 0, ENTRY, SCORE_SECTION_RVA
    )
    opt += struct.pack("<Q", pe.IMAGE_BASE)
    opt += struct.pack(
        "<IIHHHHHHIIIIHHQQQQII",
        pe.SECT_ALIGN, pe.FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
        image_sz, headers_sz, 0, 3, 0x8160,
        0x100000, 0x1000, 0x100000, 0x1000, 0, 16,
    )
    dirs = [(0, 0)] * 16
    dirs[0] = (export_rva, export_size)
    dirs[1] = (imp_rva, imp_size)
    dirs[12] = (iat_rva, iat_size)
    for rva, size in dirs:
        opt += struct.pack("<II", rva, size)
    assert len(opt) == opt_size, len(opt)
    blob += opt

    def section_header(name, data, rva, chars, raw_offset, raw_size):
        return (name.encode().ljust(8, b"\0")
                + struct.pack("<IIIIIIHHI", len(data), rva, raw_size, raw_offset,
                              0, 0, 0, 0, chars))

    for section in laid_out:
        blob += section_header(*section)
    blob += bytes(headers_sz - len(blob))
    for _, data, _, _, _, raw_size in laid_out:
        blob += data.ljust(raw_size, b"\0")
    return bytes(blob)


if __name__ == "__main__":
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "pe_double_score_return_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
