/* (kuna castwiden) 64-bit widenings C's own conversions perform.
 * Each function widens a 32-bit or narrower value to 64 bits where C converts it
 * by itself: beside a 64-bit operand of `+ - * / % & | ^`, beside a literal the
 * printer can type as 64-bit, and into an assignment, a store, an argument or a
 * return of a 64-bit type.  The keep_ functions must keep their cast: a shift, a
 * comparison, an unsigned widening beside a signed operand, and a negated literal
 * whose C value is positive (`keep_neglit`, whose printed C is checked as text
 * only: kuna prints `+ -0x80000000` there with the option off too, which C reads
 * as adding 2^31).  sq, usq, sq_diff and dist read one widened value in both
 * operands of a product, which keeps both casts; par_add and par_mul put a 32-bit
 * sum or product beside a long of the same operator, which must keep its
 * parentheses; chain_add, chain_mul, chain_add3 and chain_xor put a widened
 * operand beside a chain of the same operator, which C regroups.  The main
 * program feeds negative values, 0x80000000 .. 0xffffffff,
 * sums past 32 bits and squares past 2^32.  The round-trip test in
 * kuna-cli/tests/decompile_all_cli.rs compiles the functions as kuna prints them,
 * with the option off, on and literal, and checks that each build prints what
 * this program prints:
 *
 *   2147483647 2147483647 5 0 0 7 1 0 -3
 *   0 1 0
 *   0 0
 *   2147483646 2147483648 4 1 3 -5 0 -1664 -4
 *   18446744073709551615 1 18446744073709551613
 *   5
 *   -1 18446744073709551615
 *   4294967294 0 2147483652 4611686014132420609 -6442450941 25769803771 2147483648 3573412788608 2147483644
 *   2147483647 1 18446744071562067965
 *   0
 *   2147483647 2147483647
 *   -1 4294967295 18446744071562067973 4611686018427387904 6442450944 -25769803769 -2147483647 -3573412790272 -2147483651
 *   18446744071562067968 0 2147483648
 *   2147483647
 *   -2147483648 18446744071562067968
 *   2147483640 2147483654 18446744073709551614 49 21 -77 -6 -11648 -10
 *   18446744073709551609 0 18446744073709551607
 *   1
 *   -7 18446744073709551609
 *   2147483650 2147483644 8 9 -9 43 4 4992 0
 *   3 1 18446744073709551609
 *   0
 *   3 3
 *   18446744073709551600 0 4294967296 0
 *   18446744071562067983 34359738360 8589934591 18446742974197923840 4294967296
 *   18446744073709551600 17179869184 6442450944 0 8589934591
 *   18446744073709551603 24 4294967299 3298534883328 6148914691236517200
 *   -133 122 255 250 2147483775
 *   3 -1 -1
 *   38654705538 4294967284
 *   6148914691236517202 715827882 6148914690520689323
 *   4294967292 2 10
 *   -6 18446744069414584334
 *   4294967294 18446744071562067968
 *   -2147483647 0
 *   0 0 0 0 2147483647 2147483648 0 0
 *   -5 0 2147483647 18446744073709551603
 *   1 18446744065119617025 0 2 2147483645 2147483647 2147483647 -6442450941
 *   -7 -9 2147483644 18446744069414584332
 *   4611686014132420609 4611686014132420609 1152921504606846976 5764607516591783938 2147483645 -1 2147483647 4611686009837453315
 *   4294967289 19327352823 8589934588 18446744071562067980
 *   4611686018427387904 4611686018427387904 1152921504606846976 5764607523034234880 2147483647 0 0 -4611686016279904256
 *   -4294967301 -19327352832 -4294967297 18446744071562067955
 *   4294967296 4294967296 1073741824 5368709120 2147614719 2147549184 0 422212464869376
 *   131067 589824 2147680255 18446744073709486067
 *   4295098369 18446181119461294081 1073741824 5368905730 2147352573 2147418111 281477124063231 -422218907320317
 *   -131079 -589833 2147287036 18446744069414649868
 *   2147488281 2147488281 536895241 2684337181 2147576329 2147529989 -4611676066988167705 298549619056881
 *   92677 417069 2147622670 18446744073709505270
 *
 * Built with:
 *   gcc   -O0 -o castwiden_gcc_O0_x86_64   castwiden_x86_64.c
 *   clang -O0 -o castwiden_clang_O0_x86_64 castwiden_x86_64.c
 *   gcc   -O2 -fno-inline -o castwiden_gcc_O2_x86_64 castwiden_x86_64.c
 *   clang -O2 -fno-inline -o castwiden_clang_O2_x86_64 castwiden_x86_64.c
 */
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))

