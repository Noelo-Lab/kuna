/* clang --target=thumbv7em-none-eabi -mcpu=cortex-m4 -O2 -c callerargs_selfrel_thumb.c
   ld.lld -o callerargs_selfrel_thumb callerargs_selfrel_thumb.o --nmagic -Ttext=0x20000 -e f0000 --strip-debug
   cb is called with bytes and also through a table of self-relative words (target - &word) indexed by a
   value kuna cannot bound, with a whole word: the table word plus its own address is cb, so cb keeps its width. */
typedef unsigned char UC; typedef unsigned U;
#define NI __attribute__((noinline, used))
typedef U (*fp_t)(U, U);
NI static U cb(U v, U k) { return v * 3u + (v >> 9) + k; }
/* self-relative words: each entry holds target - &entry */
__asm__(".section .rodata.rel,\"a\"\n.p2align 2\n.globl reltab\nreltab:\n.word cb - .\n.word cb - .\n.word cb - .\n.text\n");
extern const int reltab[];
NI U call_rel(U x, const U *idx) { U i = *idx; const char *slot = (const char *)&reltab[i]; fp_t f = (fp_t)(slot + *(const int *)slot); return f(x, 7); }
NI U f0000(UC *p, U x, U y) { U i = y % 3; return cb((UC)x, y) + cb((UC)(x >> 8), y) + call_rel(x * 1000u + y, &i); }
