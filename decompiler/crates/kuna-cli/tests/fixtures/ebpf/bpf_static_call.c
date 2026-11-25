static __attribute__((noinline)) int twice(int a) { return a * 2; }
static __attribute__((noinline)) int plus7(int a) { return a + 7; }
__attribute__((section("xdp"))) int prog(void *ctx)
{ int x = (int)(long)ctx; return twice(x) ^ plus7(x); }