/* an int beside a loaded long */
NI long add_load(long *p, int i) { return p[1] + i; }
NI long minus_load(long *p, int i) { return p[1] - i; }
/* an int beside a loaded unsigned long: C converts the int to unsigned long */
NI unsigned long add_uload(unsigned long *p, int i) { return p[2] + i; }
/* an unsigned int beside a loaded unsigned long */
NI unsigned long mix_uint(unsigned long *p, unsigned int u) { return (p[0] ^ u) | (p[1] & u); }
/* a byte beside a loaded long */
NI long add_char(long *p, const signed char *c) { return p[0] + c[1]; }
/* signed division and remainder by a widened int */
NI long div_load(long *p, int i) { return p[0] / i + p[1] % i; }
/* unsigned division by a widened unsigned int */
NI unsigned long udiv_load(unsigned long *p, unsigned int u) { return p[0] / u; }
/* a product of two widened ints: one cast must stay */
NI long mul_two(int a, int b) { return (long)a * b; }
/* a widened int times and plus literals: 64-bit arithmetic */
NI long lit_mul(int i) { return (long)i * 12 + 7; }
NI long lit_add(int i) { return (long)i + 1; }
NI unsigned long lit_umul(unsigned int u) { return (unsigned long)u * 8; }
NI long lit_index(int i, int j) { return ((long)i * 12 + j) * 128; }
NI unsigned long lit_mask(unsigned int u) { return (unsigned long)u | 0x100000000UL; }
NI long lit_neg(int i) { return (long)i - 3; }

/* destinations */
NI unsigned long ret_ulong(int i) { return i; }
NI void store_long(long *p, int i) { p[1] = i; }
NI void store_ulong(unsigned long *p, int i) { p[2] = i; }
NI void store_uchar(unsigned long *p, const unsigned char *c) { p[3] = c[0]; }
NI unsigned long assign_ulong(int i, int j) {
  unsigned long v = i;
  unsigned long w = j;
  return v ^ (w << 1);
}
NI unsigned long assign_loop(const int *a, int n) {
  unsigned long s = 0;
  for (int k = 0; k < n; k++) {
    unsigned long v = a[k];
    s = s * 3 + v;
  }
  return s;
}
NI void sink(unsigned long *v) { *v ^= 1; }
NI unsigned long assign_call(int i, unsigned int u) {
  unsigned long v = i;
  unsigned long w = u;
  sink(&v);
  sink(&w);
  return v + w;
}
NI unsigned long assign_size(int i) {
  unsigned long n = i;
  sink(&n);
  return n / 3;
}
struct rec {
  long a;
  unsigned long b;
  int c;
  unsigned int d;
};
NI long field_add(struct rec *r) { return r->a + r->c; }
NI unsigned long field_umix(struct rec *r) { return (r->b - r->d) ^ (r->b + r->c); }
NI long find_len(const char *s, int n) {
  const char *q = memchr(s, 'x', n);
  return q ? q - s : -1;
}

/* one widened value read twice: both casts stay */
NI long sq(int i) { return (long)i * i; }
NI unsigned long usq(unsigned int u) { return (unsigned long)u * u; }
NI long sq_diff(int a, int b) {
  long d = (int)((unsigned int)a - (unsigned int)b);
  return d * d;
}
NI long dist(int x1, int y1, int x2, int y2) {
  long dx = (int)((unsigned int)x2 - (unsigned int)x1);
  long dy = (int)((unsigned int)y2 - (unsigned int)y1);
  return dx * dx + dy * dy;
}
/* a 32-bit sum or product beside a long of the same operator keeps its parentheses */
NI long par_add(long *p, int a, int b) { return p[1] + (int)((unsigned int)a + (unsigned int)b); }
NI long par_mul(long *p, int a, int b) { return p[1] * (int)((unsigned int)a * (unsigned int)b); }
/* a widened operand beside a chain of the same operator, which prints without
   parentheses: C pairs it with the chain's first leaf, so one of the two keeps
   its cast */
