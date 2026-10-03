/* SPARC returns a long long in %o0:%o1, the high word in %o0. Each of these
 * puts its low word in %o1 on purpose: through restore's destination register
 * (restore %g0,1,%o1 in k_one and status64), or in %i1 for restore to hand
 * back (mov 10,%i1 in sel_const, mix2's product), so none is a leftover. */
typedef long long i64;
typedef unsigned long long u64;
void ext(void);
int ext_i(int);
i64 neg_one(void) { ext(); return -1; }
i64 k_one(void) { ext(); return 1; }
i64 status64(int x) { ext(); if (x) return 1; return 0; }
u64 bool64(int a, int b) { ext_i(a); return a == b; }
i64 sel_const(int c) { ext(); return c ? 10 : 20; }
u64 mix2(unsigned x, int c) { ext(); if (c) return (u64)x << 32; return (u64)x * 3; }
u64 mix3(unsigned x, int c) { ext(); if (c) return 0x500000000ULL; return (u64)x * 3; }
