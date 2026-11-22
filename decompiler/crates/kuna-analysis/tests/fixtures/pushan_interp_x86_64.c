/* pushan_interp_x86_64: in-function bytecode interpreters, the shape Tigress
   `Virtualize` gives a function (the interpreter is the function's own body,
   its virtual PC a local, its bytecode in the image), written by hand.

   Every interpreter runs the same virtual program, with a loop and a branch:

       s = 0;
       for (i = 0; i < 10; i++) { if (a < i) s = s + i; else s = s + a; }
       return s;

   sw_writable  switch dispatch on an opcode byte, bytecode in writable .data
   th_copied    threaded code (`goto *pc`): the program is an array of handler
                addresses and operands in writable data, copied into a 32 KiB
                local with memcpy before it runs, so gcc probes the stack page
                by page first
   th_inline    the same threaded code copied by an inline `rep movsq`
                (__builtin_memcpy), as gcc copies a smaller Tigress array
   sw_readonly  switch dispatch, bytecode in .rodata
   wv_woven     switch dispatch over two programs run round-robin through
                `pc[sel]` and `sp[sel]` with `register int sel`: the virtual
                program above and a bogus one that loops forever over its own
                variable
   sw_patched   switch dispatch whose bytecode the function patches with its
                argument when that is 0x55: the patched opcode decides a
                dispatch no exploration can, and the header must say so

   `./pushan_interp_x86_64 A...` prints the five results for a = argc (each
   46 for a = 1).

   Build (gcc 11, Ubuntu 22.04 defaults: PIE, stack-clash protection; the
   copy is a memcpy call, as gcc emits for Tigress's 81 KB array, rather than
   the inline `rep movsq` it would use for this one):
       gcc -O0 -fno-builtin-memcpy -o pushan_interp_x86_64 pushan_interp_x86_64.c */
#include <stdio.h>
#include <string.h>

enum { PUSH = 0x31, ARG, LDV, STV, ADD, LT, JZ, JMP, RET };

/* The virtual program; e, f, n and l are the branch offsets to E, F, N and L
   from the instruction after the branch, in the encoding's units. */
#define PROGRAM(X, e, f, n, l)                                                \
    X(PUSH, 0) X(STV, 0) X(PUSH, 0) X(STV, 1)                                 \
    /* L: */ X(LDV, 1) X(PUSH, 10) X(LT, 0) X(JZ, e)                          \
    X(ARG, 0) X(LDV, 1) X(LT, 0) X(JZ, f)                                     \
    X(LDV, 0) X(LDV, 1) X(ADD, 0) X(JMP, n)                                   \
    /* F: */ X(LDV, 0) X(ARG, 0) X(ADD, 0)                                    \
    /* N: */ X(STV, 0) X(LDV, 1) X(PUSH, 1) X(ADD, 0) X(STV, 1) X(JMP, l)     \
    /* E: */ X(LDV, 0) X(RET, 0)

/* One byte per opcode; PUSH takes a 4-byte immediate, LDV/STV/JZ/JMP one
   byte (a variable index or a branch offset in bytes). */
#define BYTES(op, v) op, BYTES_##op(v)
#define BYTES_PUSH(v) (v) & 0xff, ((v) >> 8) & 0xff, ((v) >> 16) & 0xff, ((v) >> 24) & 0xff,
#define BYTES_ARG(v)
#define BYTES_LDV(v) (v),
#define BYTES_STV(v) (v),
#define BYTES_ADD(v)
#define BYTES_LT(v)
#define BYTES_JZ(v) (unsigned char)(v),
#define BYTES_JMP(v) (unsigned char)(v),
#define BYTES_RET(v)
#define BYTE_PROGRAM PROGRAM(BYTES, 31, 7, 4, -41)

unsigned char sw_code[] = { BYTE_PROGRAM };
const unsigned char ro_code[] = { BYTE_PROGRAM };
unsigned char pt_code[] = { BYTE_PROGRAM };
/* Program 1: b = 1; L: b = b + 3; goto L; */
unsigned char wv_code[2][64] = {
    { BYTE_PROGRAM },
    { PUSH, 1, 0, 0, 0, STV, 3, LDV, 3, PUSH, 3, 0, 0, 0, ADD, STV, 3, JMP, (unsigned char)-12 },
};

