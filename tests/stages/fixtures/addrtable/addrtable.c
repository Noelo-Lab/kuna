static int h0(int x) { return x + 1; }
static int h1(int x) { return x * 3; }
static int h2(int x) { return x - 7; }
static int (*hooks[])(int) = { h2, h1, h0 };
__attribute__((noinline)) int hook(unsigned i, int x) { return hooks[i % 3](x) + 1; }
__attribute__((noinline)) void set_hook(unsigned i, int (*f)(int)) { hooks[i % 3] = f; }
void _start(void) { for (;;) { hook(2, 5); set_hook(1, h1); } }
