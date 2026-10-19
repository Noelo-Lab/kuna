/* clang --target=thumbv7em-none-eabi -mcpu=cortex-m4 -O2 -c callerargs_reltab_thumb.c
   ld.lld -o callerargs_reltab_thumb callerargs_reltab_thumb.o --nmagic -Ttext=0x20000 -e f0000 --strip-debug
   cb is called with bytes and also through offsets relative to a table and to another function, with a
   whole word: kuna resolves the indirect call to cb, so cb keeps its width. */
typedef unsigned char UC; typedef unsigned U;
#define NI __attribute__((noinline, used))
typedef U (*fp_t)(U, U);
NI static U cb(U v, U k) { return v * 3u + (v >> 9) + k; }
NI static U other(U v, U k) { return v ^ k; }
/* a two-entry table of offsets relative to the table itself, and one relative to another function */
__asm__(".section .rodata.rel,\"a\"\n.p2align 2\n.globl reltab\nreltab:\n.word cb - reltab\n.word other - reltab\n"
        ".section .data.rel2,\"aw\"\n.p2align 2\n.globl deltas\ndeltas:\n.word cb - other\n.word 0\n.text\n");
extern const int reltab[]; extern int deltas[];
NI U call_rel(U x, U i) { fp_t f = (fp_t)((const char *)reltab + reltab[i & 1]); return f(x, 7); }
NI U call_delta(U x, U i) { fp_t f = (fp_t)((char *)other + deltas[i & 1]); return f(x, 9); }
NI U f0000(UC *p, U x, U y) { return cb((UC)x, y) + cb((UC)(x >> 8), y) + call_rel(x * 1000u + y, 0) + call_delta(x * 1000u + y, 0); }
