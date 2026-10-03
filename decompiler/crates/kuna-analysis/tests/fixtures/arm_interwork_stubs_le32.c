#define ARM __attribute__((target("arm"), noinline))
#define THM __attribute__((target("thumb"), noinline))
typedef int (*cb_t)(int);
static THM int t_helper(int x) { return (x ^ 0x5a) + 3; }
static ARM int a_ptr_only(int x) { return x * 17 - (x >> 2); }
static ARM int a_called(int x, int y) { int s = 0; for (int i = 0; i < x; i++) s += i * y; return s + 0x123; }
static THM int t_called(int x) { int s = 1; for (int i = 1; i < x; i++) s = s * 3 + i; return s; }
static THM int t_ptr_only(int x) { return x > 10 ? x - 10 : x * 2; }
static ARM int a_ptr_only2(int x) { int r = 0; while (x) { r += x & 1; x >>= 1; } return r; }
cb_t tab[4] = { a_ptr_only, t_ptr_only, a_ptr_only2, t_helper };
ARM int arm_export(int x) { return t_called(x) + a_called(x, 3) + t_helper(x); }
THM int thumb_export(int x) { return a_called(x, 5) - t_called(x + 1); }
THM int dispatch(int i, int x) { return tab[i & 3](x); }
ARM int arm_export2(int *p, int n) { int s = 0; for (int i = 0; i < n; i++) s += p[i] * 3 + t_helper(p[i]); return s; }
static ARM int a_after(int x) { return x * x + 7; }
THM int thumb_export2(int x) { return a_after(x) + t_helper(x) + 1; }
ARM int arm_export3(int x) { return thumb_export2(x) * 2 + a_after(x); }