NI long chain_add(long *p, int a, int b) { return (long)a + ((long)b + p[0]); }
NI long chain_mul(long *p, int a, int b) { return (long)a * ((long)b * p[0]); }
NI long chain_add3(long *p, int a, int b, int c) { return (long)a + ((long)b + ((long)c + p[1])); }
NI unsigned long chain_xor(unsigned long *p, unsigned int a, unsigned int b) {
  return (unsigned long)a ^ ((unsigned long)b ^ p[0]);
}

/* must keep */
NI unsigned long keep_shift(unsigned int u) { return (unsigned long)u << 40; }
NI int keep_less(long *p, int i) { return p[0] < i; }
NI unsigned long keep_mixed(long *p, const unsigned char *c) { return (unsigned long)c[0] + (unsigned long)p[0]; }
NI long keep_neglit(int i) { return (long)i - 0x80000000L; }

int main(void) {
  long lp[4] = {-5, 0x7fffffff, -1, 3};
  unsigned long up[4] = {0xfffffffffffffff0UL, 0x80000000UL, 5, 0};
  static const signed char sc[] = {1, -128, 127};
  static const unsigned char uc[] = {0xff, 0x80};
  int ints[] = {0, -1, 0x7fffffff, (int)0x80000000, -7, 3};
  unsigned int uints[] = {0, 0xffffffffu, 0x80000000u, 3};
  for (int k = 0; k < 6; k++) {
    int i = ints[k];
    printf("%ld %ld %lu %ld %ld %ld %ld %ld %ld\n", add_load(lp, i), minus_load(lp, i), add_uload(up, i),
           mul_two(i, i), mul_two(i, -3), lit_mul(i), lit_add(i), lit_index(i, i), lit_neg(i));
    printf("%lu %d %lu\n", ret_ulong(i), keep_less(lp, i), assign_ulong(i, (int)(0u - (unsigned int)i)));
    if (i != 0)
      printf("%ld\n", div_load(lp, i));
    long sl[4] = {0};
    unsigned long su[4] = {0};
    store_long(sl, i);
    store_ulong(su, i);
    printf("%ld %lu\n", sl[1], su[2]);
  }
  for (int k = 0; k < 4; k++) {
    unsigned int u = uints[k];
    printf("%lu %lu %lu %lu", mix_uint(up, u), lit_umul(u), lit_mask(u), keep_shift(u));
    if (u != 0)
      printf(" %lu", udiv_load(up, u));
    printf("\n");
  }
  unsigned long su[4] = {0};
  store_uchar(su, uc);
  printf("%ld %ld %lu %lu %lu\n", add_char(lp, sc), add_char(lp, sc + 1), su[3], keep_mixed(lp, uc),
         keep_mixed(lp + 1, uc + 1));
  printf("%ld %ld %ld\n", find_len("abcxdef", 7), find_len("abcxdef", 3), find_len("abcdef", 6));
  printf("%lu %lu\n", assign_loop(ints, 6), assign_loop(ints + 1, 3));
  printf("%lu %lu %lu\n", assign_size(-7), assign_size(0x7fffffff), assign_size((int)0x80000000));
  printf("%lu %lu %lu\n", assign_call(-1, 0xffffffffu), assign_call((int)0x80000000, 0x80000000u), assign_call(5, 7));
  struct rec rs[3] = {{-5, 7, -1, 0xffffffffu}, {0x7fffffff, 0xfffffffffffffff0UL, 0x7fffffff, 1},
                      {1, 2, (int)0x80000000, 0x80000000u}};
  for (int k = 0; k < 3; k++)
    printf("%ld %lu\n", field_add(&rs[k]), field_umix(&rs[k]));
  int big[] = {0, -1, 0x7fffffff, (int)0x80000000, 0x10000, -0x10001, 46341};
  for (int k = 0; k < 7; k++) {
    int i = big[k];
    printf("%ld %lu %ld %ld %ld %ld %ld %ld\n", sq(i), usq((unsigned int)i), sq_diff(i, i >> 1), dist(0, 0, i, i >> 1),
           par_add(lp, i, i), par_add(lp, i, 1), par_mul(lp, i, i), par_mul(lp, i, 3));
    printf("%ld %ld %ld %lu\n", chain_add(lp, i, i), chain_mul(lp + 3, i, 3), chain_add3(lp, i, i, i),
           chain_xor(up, (unsigned int)i, 3u));
  }
  return 0;
}
