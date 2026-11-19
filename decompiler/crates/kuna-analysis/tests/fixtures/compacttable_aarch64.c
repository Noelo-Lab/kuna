/* (kuna) Compact jump tables in the reference walk: clang -O2 lowers `dispatch`'s dense switch to
 * an AArch64 compact table (ldrb; adr; add x, x, w, lsl #2; br). Each case
 * calls its own function, which only the table reaches.
 * Build: clang --target=aarch64-linux-gnu -O2 -nostdlib -ffreestanding -static
 *        -fuse-ld=lld -o compacttable_aarch64 compacttable_aarch64.c */
__attribute__((noinline)) int c0(int x) { return x * 3 + 1; }
__attribute__((noinline)) int c1(int x) { return x ^ 0x55; }
__attribute__((noinline)) int c2(int x) { return x - 7; }
__attribute__((noinline)) int c3(int x) { return x << 2; }
__attribute__((noinline)) int c4(int x) { return x / 5; }
__attribute__((noinline)) int c5(int x) { return ~x; }

__attribute__((noinline)) int dispatch(unsigned op, int x)
{
    switch (op) {
    case 0: return c0(x) + 1;
    case 1: return c1(x) + 2;
    case 2: return c2(x) + 3;
    case 3: return c3(x) + 4;
    case 4: return c4(x) + 5;
    case 5: return c5(x) + 6;
    case 6: return x;
    case 7: return c0(x) * 2;
    case 8: return c1(x) * 3;
    case 9: return c2(x) * 4;
    case 10: return c3(x) * 5;
    case 11: return c5(x) * 7;
    }
    return -1;
}

void _start(void)
{
    volatile int sink = dispatch(*(volatile int *)0x1000, 3);
    (void)sink;
    for (;;) {}
}
