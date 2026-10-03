/* Built three times: PART 1 and 3 with -mthumb, PART 2 with -marm. */
#if PART == 1
int thumb_first(int x) { return x * 5 + 3; }
#elif PART == 2
int arm_export(int x) { int s = 0; for (int i = 0; i < x; i++) s += i ^ 0x33; return s; }
#else
typedef int (*fn)(int);
static int t_static(int x) { return x > 10 ? x - 10 : x * 2; }
static int t_static2(int x) { int r = 0; while (x) { r += x & 1; x >>= 1; } return r; }
fn table[2] = { t_static, t_static2 };
int thumb_export(int i, int x) { return table[i & 1](x) + 1; }
#endif
