#!/usr/bin/env python3
"""Builds pushan_vm_x86_64: a static x86-64 ELF whose `func` is a protector's
entry stub (`push imm; jmp vm_entry`) into a push/ret-dispatched stack-machine
interpreter in its own 64 KiB executable section `.vmp0`.  The bytecode sits in
a 256-byte block of seeded random filler (entropy above 7 bits/byte) and
computes `return arg0 == 0 ? 7 : arg0 + 5`; its JZ selects the next virtual PC
with a cmov.  Regenerate with `python3 pushan_vm_x86_64.py` (needs GNU as/ld)."""
import os, random, subprocess, tempfile
OP = {"LDCTX": 0x3c, "LDIMM": 0x9e, "ADD": 0x51, "EXIT": 0xd7, "JZ": 0x6a}
def prog():
    # return arg0 == 0 ? 7 : arg0 + 5
    b = []
    b += [OP["LDCTX"], 0]
    jz = len(b); b += [OP["JZ"], 0]
    b += [OP["LDCTX"], 0, OP["LDIMM"], 5, 0, 0, 0, OP["ADD"], OP["EXIT"]]
    tgt = len(b)
    b += [OP["LDIMM"], 7, 0, 0, 0, OP["EXIT"]]
    b[jz + 1] = tgt - (jz + 2)
    return b
rng = random.Random(7680)
code = prog()
pad = [rng.randrange(256) for _ in range(256)]
blk = pad[:64] + code + pad[64 + len(code):]
out = [".intel_syntax noprefix", ".section .text,\"ax\",@progbits", ".globl func", ".type func,@function", "func:",
       "    push 0x5a17", "    jmp vm_entry", ".size func, .-func",
       ".section .vmp0,\"ax\",@progbits", ".p2align 12", "vm_entry:",
       "    push rbx", "    push rbp", "    push rsi", "    push rdi", "    mov rbp, rsp",
       "    lea rsi, [rip+bytecode+64]",
       "dispatch:", "    movzx eax, byte ptr [rsi]", "    add rsi, 1", "    lea rdx, [rip+handlers]",
       "    push qword ptr [rdx+rax*8]", "    ret",
       "h_ldctx:", "    movzx eax, byte ptr [rsi]", "    add rsi, 1", "    push qword ptr [rbp+rax]", "    jmp dispatch",
       "h_ldimm:", "    movsxd rax, dword ptr [rsi]", "    add rsi, 4", "    push rax", "    jmp dispatch",
       "h_add:", "    pop rax", "    add qword ptr [rsp], rax", "    jmp dispatch",
       "h_jz:", "    pop rax", "    movsx rcx, byte ptr [rsi]", "    add rsi, 1", "    lea rcx, [rsi+rcx]",
       "    test rax, rax", "    cmovz rsi, rcx", "    jmp dispatch",
       "h_exit:", "    pop rax", "    mov rsp, rbp", "    pop rdi", "    pop rsi", "    pop rbp", "    pop rbx",
       "    add rsp, 8", "    ret",
       ".p2align 8", "handlers:"]
names = {OP["LDCTX"]: "h_ldctx", OP["LDIMM"]: "h_ldimm", OP["ADD"]: "h_add", OP["EXIT"]: "h_exit", OP["JZ"]: "h_jz"}
for i in range(256):
    out.append(f"    .quad {names.get(i, '0')}")
out += [".p2align 8", "bytecode:", "    .byte " + ",".join(str(x) for x in blk)]
out += [".p2align 4", "    .space 0x10000 - (. - vm_entry)", ""]
here = os.path.dirname(os.path.abspath(__file__))
with tempfile.TemporaryDirectory() as d:
    s, o = os.path.join(d, "vm.s"), os.path.join(d, "vm.o")
    open(s, "w").write("\n".join(out))
    subprocess.run(["as", "--64", "-o", o, s], check=True)
    subprocess.run(["ld", "-static", "-e", "func", "--discard-all", "--build-id=none", "-o",
                    os.path.join(here, "pushan_vm_x86_64"), o], check=True)
