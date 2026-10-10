#define ARM __attribute__((target("arm"), noinline))
#define THUMB __attribute__((target("thumb"), noinline))
extern int sink(char *);
int (*g_cb)(int);
THUMB int t_first(int x) { return x * 13 + 1; }
static ARM int s_buf(int x);
static THUMB int t_help(int x);
ARM int a_func(int x) { int r = s_buf(x) + 4; g_cb = t_help; return r; }
static ARM int s_buf(int x) { char b[16]; b[x & 15] = 1; return sink(b) + x; }
static THUMB int t_help(int x) { return (x ^ 0x3c) * 7 - 5; }
THUMB int t_last(int x) { return t_help(x) - 2; }
