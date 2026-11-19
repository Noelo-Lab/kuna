/* (kuna) Option `globalwidth`: symbol-table data objects with no debug info.
 * Build: clang -O2 -nostdlib -ffreestanding -static -fno-pic -o globalwidth_x86_64 globalwidth_x86_64.c */
unsigned long g1; int g2; short g3[8]; char gbuf[32];
__attribute__((noinline)) int f(int i) { g2 = i; g3[i & 7] = (short)i; gbuf[i & 31] = 1; return (int)g1 + g2 + g3[1]; }
void _start(void) { volatile int k = f(*(volatile int*)0x1000); (void)k; for(;;){} }