#define SWITCH_BODY(PC, SP, VARS)                                             \
    switch (*PC) {                                                            \
    case PUSH: *++SP = *(int *)(PC + 1); PC += 5; break;                      \
    case ARG: *++SP = a; PC += 1; break;                                      \
    case LDV: *++SP = VARS[PC[1]]; PC += 2; break;                            \
    case STV: VARS[PC[1]] = *SP--; PC += 2; break;                            \
    case ADD: SP[-1] = SP[-1] + SP[0]; SP--; PC += 1; break;                  \
    case LT: SP[-1] = SP[-1] < SP[0]; SP--; PC += 1; break;                   \
    case JZ: PC += 2; if (!*SP--) PC += (signed char)PC[-1]; break;           \
    case JMP: PC += 2 + (signed char)PC[1]; break;                            \
    case RET: return *SP;                                                     \
    }

long sw_writable(long a)
{
    unsigned char *pc = sw_code;
    long stack[16], *sp = stack, vars[4];
    for (;;)
        SWITCH_BODY(pc, sp, vars)
}

long sw_readonly(long a)
{
    const unsigned char *pc = ro_code;
    long stack[16], *sp = stack, vars[4];
    for (;;)
        SWITCH_BODY(pc, sp, vars)
}

long sw_patched(long a)
{
    unsigned char *pc = pt_code;
    long stack[16], *sp = stack, vars[4];
    if (a == 0x55)
        pt_code[21] = (unsigned char)a;
    for (;;)
        SWITCH_BODY(pc, sp, vars)
}

long wv_woven(long a)
{
    unsigned char *pc[2];
    long stack[2][16], *sp[2], vars[8];
    register int sel;
    sel = 0; pc[sel] = wv_code[sel]; sp[sel] = stack[sel];
    sel = 1; pc[sel] = wv_code[sel]; sp[sel] = stack[sel];
    sel = 0;
    for (;;) {
        SWITCH_BODY(pc[sel], sp[sel], (vars + 4 * sel))
        sel = sel + 1 >= 2 ? 0 : sel + 1;
    }
}

/* Two slots per instruction: the handler's address and the operand (a
   branch offset counts slots). */
#define SLOTS(op, v) &&h_##op, (void *)(long)(v),

long th_copied(long a)
{
    static void *init[] = { PROGRAM(SLOTS, 34, 8, 6, -42) };
    void *code[4096];
    void **pc;
    long stack[16], *sp = stack, vars[4];
    memcpy(code, init, sizeof init);
    pc = code;
    goto **pc;
h_PUSH: *++sp = (long)pc[1]; pc += 2; goto **pc;
h_ARG: *++sp = a; pc += 2; goto **pc;
h_LDV: *++sp = vars[(long)pc[1]]; pc += 2; goto **pc;
h_STV: vars[(long)pc[1]] = *sp--; pc += 2; goto **pc;
h_ADD: sp[-1] = sp[-1] + sp[0]; sp--; pc += 2; goto **pc;
h_LT: sp[-1] = sp[-1] < sp[0]; sp--; pc += 2; goto **pc;
h_JZ: pc += 2; if (!*sp--) pc += (long)pc[-1]; goto **pc;
h_JMP: pc += 2 + (long)pc[1]; goto **pc;
h_RET: return *sp;
}

long th_inline(long a)
{
    static void *init[] = { PROGRAM(SLOTS, 34, 8, 6, -42) };
    void *code[64];
    void **pc;
    long stack[16], *sp = stack, vars[4];
    __builtin_memcpy(code, init, sizeof init);
    pc = code;
    goto **pc;
h_PUSH: *++sp = (long)pc[1]; pc += 2; goto **pc;
h_ARG: *++sp = a; pc += 2; goto **pc;
h_LDV: *++sp = vars[(long)pc[1]]; pc += 2; goto **pc;
h_STV: vars[(long)pc[1]] = *sp--; pc += 2; goto **pc;
h_ADD: sp[-1] = sp[-1] + sp[0]; sp--; pc += 2; goto **pc;
h_LT: sp[-1] = sp[-1] < sp[0]; sp--; pc += 2; goto **pc;
h_JZ: pc += 2; if (!*sp--) pc += (long)pc[-1]; goto **pc;
h_JMP: pc += 2 + (long)pc[1]; goto **pc;
h_RET: return *sp;
}

int main(int argc, char **argv)
{
    long a = argc;
    printf("%ld %ld %ld %ld %ld\n", sw_writable(a), sw_readonly(a), wv_woven(a), th_copied(a), th_inline(a));
    return 0;
}
