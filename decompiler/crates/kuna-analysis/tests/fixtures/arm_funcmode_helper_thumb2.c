static int __attribute__((noinline)) t_static(int x) { return (x ^ 0x55) * 9 - 4; }
int t_tail(int x) { return t_static(x) + 2; }
