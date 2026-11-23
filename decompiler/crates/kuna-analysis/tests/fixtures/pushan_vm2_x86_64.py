#!/usr/bin/env python3
"""Builds pushan_vm2_x86_64: a static x86-64 ELF with one stack-machine
interpreter in its own 64 KiB executable section `.vmp0` and several protected
functions, each an entry stub `push key; jmp vm_entry` whose key selects its
bytecode.  The bytecodes are hand-written programs whose correct decompilation
is known, aimed at the devirtualizer's soundness: loops whose bounds and exit
tests need more than two observed counter values, a branch on memory written
earlier, correlated branches, two branches to one label, values live across a
native call, a local a call writes through its address, a read-only table and
a stack array indexed by a loop counter, a branch on a mutable global, stores
through a pointer argument on two paths, a loop that grows the stack, a
virtual jump through an argument, two virtual exits into one native
epilogue, one of them an instruction ahead of it, a virtual branch that
selects its target arithmetically on a flag a join merged and a comparison,
arguments read at addresses rebuilt through mixed boolean-arithmetic
identities with an exit behind an opaque predicate on the stack pointer, and
a call that writes through a pointer passed on the stack and through a pointer
argument past its first 16 bytes, a call whose register arguments are the
caller's own first argument, a constant and the address of a local the
callee writes, and a call made while the interpreter keeps a copy of a loop
counter below the stack pointer, in the callee's frame.
Every program sits
in its own 256-byte block of seeded random filler (entropy above 7 bits/byte).
Regenerate with `python3 pushan_vm2_x86_64.py` (needs GNU as/ld)."""
import os, random, subprocess, tempfile

OPS = ["LDCTX", "STCTX", "LEAV", "LDIMM", "ADD", "SUB", "MUL", "LT", "EQ", "LD", "ST", "JZ", "JMP", "JMPI", "CALL", "CALLP", "EXIT"]
rng = random.Random(7681)
codes = rng.sample(range(1, 256), len(OPS))
OP = dict(zip(OPS, codes))
OP["LEAVE"] = min(set(range(1, 256)) - set(codes))
OP["SEL"] = min(set(range(1, 256)) - set(codes) - {OP["LEAVE"]})
OP["LDCTXM"], OP["EXITO"], OP["EXITS"] = sorted(set(range(1, 256)) - set(codes) - {OP["LEAVE"], OP["SEL"]})[:3]
OP["CALL7"] = min(set(range(1, 256)) - set(OP.values()))
OP["CALL3"] = min(set(range(1, 256)) - set(OP.values()))
OP["CALLS"] = min(set(range(1, 256)) - set(OP.values()))
A0, A1 = 0, 8


def V(i):
    return (-0x80 + 8 * i) & 0xff


def asm(lines):
    """Two passes over (op, arg) tuples and label strings; JZ/JMP take a label."""
    def size(item):
        if isinstance(item, str):
            return 0
        op = item[0]
        return {"LDCTX": 2, "STCTX": 2, "LEAV": 2, "LDCTXM": 2, "LDIMM": 5, "JZ": 2, "JMP": 2, "SEL": 3}.get(op, 1)
    labels, pc = {}, 0
    for it in lines:
        if isinstance(it, str):
            labels[it] = pc
        pc += size(it)
    out = []
    for it in lines:
        if isinstance(it, str):
            continue
        op, *args = it
        out.append(OP[op])
        if op in ("LDCTX", "STCTX", "LEAV", "LDCTXM"):
            out.append(args[0] & 0xff)
        elif op == "LDIMM":
            out += list((args[0] & 0xffffffff).to_bytes(4, "little"))
        elif op in ("JZ", "JMP"):
            d = labels[args[0]] - (len(out) + 1)
            assert -128 <= d < 128
            out.append(d & 0xff)
        elif op == "SEL":
            end = len(out) + 2
            for lab in args:
                d = labels[lab] - end
                assert -128 <= d < 128
                out.append(d & 0xff)
    return out


def ldimm_sym(name):
    return ("LDIMM", name)


