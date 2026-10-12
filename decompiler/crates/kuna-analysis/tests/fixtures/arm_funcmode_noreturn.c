extern void abort(void);
#define ARM __attribute__((target("arm"), noinline))
#define THUMB __attribute__((target("thumb"), noinline))
THUMB int t_first(int x) { return x * 13 + 1; }
static ARM int s_chk(int x);
static THUMB int t_help(int x);
ARM int a_func(int x) { return s_chk(x) + t_help(x + 1); }
static ARM int s_chk(int x) { if (x < -100) abort(); return x * 2; }
static THUMB int t_help(int x) { return (x ^ 0x3c) * 7 - 5; }
THUMB int t_last(int x) { return a_func(x) - 2; }
