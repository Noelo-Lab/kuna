static int __attribute__((noinline)) s_static(int x, int y) { return x * 5 + y * 7 + 3; }
int a_func(int x) { return s_static(x, x >> 2) * 3; }