PROGRAMS = {
    # s = 0; for (i = 0; i < 10; i++) s += i; return s;
    "f_sum": [("LDIMM", 0), ("STCTX", V(0)), ("LDIMM", 0), ("STCTX", V(1)),
              "L", ("LDCTX", V(1)), ("LDIMM", 10), ("LT",), ("JZ", "E"),
              ("LDCTX", V(0)), ("LDCTX", V(1)), ("ADD",), ("STCTX", V(0)),
              ("LDCTX", V(1)), ("LDIMM", 1), ("ADD",), ("STCTX", V(1)), ("JMP", "L"),
              "E", ("LDCTX", V(0)), ("EXIT",)],
    # s = 0; i = 0; do { s += a0; i++; } while (i != 7); return s;
    "f_eq": [("LDIMM", 0), ("STCTX", V(0)), ("LDIMM", 0), ("STCTX", V(1)),
             "L", ("LDCTX", V(0)), ("LDCTX", A0), ("ADD",), ("STCTX", V(0)),
             ("LDCTX", V(1)), ("LDIMM", 1), ("ADD",), ("STCTX", V(1)),
             ("LDCTX", V(1)), ("LDIMM", 7), ("EQ",), ("JZ", "L"),
             ("LDCTX", V(0)), ("EXIT",)],
    # i = 0; while (i < 0x10000000000) i++; return i;
    "f_big": [("LDIMM", 0), ("STCTX", V(1)),
              "L", ("LDCTX", V(1)), ("LDIMM", 0x100000), ("LDIMM", 0x100000), ("MUL",), ("LT",), ("JZ", "E"),
              ("LDCTX", V(1)), ("LDIMM", 1), ("ADD",), ("STCTX", V(1)), ("JMP", "L"),
              "E", ("LDCTX", V(1)), ("EXIT",)],
    # v = a0 < 5 (through its address); return v ? 1 : 2;
    "f_mem": [("LDCTX", A0), ("LDIMM", 5), ("LT",), ("LEAV", V(2)), ("ST",),
              ("LEAV", V(2)), ("LD",), ("JZ", "E"),
              ("LDIMM", 1), ("EXIT",),
              "E", ("LDIMM", 2), ("EXIT",)],
    # x = a0 < 5 ? 1 : 2; return a0 < 5 ? x + 10 : x + 20;   (11 or 22)
    "f_corr": [("LDCTX", A0), ("LDIMM", 5), ("LT",), ("JZ", "E1"),
               ("LDIMM", 1), ("STCTX", V(0)), ("JMP", "J"),
               "E1", ("LDIMM", 2), ("STCTX", V(0)),
               "J", ("LDCTX", A0), ("LDIMM", 5), ("LT",), ("JZ", "E2"),
               ("LDCTX", V(0)), ("LDIMM", 10), ("ADD",), ("EXIT",),
               "E2", ("LDCTX", V(0)), ("LDIMM", 20), ("ADD",), ("EXIT",)],
    # v = a0 * 3; r = ext(v); return v + r;
    "f_call": [("LDCTX", A0), ("LDIMM", 3), ("MUL",), ("STCTX", V(0)),
               ("LDCTX", V(0)), ("CALL",), ("STCTX", V(1)),
               ("LDCTX", V(0)), ("LDCTX", V(1)), ("ADD",), ("EXIT",)],
    # for (i = 0; i < 4; i++) if (tbl[i] >= 100) return i; return -1;   (tbl = {1, 2, 300, 4})
    "f_tbl": [("LDIMM", 0), ("STCTX", V(1)),
              "L", ("LDCTX", V(1)), ("LDIMM", 4), ("LT",), ("JZ", "N"),
              ldimm_sym("tbl"), ("LDCTX", V(1)), ("LDIMM", 8), ("MUL",), ("ADD",), ("LD",),
              ("LDIMM", 100), ("LT",), ("JZ", "F"),
              ("LDCTX", V(1)), ("LDIMM", 1), ("ADD",), ("STCTX", V(1)), ("JMP", "L"),
              "F", ("LDCTX", V(1)), ("EXIT",),
              "N", ("LDIMM", -1), ("EXIT",)],
    # a[0..3] = 0; for (i = 0; i < 4; i++) a[i] = 2 * i; return a[2] == 4;
    "f_arr": [("LDIMM", 0), ("STCTX", V(4)), ("LDIMM", 0), ("STCTX", V(5)),
              ("LDIMM", 0), ("STCTX", V(6)), ("LDIMM", 0), ("STCTX", V(7)),
              ("LDIMM", 0), ("STCTX", V(1)),
              "L", ("LDCTX", V(1)), ("LDIMM", 4), ("LT",), ("JZ", "D"),
              ("LDCTX", V(1)), ("LDIMM", 2), ("MUL",),
              ("LEAV", V(4)), ("LDCTX", V(1)), ("LDIMM", 8), ("MUL",), ("ADD",), ("ST",),
              ("LDCTX", V(1)), ("LDIMM", 1), ("ADD",), ("STCTX", V(1)), ("JMP", "L"),
              "D", ("LDCTX", V(6)), ("LDIMM", 4), ("EQ",), ("JZ", "Z"),
              ("LDIMM", 1), ("EXIT",),
              "Z", ("LDIMM", 0), ("EXIT",)],
    # return g_flag ? 1 : 2;   (g_flag is a writable global, 0 in the file)
    "f_glob": [ldimm_sym("g_flag"), ("LD",), ("JZ", "Z"),
               ("LDIMM", 1), ("EXIT",),
               "Z", ("LDIMM", 2), ("EXIT",)],
    # g_side = 0; ext(5); return g_side ? 2 : 1;   (ext writes g_side)
    "f_gcall": [("LDIMM", 0), ldimm_sym("g_side"), ("ST",),
                ("LDIMM", 5), ("CALL",), ("STCTX", V(0)),
                ldimm_sym("g_side"), ("LD",), ("JZ", "Z"),
                ("LDIMM", 2), ("EXIT",),
                "Z", ("LDIMM", 1), ("EXIT",)],
    # if (a1) *a0 = 1; else *a0 = 2; return *a0 == 1 ? 10 : 20;
    "f_ptr": [("LDCTX", A1), ("JZ", "E"),
              ("LDIMM", 1), ("LDCTX", A0), ("ST",), ("JMP", "J"),
              "E", ("LDIMM", 2), ("LDCTX", A0), ("ST",),
              "J", ("LDCTX", A0), ("LD",), ("LDIMM", 1), ("EQ",), ("JZ", "T"),
              ("LDIMM", 10), ("EXIT",),
              "T", ("LDIMM", 20), ("EXIT",)],
    # if (!a0) goto X; if (!a1) goto X; return 3; X: return 4;   (two branches to one label)
    "f_join": [("LDCTX", A0), ("JZ", "X"), ("LDCTX", A1), ("JZ", "X"),
               ("LDIMM", 3), ("EXIT",),
               "X", ("LDIMM", 4), ("EXIT",)],
    # v = 5; setv(&v); return v == 5 ? 1 : 2;   (setv writes through its argument)
    "f_callp": [("LDIMM", 5), ("STCTX", V(3)), ("LEAV", V(3)), ("CALLP",), ("STCTX", V(0)),
                ("LDCTX", V(3)), ("LDIMM", 5), ("EQ",), ("JZ", "T"),
                ("LDIMM", 1), ("EXIT",),
                "T", ("LDIMM", 2), ("EXIT",)],
    # i = 0; do { push 7 (left on the stack); i++; } while (i < 10); return i;
    "f_deep": [("LDIMM", 0), ("STCTX", V(1)),
               "L", ("LDIMM", 7),
               ("LDCTX", V(1)), ("LDIMM", 1), ("ADD",), ("STCTX", V(1)),
               ("LDCTX", V(1)), ("LDIMM", 10), ("LT",), ("JZ", "D"), ("JMP", "L"),
               "D", ("LDCTX", V(1)), ("EXIT",)],
    # goto *(pc + a0): a virtual jump through an argument nothing bounds
    "f_ijmp": [("LDCTX", A0), ("JMPI",), ("LDIMM", 1), ("EXIT",)],
    # *a0 = 0; while (*a0 < 10) *a0 = *a0 + 1; return *a0;
    "f_ploop": [("LDIMM", 0), ("LDCTX", A0), ("ST",),
                "L", ("LDCTX", A0), ("LD",), ("LDIMM", 10), ("LT",), ("JZ", "D"),
                ("LDCTX", A0), ("LD",), ("LDIMM", 1), ("ADD",), ("LDCTX", A0), ("ST",), ("JMP", "L"),
                "D", ("LDCTX", A0), ("LD",), ("EXIT",)],
    # return fin(a0 ? a0 + 5 : 7 + 1);   (each arm leaves the VM for the stub's native tail:
    # the else arm at its `add rax, 1`, the then arm one instruction later)
    "f_epi": [("LDCTX", A0), ("JZ", "Z"),
              ("LDCTX", A0), ("LDIMM", 5), ("ADD",), ldimm_sym("f_epi_tail"), ("LEAVE",),
              "Z", ("LDIMM", 7), ldimm_sym("f_epi_inc"), ("LEAVE",)],
    # x = a0 < 5 (as 1 or 0 on two paths that join); c = a1 < 3;
    # if (x) { if (c) return 2; return 1; } return 3;   (SEL picks its target
    # arithmetically from the low bit of x and from c, as VMProtect's vJCC does)
    "f_sel": [("LDCTX", A0), ("LDIMM", 5), ("LT",), ("JZ", "E"),
              ("LDIMM", 1), ("STCTX", V(0)), ("JMP", "J"),
              "E", ("LDIMM", 0), ("STCTX", V(0)),
              "J", ("LDCTX", A1), ("LDIMM", 3), ("LT",), ("LDCTX", V(0)), ("SEL", "T", "F"),
              ("LDIMM", 1), ("EXIT",),
              "T", ("LDIMM", 2), ("EXIT",),
              "F", ("LDIMM", 3), ("EXIT",)],
    # return a0 + a1;   (Themida-style handlers: each argument is read at an
    # address rebuilt from the frame pointer and rdx through a mixed
    # boolean-arithmetic identity, and the exit tests an opaque predicate on
    # the stack pointer whose other arm jumps to `fin`)
    "f_mba": [("LDCTXM", A0), ("LDCTXM", A1), ("ADD",), ("EXITO",)],
    # return a0;   (but the exit tests bit 16 of the stack pointer, which no
    # identity decides, and its other arm jumps to `fin` with the
    # interpreter's frame on the stack)
    "f_spbit": [("LDCTX", A0), ("EXITS",)],
    # v = 5; x = a0[4]; ext7(a0, 0, 0, 0, 0, 0, &v);
    # if (v != 5) return 1; if (a0[4] != x) return 2; return 3;
    # (ext7 writes v through its seventh argument, passed on the stack, and
    # a0[4] through its first, 32 bytes past the pointer)
    "f_out": [("LDIMM", 5), ("STCTX", V(3)),
              ("LDCTX", A0), ("LDIMM", 32), ("ADD",), ("LD",), ("STCTX", V(1)),
              ("LDCTX", A0), ("LEAV", V(3)), ("CALL7",), ("STCTX", V(0)),
              ("LDCTX", V(3)), ("LDIMM", 5), ("EQ",), ("JZ", "A"),
              ("LDCTX", A0), ("LDIMM", 32), ("ADD",), ("LD",), ("LDCTX", V(1)), ("EQ",), ("JZ", "B"),
              ("LDIMM", 3), ("EXIT",),
              "A", ("LDIMM", 1), ("EXIT",),
              "B", ("LDIMM", 2), ("EXIT",)],
    # v = 5; ext3(a0, 1, &v); return v == 5 ? 2 : 1;   (ext3 stores its second
    # argument through its third, so v is 1 and the function returns 1)
    "f_arg": [("LDIMM", 5), ("STCTX", V(3)),
              ("LDCTX", A0), ("LDIMM", 1), ("LEAV", V(3)), ("CALL3",), ("STCTX", V(0)),
              ("LDCTX", V(3)), ("LDIMM", 5), ("EQ",), ("JZ", "T"),
              ("LDIMM", 2), ("EXIT",),
              "T", ("LDIMM", 1), ("EXIT",)],
    # i = 0; for (;;) { ext(7); if (!(i < 3)) break; i++; } return i;   (the
    # call handler also copies i 0x100 bytes below the stack pointer, where
    # the callee's frame lies, as an interpreter leaves copies of its context
    # behind; the test after the call reads i from the context)
    "f_below": [("LDIMM", 0), ("STCTX", V(1)),
                "L", ("LDIMM", 7), ("CALLS",), ("STCTX", V(0)),
                ("LDCTX", V(1)), ("LDIMM", 3), ("LT",), ("JZ", "E"),
                ("LDCTX", V(1)), ("LDIMM", 1), ("ADD",), ("STCTX", V(1)), ("JMP", "L"),
                "E", ("LDCTX", V(1)), ("EXIT",)],
}

