/* A Thumb export, an A32 assembly export with no .size, then static Thumb
   functions, built into a stripped armel shared object (see fixtures/README.md). */
#if PART == 1
int thumb_first(int x) { return x * 5 + 2; }
#elif PART == 2
__asm__(".syntax unified\n.arm\n.text\n.global asm_nosz\n.type asm_nosz, %function\nasm_nosz:\n\tadd r0, r0, #1\n\tbx lr\n");
#elif PART == 3
static int __attribute__((noinline)) tsum(int x) { int s = 0; for (int i = 0; i < x; i++) s += (i * i) ^ 5; return s; }
static int __attribute__((noinline)) tpop(unsigned x) { int c = 0; while (x) { c += x & 1; x >>= 1; } return c; }
int thumb_api2(int x) { return tsum(x) + tpop(x); }
#endif
