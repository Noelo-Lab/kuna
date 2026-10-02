/* A call that never returns at its site although its callee can return,
   built into a stripped armel shared object (see fixtures/README.md). */
#include <stdio.h>
#if PART == 1
int th_helper(int x) __attribute__((visibility("hidden")));
int t2(int x) __attribute__((visibility("hidden")));
void a_last(int *buf, int x) __attribute__((visibility("hidden")));
int api(int *buf, int x) { int r = th_helper(x); if (r < 0) a_last(buf, r); return r + 7; }
int api2(int x) { puts("api2"); return t2(x) * 5; }
#elif PART == 2
int u(int a, int b) __attribute__((visibility("hidden")));
__attribute__((visibility("hidden"))) int th_helper(int x) { int s = 0; for (int i = 0; i < x; i++) s += i * i; return s; }
__attribute__((visibility("hidden"))) int t2(int x) { return u(x, x + 9) + 1; }
#elif PART == 3
int g_last;
__attribute__((visibility("hidden"))) void report_bad(int v) { g_last = v; }
__attribute__((visibility("hidden"))) void a_last(int *buf, int x) { buf[1] = x * 3; report_bad(x); __builtin_unreachable(); }
#elif PART == 4
__attribute__((visibility("hidden"))) int u(int a, int b) { return (a << 3) - (b >> 1) + (a & b); }
#endif