NAMES = list(PROGRAMS)
DATA = {"tbl": 0x0, "g_flag": 0x0, "g_side": 0x0, "f_epi_tail": 0x0, "f_epi_inc": 0x0}
LOCAL = {"f_epi_tail", "f_epi_inc"}
STUB_PRE = {"f_epi": ["push rbx"]}
STUB_TAIL = {"f_epi": ["f_epi_inc:", "    add rax, 1", "f_epi_tail:", "    mov rdi, rax", "    call fin",
                       "    pop rbx", "    ret"]}


def build(addrs):
    blocks = []
    for name in NAMES:
        items = [(it[0], addrs[it[1]]) if isinstance(it, tuple) and it[0] == "LDIMM" and isinstance(it[1], str) else it
                 for it in PROGRAMS[name]]
        code = asm(items)
        assert len(code) <= 192, name
        pad = [rng.randrange(256) for _ in range(256)]
        blocks.append(pad[:64] + code + pad[64 + len(code):])
    return blocks


def source(blocks):
    out = [".intel_syntax noprefix", ".section .text,\"ax\",@progbits"]
    for i, name in enumerate(NAMES):
        out += [f".globl {name}", f".type {name},@function", f"{name}:"]
        out += ["    " + x for x in STUB_PRE.get(name, [])]
        out += [f"    push {i * 256 + 64}", "    jmp vm_entry"] + STUB_TAIL.get(name, []) + [f".size {name}, .-{name}"]
    out += [".globl ext", ".type ext,@function", "ext:",
            "    mov qword ptr [rip+g_side], rdi", "    lea rax, [rdi+rdi+1]", "    ret", ".size ext, .-ext",
            ".globl setv", ".type setv,@function", "setv:",
            "    mov qword ptr [rdi], 9", "    xor eax, eax", "    ret", ".size setv, .-setv",
            ".globl fin", ".type fin,@function", "fin:",
            "    lea rax, [rdi+rdi*2]", "    ret", ".size fin, .-fin",
            ".globl ext7", ".type ext7,@function", "ext7:",
            "    mov rax, qword ptr [rsp+8]", "    mov qword ptr [rax], 9", "    mov qword ptr [rdi+32], 7",
            "    xor eax, eax", "    ret", ".size ext7, .-ext7",
            ".globl ext3", ".type ext3,@function", "ext3:",
            "    mov qword ptr [rdx], rsi", "    mov rax, rdi", "    ret", ".size ext3, .-ext3",
            ".section .rodata,\"a\",@progbits", ".p2align 3", ".globl tbl", "tbl:", "    .quad 1, 2, 300, 4",
            ".section .data,\"aw\",@progbits", ".p2align 3", ".globl g_flag", "g_flag:", "    .quad 0",
            ".globl g_side", "g_side:", "    .quad 0",
            ".section .vmp0,\"ax\",@progbits", ".p2align 12", "vm_entry:",
            "    push rbx", "    push rbp", "    push rsi", "    push rdi", "    mov rbp, rsp", "    sub rsp, 0x100",
            "    mov rsi, qword ptr [rbp+32]", "    lea rax, [rip+bytecode]", "    add rsi, rax",
            "dispatch:", "    movzx eax, byte ptr [rsi]", "    add rsi, 1", "    lea rdx, [rip+handlers]",
            "    push qword ptr [rdx+rax*8]", "    ret",
            "h_ldctx:", "    movsx rax, byte ptr [rsi]", "    add rsi, 1", "    push qword ptr [rbp+rax]", "    jmp dispatch",
            "h_stctx:", "    movsx rax, byte ptr [rsi]", "    add rsi, 1", "    pop qword ptr [rbp+rax]", "    jmp dispatch",
            "h_leav:", "    movsx rax, byte ptr [rsi]", "    add rsi, 1", "    lea rcx, [rbp+rax]", "    push rcx", "    jmp dispatch",
            "h_ldimm:", "    movsxd rax, dword ptr [rsi]", "    add rsi, 4", "    push rax", "    jmp dispatch",
            "h_add:", "    pop rax", "    add qword ptr [rsp], rax", "    jmp dispatch",
            "h_sub:", "    pop rax", "    sub qword ptr [rsp], rax", "    jmp dispatch",
            "h_mul:", "    pop rax", "    pop rcx", "    imul rax, rcx", "    push rax", "    jmp dispatch",
            "h_lt:", "    pop rax", "    pop rcx", "    xor edx, edx", "    cmp rcx, rax", "    setl dl", "    push rdx", "    jmp dispatch",
            "h_eq:", "    pop rax", "    pop rcx", "    xor edx, edx", "    cmp rcx, rax", "    sete dl", "    push rdx", "    jmp dispatch",
            "h_ld:", "    pop rax", "    push qword ptr [rax]", "    jmp dispatch",
            "h_st:", "    pop rax", "    pop rcx", "    mov qword ptr [rax], rcx", "    jmp dispatch",
            "h_jz:", "    pop rax", "    movsx rcx, byte ptr [rsi]", "    add rsi, 1", "    lea rcx, [rsi+rcx]",
            "    test rax, rax", "    cmovz rsi, rcx", "    jmp dispatch",
            "h_jmp:", "    movsx rcx, byte ptr [rsi]", "    add rsi, 1", "    add rsi, rcx", "    jmp dispatch",
            "h_jmpi:", "    pop rcx", "    add rsi, rcx", "    jmp dispatch",
            "h_call:", "    pop rdi", "    push rsi", "    call ext", "    pop rsi", "    push rax", "    jmp dispatch",
            "h_callp:", "    pop rdi", "    push rsi", "    call setv", "    pop rsi", "    push rax", "    jmp dispatch",
            "h_exit:", "    pop rax", "    mov rsp, rbp", "    pop rdi", "    pop rsi", "    pop rbp", "    pop rbx",
            "    add rsp, 8", "    ret",
            "h_leave:", "    pop rcx", "    pop rax", "    mov rsp, rbp", "    pop rdi", "    pop rsi", "    pop rbp",
            "    pop rbx", "    add rsp, 8", "    jmp rcx",
            "h_sel:", "    pop rax", "    pop rdx", "    movsx rcx, byte ptr [rsi]", "    movsx r8, byte ptr [rsi+1]",
            "    add rsi, 2", "    and eax, 1", "    neg rax", "    neg rdx", "    and rcx, rdx", "    and rcx, rax",
            "    not rax", "    and r8, rax", "    add rsi, rcx", "    add rsi, r8", "    jmp dispatch",
            # y = rbp + disp as (y & M) | (N - (~y & N)) with N = ~M, plus
            # (rdx ^ rbx) + 2 * (rdx & rbx) - rdx - rbx, which is 0 whatever
            # the two registers hold
            "h_ldctxm:", "    movsx rax, byte ptr [rsi]", "    add rsi, 1", "    lea rcx, [rbp+rax]",
            "    movabs r8, 0x5a3cc3a5f00f0ff0", "    mov rax, rcx", "    and rax, r8", "    not rcx",
            "    movabs r9, 0xa5c33c5a0ff0f00f", "    and rcx, r9", "    sub r9, rcx", "    or rax, r9",
            "    mov r10, rdx", "    xor r10, rbx", "    mov r11, rdx", "    and r11, rbx", "    add r11, r11",
            "    add r10, r11", "    sub r10, rdx", "    sub r10, rbx", "    add rax, r10",
            "    push qword ptr [rax]", "    jmp dispatch",
            # the exit compares the stack pointer with its own rebuilt form,
            # mixed with the same zero over the result and rbx
            "h_exito:", "    pop rax", "    mov rcx, rsp", "    movabs r8, 0x5a3cc3a5f00f0ff0", "    mov rdx, rcx",
            "    and rdx, r8", "    not rcx", "    movabs r9, 0xa5c33c5a0ff0f00f", "    and rcx, r9", "    sub r9, rcx",
            "    or rdx, r9", "    mov r10, rax", "    xor r10, rbx", "    mov r11, rax", "    and r11, rbx",
            "    add r11, r11", "    add r10, r11", "    sub r10, rax", "    sub r10, rbx", "    add rdx, r10",
            "    cmp rdx, rsp", "    jne h_bogus",
            "    mov rsp, rbp", "    pop rdi", "    pop rsi", "    pop rbp", "    pop rbx", "    add rsp, 8", "    ret",
            "h_exits:", "    pop rax", "    test esp, 0x10000", "    jnz h_bogus",
            "    mov rsp, rbp", "    pop rdi", "    pop rsi", "    pop rbp", "    pop rbx", "    add rsp, 8", "    ret",
            "h_bogus:", "    jmp fin",
            "h_call7:", "    pop rax", "    pop rdi", "    push rsi", "    push rax", "    xor esi, esi", "    xor edx, edx",
            "    xor ecx, ecx", "    xor r8d, r8d", "    xor r9d, r9d", "    call ext7", "    add rsp, 8", "    pop rsi",
            "    push rax", "    jmp dispatch",
            ".p2align 8", "handlers:"]
    handler = {c: "h_" + o.lower() for o, c in OP.items()}
    for i in range(256):
        out.append(f"    .quad {handler.get(i, '0')}")
    out += [".p2align 8", "bytecode:"]
    for b in blocks:
        out.append("    .byte " + ",".join(str(x) for x in b))
    # handlers added after the first programs sit past the bytecode, so the
    # handler table and the bytecode keep their addresses
    out += ["h_call3:", "    pop rdx", "    pop rax", "    pop rdi", "    mov rbx, rsi", "    mov rsi, rax",
            "    call ext3", "    mov rsi, rbx", "    push rax", "    jmp dispatch",
            "h_calls:", "    pop rdi", f"    mov rax, qword ptr [rbp-{0x100 - V(1):#x}]", "    mov qword ptr [rsp-0x100], rax",
            "    mov rbx, rsi", "    call ext", "    mov rsi, rbx", "    push rax", "    jmp dispatch"]
    out += [".p2align 4", "    .space 0x10000 - (. - vm_entry)", ""]
    return "\n".join(out)


def link(d, src, dst, discard=True):
    s, o = os.path.join(d, "vm.s"), os.path.join(d, "vm.o")
    open(s, "w").write(src)
    subprocess.run(["as", "--64", "-o", o, s], check=True)
    subprocess.run(["ld", "-static", "-e", NAMES[0]] + (["--discard-all"] if discard else []) + ["--build-id=none", "-o", dst, o], check=True)


def symbols(path):
    nm = subprocess.run(["nm", path], check=True, capture_output=True, text=True).stdout
    found = {}
    for line in nm.splitlines():
        parts = line.split()
        if len(parts) == 3:
            found[parts[2]] = int(parts[0], 16)
    return found


here = os.path.dirname(os.path.abspath(__file__))
dst = os.path.join(here, "pushan_vm2_x86_64")
with tempfile.TemporaryDirectory() as d:
    state = rng.getstate()
    probe = os.path.join(d, "probe")
    link(d, source(build({k: 0 for k in DATA})), probe, discard=False)
    nm = symbols(probe)
    addrs = {k: nm[k] for k in DATA}
    rng.setstate(state)
    link(d, source(build(addrs)), dst)
    assert all(symbols(dst)[k] == addrs[k] for k in DATA if k not in LOCAL)
