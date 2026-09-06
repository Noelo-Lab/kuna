#!/usr/bin/env python3
"""Regenerate `getpcthunk_i386` -- the vendored witness for `option calleepreserves`.

A minimal i386 ET_EXEC carrying, verbatim, function `sub_8049f20` of the crackme
`collide` (crackmes.one 5ab77f5833c5d40ad448c399) and the get-PC thunk at
0x8049f55 it calls:

    8049f29  mov edx,[ebp+0xc]     ; the second parameter
    8049f2c  call 0x8049f55        ; mov ebx,[esp]; ret -- writes EBX and nothing else
    8049f3e  mov [esp+0x8],edx     ; the third argument of the call below
    8049f42  mov edx,[ebp+0x8]     ; the first parameter

The only edit is the second CALL: the original targets the __lxstat PLT entry,
which this image does not have, so it is retargeted to an in-image
`xor eax,eax; ret` at 0x8049f60.  That callee writes no register x86gcc.cspec
marks <unaffected>, so it is also a negative control -- the rule declines there.
"""
import struct

CODE_VA = 0x8049F20
LOAD_VA = 0x8049000

fn = bytes.fromhex(
    "5589e583ec10895dfc8b550ce82400000081c36f010000"
    "c7042403000000895424088b550889542404e812e4ffff8b5dfc89ec5dc3"
)
thunk = bytes.fromhex("8b1c24c3")
callee = bytes.fromhex("31c0c3")

callee_va = 0x8049F60  # the first 16-byte boundary past the thunk
pad = callee_va - (CODE_VA + len(fn) + len(thunk))
assert fn[0x29] == 0xE8
fn = fn[:0x2A] + struct.pack("<i", callee_va - (0x8049F49 + 5)) + fn[0x2E:]
text = fn + thunk + b"\x90" * pad + callee

text_off = CODE_VA - LOAD_VA
shstr = b"\x00.text\x00.shstrtab\x00"
text_end = text_off + len(text)
shstr_off = (text_end + 15) & ~15
sh_off = (shstr_off + len(shstr) + 15) & ~15

img = bytearray(sh_off + 3 * 40)
img[text_off:text_end] = text
img[shstr_off:shstr_off + len(shstr)] = shstr
img[0:52] = struct.pack(
    "<16sHHIIIIIHHHHHH",
    b"\x7fELF\x01\x01\x01\x00" + b"\x00" * 8, 2, 3, 1,
    CODE_VA, 0x34, sh_off, 0, 52, 32, 1, 40, 3, 2,
)
img[0x34:0x54] = struct.pack("<IIIIIIII", 1, 0, LOAD_VA, LOAD_VA, text_end, text_end, 5, 0x1000)
img[sh_off + 40:sh_off + 80] = struct.pack("<10I", 1, 1, 0x6, CODE_VA, text_off, len(text), 0, 0, 16, 0)
img[sh_off + 80:sh_off + 120] = struct.pack("<10I", 7, 3, 0, 0, shstr_off, len(shstr), 0, 0, 1, 0)

with open("getpcthunk_i386", "wb") as f:
    f.write(bytes(img))
