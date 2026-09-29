#!/usr/bin/env python3
"""Generate `passthrough_callfwd_x86_64` -- a minimal x86-64 ELF whose
forwarders hand their incoming registers to a target by a CALL, not a JMP.

`passthrough` gives a function a parameter it forwards untouched to a callee
whose body reads it. A callee that forwards by a call (`sub rsp,8; call narrow;
add rsp,8; ret`) reads nothing before that call, where the callee-body walk
ends, so its callers never got the arguments it passes on (issue #749). The
walk now also counts what each direct call's target reads, for the bytes the
forwarder has not written by that call, up to three calls deep.

Regenerate with:

    python3 passthrough_callfwd_x86_64.py

Functions (16-byte aligned from 0x401000):

  narrow          lea rax,[rdi+rsi]; ret
  fwd             jmp narrow                                  forwards by a jump
  fwd_call        sub rsp,8; call narrow; add rsp,8; ret      forwards by a call
  fwd_call2       the same, calling fwd_call                  two calls deep
  fwd_clob        sub rsp,8; mov esi,5; call narrow; ...      forwards rdi only
  fwd_cond        test edx,edx; je L; <fwd_call's body>; L: xor eax,eax; ret
  fwd_ind         sub rsp,8; call [rip+0]; add rsp,8; ret     indirect: nothing
  cyc_a, cyc_b    sub rsp,8; call <the other>; add rsp,8; ret a call cycle
  pass_<f>        push rbx; call <f>; add rax,rax; pop rbx; ret
"""
import os
import struct

BASE = 0x401000
FUNCS = [
    ('narrow', '488d0437c3', []),
    ('fwd', 'e900000000', [(1, 'narrow')]),
    ('fwd_call', '4883ec08e8000000004883c408c3', [(5, 'narrow')]),
    ('fwd_call2', '4883ec08e8000000004883c408c3', [(5, 'fwd_call')]),
    ('fwd_clob', '4883ec08be05000000e8000000004883c408c3', [(10, 'narrow')]),
    ('fwd_cond', '85d2740e4883ec08e8000000004883c408c331c0c3', [(9, 'narrow')]),
    ('fwd_ind', '4883ec08ff15000000004883c408c3', []),
    ('cyc_a', '4883ec08e8000000004883c408c3', [(5, 'cyc_b')]),
    ('cyc_b', '4883ec08e8000000004883c408c3', [(5, 'cyc_a')]),
] + [('pass_' + f, '53e8000000004801c05bc3', [(2, f)])
     for f in ('fwd', 'fwd_call', 'fwd_call2', 'fwd_clob', 'fwd_cond', 'fwd_ind', 'cyc_a')]

code, syms, lens = bytearray(), {}, {}
for name, hexcode, _ in FUNCS:
    syms[name] = len(code)
    lens[name] = len(bytes.fromhex(hexcode))
    code += bytes.fromhex(hexcode)
    code += b'\xcc' * (-len(code) % 16)
for name, _, rels in FUNCS:
    for off, dst in rels:
        at = syms[name] + off
        code[at:at + 4] = struct.pack('<i', syms[dst] - (at + 4))

strtab = b'\0' + b''.join(n.encode() + b'\0' for n in syms)
symtab, o = bytearray(24), 1
for name, addr in syms.items():
    symtab += struct.pack('<IBBHQQ', o, 0x12, 0, 1, BASE + addr, lens[name])
    o += len(name) + 1
shstr = b'\0.text\0.symtab\0.strtab\0.shstrtab\0'
t_off = 0x1000
s_off = t_off + len(code)
st_off = s_off + len(symtab)
sh_off_str = st_off + len(strtab)
shoff = (sh_off_str + len(shstr) + 7) & ~7
eh = b'\x7fELF\x02\x01\x01' + bytes(9) + struct.pack(
    '<HHIQQQIHHHHHH', 2, 62, 1, BASE, 64, shoff, 0, 64, 56, 1, 64, 5, 4)
ph = struct.pack('<IIQQQQQQ', 1, 5, t_off, BASE, BASE, len(code), len(code), 0x1000)
sh = [bytes(64),
      struct.pack('<IIQQQQIIQQ', 1, 1, 6, BASE, t_off, len(code), 0, 0, 16, 0),
      struct.pack('<IIQQQQIIQQ', 7, 2, 0, 0, s_off, len(symtab), 3, 1, 8, 24),
      struct.pack('<IIQQQQIIQQ', 15, 3, 0, 0, st_off, len(strtab), 0, 0, 1, 0),
      struct.pack('<IIQQQQIIQQ', 23, 3, 0, 0, sh_off_str, len(shstr), 0, 0, 1, 0)]
out = bytearray(shoff + 64 * 5)
out[0:64], out[64:120] = eh, ph
out[t_off:t_off + len(code)] = code
out[s_off:s_off + len(symtab)] = symtab
out[st_off:st_off + len(strtab)] = strtab
out[sh_off_str:sh_off_str + len(shstr)] = shstr
out[shoff:] = b''.join(sh)
dest = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'passthrough_callfwd_x86_64')
with open(dest, 'wb') as fh:
    fh.write(out)
