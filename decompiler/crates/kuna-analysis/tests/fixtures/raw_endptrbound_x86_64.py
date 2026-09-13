#!/usr/bin/env python3
"""Regenerate the headerless Win64 pointer-walk fixture for `endptrbound`."""

from pathlib import Path


# Mapped at 0x140001000 (`--raw-image --target x86:LE:64:default:windows`).
#
# walk @ 0x140001000: an 8-byte stack buffer built from two dword constants,
# walked bytewise to one past its end.  [rbp+0x2f] is both the loop bound and
# the object handed to push_back.
#   push rbp; push rsi; push rdi; lea rbp,[rsp-0x67]; sub rsp,0xa0
#   xor eax,eax; mov [rbp+0x2f],rax; mov [rbp+0x3f],rax; mov qword [rbp+0x47],0xf
#   mov dword [rbp+0x27],0x3e3c2f3e; mov dword [rbp+0x2b],0x2e383036
#   lea rsi,[rbp+0x27]
#   loop: movzx edx,byte [rsi]; lea rcx,[rbp+0x2f]; call push_back
#         inc rsi; lea rax,[rbp+0x2f]; cmp rsi,rax; jne loop
#   add rsp,0xa0; pop rdi; pop rsi; pop rbp; ret
WALK = (
    "555657488d6c24994881eca000000031c04889452f4889453f48c745470f0000"
    "00c745273e2f3c3ec7452b3630382e488d75270fb616488d4d2fe8c100000048"
    "ffc6488d452f4839c675e84881c4a00000005f5e5dc3"
)

# midpoint @ 0x140001080: the same buffer, but EBX counts 16 steps and the
# pointer is compared against [rbp+0x2f] only to skip one call -- a branch that
# stays inside the loop, so it is not the walk's bound.
#   ... lea rsi,[rbp+0x27]; mov ebx,16
#   loop: movzx edx,byte [rsi]; lea rax,[rbp+0x2f]; cmp rsi,rax; je skip
#         lea rcx,[rbp+0x2f]; call push_back
#   skip: inc rsi; dec ebx; jne loop
MIDPOINT = (
    "55565753488d6c24994881eca000000031c04889452f4889453f48c745470f00"
    "0000c745273e2f3c3ec7452b3630382e488d7527bb100000000fb616488d452f"
    "4839c67409488d4d2fe83200000048ffc6ffcb75e44881c4a00000005b5f5e5d"
    "c3"
)

# push_back @ 0x140001100: mov rax,[rcx+0x10]; mov [rcx+rax],dl; inc rax;
# mov [rcx+0x10],rax; ret
PUSH_BACK = "488b411088140148ffc048894110c3"


def image() -> bytes:
    out = bytearray(b"\xcc" * 0x100)
    walk, midpoint = bytes.fromhex(WALK), bytes.fromhex(MIDPOINT)
    out[0 : len(walk)] = walk
    out[0x80 : 0x80 + len(midpoint)] = midpoint
    return bytes(out) + bytes.fromhex(PUSH_BACK)


if __name__ == "__main__":
    Path(__file__).with_suffix("").write_bytes(image())
