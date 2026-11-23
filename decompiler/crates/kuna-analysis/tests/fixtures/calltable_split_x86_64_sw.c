/* (kuna) Fixture calltable_split_x86_64, file 3 of 3: a range-checked switch
 * whose table is followed in .rodata by `ftab`, a table of functions the
 * entry calls directly. */
extern volatile int sink;
typedef int (*fp)(int);
__attribute__((noinline)) int fa(int x) { sink = x; return x * 3 + 1; }
__attribute__((noinline)) int fb(int x) { sink = x + 1; return x * 5 + 2; }
__attribute__((noinline)) int fc(int x) { sink = x + 2; return x * 7 + 3; }
__attribute__((noinline)) int fd(int x) { sink = x + 3; return x * 9 + 4; }
__attribute__((noinline)) int fe(int x) { sink = x + 4; return x * 11 + 4; }
__attribute__((noinline)) int ff(int x) { sink = x + 5; return x * 13 + 4; }
__attribute__((noinline)) int sw(unsigned x) {
    switch (x) {
    case 0: sink = 10; return 1;
    case 1: sink = 13; return 8;
    case 2: sink = 16; return 15;
    case 3: sink = 19; return 22;
    case 4: sink = 22; return 29;
    case 5: sink = 25; return 36;
    case 6: sink = 28; return 43;
    case 7: sink = 31; return 50;
    default: return -1;
    }
}
const fp ftab[8] = {fa, fb, fc, fd, fe, ff, fa, fb};
__attribute__((noinline)) int vrun(unsigned i, int x) { if (i < 8) return ftab[i](x) + 1; return 0; }
