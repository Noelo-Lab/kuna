/* Built twice: PART 1 with -marm, PART 2 with -mthumb. */
#if PART == 1
extern int t0(int);
extern int (*tab[2])(int);
__attribute__((noreturn)) void _start(void) {
  volatile int r = t0(3) + tab[0](4) + tab[1](5);
  for (;;) r++;
}
#else
int t0(int x) { return x * 7 + 1; }
static int helper(int x) { return (x ^ 0x5a) + 3; }
static int t_ptr(int x) { int s = 0; for (int i = 0; i < x; i++) s += helper(i); return s; }
static int t_ptr2(int x) { return x > 10 ? x - 10 : x * 2; }
int (*tab[2])(int) = { t_ptr, t_ptr2 };
#endif
