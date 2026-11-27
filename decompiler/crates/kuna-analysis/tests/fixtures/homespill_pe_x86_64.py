#!/usr/bin/env python3
"""Generate a minimal PE32+ whose functions spill their parameters to home slots.

Each function stores its incoming argument registers into the home slots the
Windows x64 convention reserves for them, as MSVC /Od does, and most never read
the slot back:

  0x140001000  entry: calls the five functions below
  0x140001100  caller(int, int, int, char): spills all four, then passes the
               reloaded char as the fifth of nine arguments to callee
  0x140001200  callee(long long *, long long *, unsigned char *, long long)
  0x140001300  unused2(int, int): spills both, returns 0
  0x140001340  mixed(double, int): spills both, returns the int
  0x140001380  negatives(void): stores rbx, r8 (into rcx's slot) and a zeroed
               r9 into home slots; none of those is a parameter
  0x1400013c0  reused(void): spills rdx, then keeps a local in the same slot

The bytes were assembled with `llvm-mc -triple=x86_64` from the listing in the
comments.

    python3 homespill_pe_x86_64.py
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
DATA_RVA = 0x2000

FUNCTIONS = [
    # sub rsp,0x28; mov ecx,1; mov edx,2; mov r8d,3; mov r9d,4; call caller
    # mov ecx,1; mov edx,2; call unused2
    # movsd xmm0,[rip+dbl]; mov edx,7; call mixed; call negatives
    # mov edx,9; call reused; add rsp,0x28; ret
    (0x000, "4883ec28b901000000ba0200000041b80300000041b904000000e8e1000000"
            "b901000000ba02000000e8d2020000f20f1005fa0f0000ba07000000e80003"
            "0000e83b030000ba09000000e8710300004883c428c3"),
    # mov [rsp+0x20],r9b; mov [rsp+0x18],r8d; mov [rsp+0x10],edx
    # mov [rsp+0x8],ecx; sub rsp,0x68
    # mov eax,[rip+gflag]; and eax,3; cmp eax,3; jne zero
    # mov qword [rsp+0x40],0x22; mov eax,[rip+gflag]; mov [rsp+0x38],rax
    # mov qword [rsp+0x30],0x22; mov qword [rsp+0x28],0x22
    # movzx eax,byte [rsp+0x88]; mov [rsp+0x20],eax
    # mov r9d,0x22; lea r8,[rip+msg]; mov rdx,[rip+gb]; mov rcx,[rip+ga]
    # call callee; movabs rcx,0x8000000000000005; cmp rax,rcx; jne zero
    # mov rax,rcx; jmp out; zero: xor eax,eax; out: add rsp,0x68; ret
    (0x100, "44884c2420448944241889542410894c24084883ec688b05f40e000083e003"
            "83f803756648c7442440220000008b05dd0e0000488944243848c744243022"
            "00000048c7442428220000000fb68424880000008944242041b9220000004c"
            "8d05b50e0000488b159e0e0000488b0d8f0e0000e88a00000048b905000000"
            "000000804839c875054889c8eb0231c04883c468c3"),
    # mov [rsp+0x20],r9; mov [rsp+0x18],r8; mov rax,[rcx]; add rax,[rdx]
    # movzx r8d,byte [r8]; add rax,r8; add rax,r9; ret
    (0x200, "4c894c24204c89442418488b01480302450fb6004c01c04c01c8c3"),
    # mov [rsp+0x10],edx; mov [rsp+0x8],ecx; xor eax,eax; ret
    (0x300, "89542410894c240831c0c3"),
    # mov [rsp+0x10],edx; movsd [rsp+0x8],xmm0; mov eax,[rsp+0x10]; ret
    (0x340, "89542410f20f114424088b442410c3"),
    # mov [rsp+0x10],rbx; mov [rsp+0x8],r8; xor r9d,r9d; mov [rsp+0x20],r9
    # mov eax,1; ret
    (0x380, "48895c24104c894424084531c94c894c2420b801000000c3"),
    # mov [rsp+0x10],rdx; mov dword [rsp+0x10],5; mov eax,[rsp+0x10]; ret
    (0x3c0, "4889542410c7442410050000008b442410c3"),
]


def build_text():
    text = bytearray(0x400)
    for offset, code in FUNCTIONS:
        body = bytes.fromhex(code)
        text[offset:offset + len(body)] = body
    return bytes(text).rstrip(b"\0")


def build_data():
    data = bytearray()
    data += struct.pack("<QQ", 0, 0)                  # ga, gb
    data += struct.pack("<II", 3, 0)                  # gflag
    data += b"status %d %d %d\n\0".ljust(24, b"\0")   # msg
    data += struct.pack("<d", 1.0)                    # dbl at 0x140002030
    return bytes(data)


def build():
    text = build_text()
    data = build_data()
    headers_size = FILE_ALIGN
    text_size = -(-len(text) // FILE_ALIGN) * FILE_ALIGN
    data_size = FILE_ALIGN

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)
    image = bytearray(dos)
    image += b"PE\0\0"
    image += struct.pack("<HHIIIHH", 0x8664, 2, 0, 0, 0, 240, 0x0022)
    optional = bytearray()
    optional += struct.pack(
        "<HBBIIIII", 0x20B, 14, 0, text_size, data_size, 0, TEXT_RVA, TEXT_RVA
    )
    optional += struct.pack("<Q", IMAGE_BASE)
    optional += struct.pack(
        "<IIHHHHHHIIIIHHQQQQII",
        SECTION_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
        DATA_RVA + SECTION_ALIGN, headers_size, 0, 3, 0x8160,
        0x100000, 0x1000, 0x100000, 0x1000, 0, 16,
    )
    optional += bytes(16 * 8)
    assert len(optional) == 240
    image += optional

    def section(name, virtual_size, rva, raw_size, raw_offset, flags):
        return name.encode().ljust(8, b"\0") + struct.pack(
            "<IIIIIIHHI", virtual_size, rva, raw_size, raw_offset, 0, 0, 0, 0, flags
        )

    image += section(".text", len(text), TEXT_RVA, text_size, headers_size, 0x60000020)
    image += section(
        ".data", len(data), DATA_RVA, data_size, headers_size + text_size, 0xC0000040
    )
    image += bytes(headers_size - len(image))
    image += text.ljust(text_size, b"\0")
    image += data.ljust(data_size, b"\0")
    return bytes(image)


if __name__ == "__main__":
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "homespill_pe_x86_64.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
