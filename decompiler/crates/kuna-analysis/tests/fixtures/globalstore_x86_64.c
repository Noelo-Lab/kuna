/* A value the binary stores to a global and then keeps using from its
 * register: the printed C must use the value, not read the global back.
 * The globals are volatile, so the compiler itself never re-reads them;
 * `usink` is unsigned while the value stored to it is signed, and the others
 * are signed while the values stored to them are unsigned.  `w_alias` stores
 * before a pointer store that may alias the global, so the store must stay
 * where the binary makes it; `w_sext` and `w_first` sign-extend the stored
 * value (`w_first` is cron's `first_word`), `w_i2f`
 * converts it to double and `w_eqc` compares a byte against a constant with
 * its top bit set.  `w_sadd`, `w_xordiv`, `w_f`, `w_sidx` and `w_cond` reach
 * the sign-sensitive operation through `+` or `^`, whose result C types after
 * the operand, and `w_cond` stores the value of a join.  `w_fold16`, `w_fold8`,
 * `w_neg16`, `w_sub16`, `w_not16` and `w_xor16` compare a short or a byte
 * computed from the stored value against a constant; a fold moves that constant
 * across the `+`, `-`, `~` or `^` (`u + 1 == 0` becomes `u == 0xffff`),
 * `w_carry16` tests the carry of `u + 5` and `w_meld16` ors two compares that
 * merge into `u < 2`.  `w_realias`, `w_rephi`, `w_reboth` and `w_recopy` store
 * to a plain `int` and then store through a pointer that may point at it, so
 * the compiler loads the global back: that load must print as a read of the
 * global, while `w_reboth` and `w_recopy` also use the value from its register.
 * At -O0 `w_rephi` shows a separate, older defect (the load that feeds the join
 * prints as the value), so that build keeps it from the source. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

volatile int sink;
volatile int sink2;
volatile unsigned usink;
volatile unsigned char csink;
volatile unsigned short hsink;
volatile short shsink;
int gre;
int gre2;
unsigned long res;
int retsel;
struct words { char pad[0x40002]; char rb[2][0x20001]; } words;

int w_branch(unsigned init, int k);
unsigned w_line(unsigned init);
unsigned w_loop(unsigned a, int n);
int w_less(unsigned a, int k);
unsigned w_div(unsigned a, int k);
int w_signed(int a, int k);
void w_order(unsigned a, int k);
int w_once(unsigned a);
int w_alias(unsigned long *id, const char *arg);
long w_sext(void);
char *w_first(const char *s, const char *t);
double w_i2f(int a, int k);
int w_eqc(unsigned char a);
int w_sadd(int a, int k);
unsigned w_xordiv(unsigned a, int k);
double w_f(int a, int k);
long w_sidx(int a, const long *t);
int w_cond(int a, int b, int k);
int w_fold16(int a, int k);
int w_fold8(int a, int k);
int w_neg16(int a, int k);
int w_sub16(int a, int k);
int w_not16(int a, int k);
int w_xor16(int a, int k);
int w_carry16(unsigned a, int k);
int w_meld16(unsigned a, int k);
int w_realias(unsigned a, int *p, int k);
int w_rephi(unsigned a, int *p, int k, int c);
int w_reboth(unsigned a, int *p, int k);
int w_recopy(unsigned a, int *p, int k);

#ifndef GLOBALSTORE_HARNESS
#define NI __attribute__((noinline))
NI int w_branch(unsigned init, int k) { int r; if (k) { r = k; } else { unsigned u = init * 3; sink = u; r = (int)(u >> 4); } return r; }
NI unsigned w_line(unsigned init) { unsigned u = init * 3; sink = u; return (u >> 7) ^ (u >> 4) ^ (u << 3); }
NI unsigned w_loop(unsigned a, int n) { unsigned acc = 0; for (int i = 0; i < n; i++) { unsigned u = a * i + 1; sink = u; acc += u >> 2; } return acc; }
NI int w_less(unsigned a, int k) { int r = 0; if (k) { unsigned u = a * 3; sink = u; r = u < 5 ? 1 : 2; } return r; }
NI unsigned w_div(unsigned a, int k) { unsigned r = 0; if (k) { unsigned u = a * 3; sink = u; r = u / 7 + u % 5; } return r; }
NI int w_signed(int a, int k) { int r = 0; if (k) { int u = a * 3; usink = u; r = u >> 4; } return r; }
NI void w_order(unsigned a, int k) { if (k) { unsigned u = a * 3; sink2 = 5; sink = u; } }
NI int w_once(unsigned a) { unsigned u = a * 3; sink = u; sink2 = 5; if (u <= 4) return 1; return 2; }
NI int w_alias(unsigned long *id, const char *arg) {
  if (strcmp(arg, "three") == 0) { res = 3; *id = 3; return 1; }
  char *end;
  res = strtoul(arg, &end, 0);
  if (!end || end == arg || *end || res > 255) return -1;
  *id = 7;
  return 0;
}
NI long w_sext(void) { retsel = 1 - retsel; return (long)retsel * 0x20001; }
#endif
#if !defined(GLOBALSTORE_HARNESS) || defined(GLOBALSTORE_KEEP_FIRST)
__attribute__((noinline)) char *w_first(const char *s, const char *t) {
  char *rb, *rp;
  retsel = 1 - retsel;
  rb = &words.rb[retsel][0];
  rp = rb;
  while (*s && strchr(t, *s)) s++;
  while (*s && !strchr(t, *s) && rp < &rb[0x20000]) *rp++ = *s++;
  *rp = '\0';
  return rb;
}
#endif
#ifndef GLOBALSTORE_HARNESS
NI double w_i2f(int a, int k) { double r = 0; if (k) { int u = a * 3; usink = u; r = (double)u; } return r; }
NI int w_eqc(unsigned char a) { unsigned char u = a * 3; csink = u; return u == 0xfd ? 1 : 2; }
NI int w_sadd(int a, int k) { int r = 0; if (k) { int u = a * 3; usink = u; r = (u + 1) >> 4; } return r; }
NI unsigned w_xordiv(unsigned a, int k) { unsigned r = 0; if (k) { unsigned u = a * 3; sink = u; r = (u ^ 0x10) / 7; } return r; }
NI double w_f(int a, int k) { double r = 0; if (k) { int u = a * 3; usink = u; r = (double)(u + 1); } return r; }
NI long w_sidx(int a, const long *t) { int i = a * 3; usink = i; return t[(i + 1) & 3] + (long)(i + 1); }
NI int w_cond(int a, int b, int k) { int u = k ? a * 3 : b * 5; usink = u; return (u + 1) / 7; }
NI int w_fold16(int a, int k) { int r = 7; if (k) { unsigned short u = a * 3; hsink = u; unsigned short w = u + 1; r = w == 0; } return r * 2 + 1; }
NI int w_fold8(int a, int k) { int r = 7; if (k) { signed char u = a * 3; csink = u; signed char w = u + 2; r = w == 1; } return r * 2 + 1; }
NI int w_neg16(int a, int k) { int r = 7; if (k) { short u = a * 3; hsink = u; short w = -u; r = w == 1; } return r * 2 + 1; }
NI int w_sub16(int a, int k) { int r = 7; if (k) { unsigned short u = a * 3; hsink = u; unsigned short w = u - 2; r = w == 0x7ffe; } return r * 2 + 1; }
NI int w_not16(int a, int k) { int r = 7; if (k) { unsigned short u = a * 3; hsink = u; unsigned short w = ~u; r = w == 0; } return r * 2 + 1; }
NI int w_xor16(int a, int k) { int r = 7; if (k) { unsigned short u = a * 3; hsink = u; unsigned short w = u ^ 0x7fff; r = w == 0x8000; } return r * 2 + 1; }
NI int w_meld16(unsigned a, int k) { int r = 7; if (k) { unsigned short u = a * 3; shsink = u; r = (u == 0) | (u == 1); } return r * 2 + 1; }
NI int w_carry16(unsigned a, int k) { int r = 7; if (k) { unsigned short u = a * 3; shsink = u; unsigned short t; r = __builtin_add_overflow(u, (unsigned short)5, &t); } return r * 2 + 1; }
NI int w_realias(unsigned a, int *p, int k) { unsigned u = a * 3; gre = u; *p = k; return gre / 16; }
NI int w_reboth(unsigned a, int *p, int k) { unsigned u = a * 3; gre = u; *p = k; return (gre >> 4) + (int)(u >> 4); }
NI int w_recopy(unsigned a, int *p, int k) { unsigned u = a * 3; gre = u; *p = k; gre2 = gre; return (int)(u >> 4); }
#endif
#if !defined(GLOBALSTORE_HARNESS) || defined(GLOBALSTORE_KEEP_REPHI)
__attribute__((noinline)) int w_rephi(unsigned a, int *p, int k, int c) {
  unsigned u = a * 3;
  gre = u;
  *p = k;
  int v = c ? gre : 7;
  return v / 16;
}
#endif

int main(void) {
  static const unsigned vals[] = {0, 1, 0x7f, 0x2aaaaaab, 0x55555556, 0x80000000u, 0xaaaaaaabu, 0xfffffff0u};
  unsigned long h = 0;
  for (unsigned t = 0; t < sizeof vals / sizeof *vals; t++) {
    unsigned v = vals[t];
    h = h * 31 + (unsigned)w_branch(v, 0);
    h = h * 31 + w_line(v);
    h = h * 31 + w_loop(v, 3);
    h = h * 31 + (unsigned)w_less(v, 1);
    h = h * 31 + w_div(v, 1);
    h = h * 31 + (unsigned)w_signed((int)v, 1);
    w_order(v, 1);
    h = h * 31 + (unsigned)sink + (unsigned)sink2 + usink;
    h = h * 31 + (unsigned)w_once(v);
    printf("%08x %d %u %u %d %u %d %d %d %u\n", v, w_branch(v, 0), w_line(v), w_loop(v, 3), w_less(v, 1), w_div(v, 1),
           w_signed((int)v, 1), w_once(v), sink, usink);
  }
  unsigned long id = 0;
  int ra = w_alias(&res, "5");
  printf("alias %d %lu", ra, res);
  ra = w_alias(&id, "9");
  printf(" %d %lu %lu", ra, id, res);
  ra = w_alias(&res, "300");
  printf(" %d %lu", ra, res);
  ra = w_alias(&res, "three");
  printf(" %d %lu\n", ra, res);
  for (int t = 0; t < 4; t++) {
    retsel = t - 1;
    long v = w_sext();
    printf("sext %d %ld %d\n", t - 1, v, retsel);
  }
  for (int t = 0; t < 3; t++) {
    retsel = t;
    long off = (long)(w_first("  ab c", " ") - (char *)&words);
    printf("first %d %ld %d\n", t, off, retsel);
  }
  double f1 = w_i2f(-5, 1);
  double f2 = w_i2f(0x7fffffff, 1);
  printf("i2f %f %f %u\n", f1, f2, usink);
  int e1 = w_eqc(0xff);
  int e2 = w_eqc(0x54);
  int e3 = w_eqc(0x01);
  printf("eqc %d %d %d %d\n", e1, e2, e3, (int)csink);
  static const long tab[4] = {10, 20, 30, 40};
  for (unsigned t = 0; t < sizeof vals / sizeof *vals; t++) {
    unsigned v = vals[t];
    int sa = w_sadd((int)v, 1);
    unsigned xd = w_xordiv(v, 1);
    double fd = w_f((int)v, 1);
    long si = w_sidx((int)v, tab);
    int c1 = w_cond((int)v, (int)v, 1);
    int c0 = w_cond((int)v, (int)v, 0);
    printf("derived %08x %d %u %f %ld %d %d %u %d\n", v, sa, xd, fd, si, c1, c0, usink, sink);
  }
  unsigned long hf = 0;
  for (unsigned v = 0; v < 0x20000; v++) {
    hf = hf * 1000003 + (unsigned)w_fold16((int)v, 1) * 3 + (unsigned)w_fold8((int)v, 1) * 5 + (unsigned)w_neg16((int)v, 1) * 7;
    hf = hf * 1000003 + (unsigned)w_sub16((int)v, 1) * 3 + (unsigned)w_not16((int)v, 1) * 5 + (unsigned)w_xor16((int)v, 1) * 7;
    hf = hf * 1000003 + (unsigned)w_carry16(v, 1) + (unsigned)w_meld16(v * 0x5555u, 1) * 9 + hsink + (unsigned)shsink * 3 + csink;
  }
  printf("fold %lx %u %d %u\n", hf, hsink, shsink, csink);
  for (unsigned t = 0; t < sizeof vals / sizeof *vals; t++) {
    unsigned v = vals[t];
    int other = 0;
    int a1 = w_realias(v, &gre, -77);
    int a2 = w_realias(v, &other, -77);
    int b1 = w_rephi(v, &gre, -77, 1);
    int b2 = w_rephi(v, &other, -77, 1);
    int b3 = w_rephi(v, &gre, -77, 0);
    int c1 = w_reboth(v, &gre, -77);
    int c2 = w_reboth(v, &other, -77);
    int d1 = w_recopy(v, &gre, -77);
    int d1g = gre2;
    int d2 = w_recopy(v, &other, -77);
    printf("realias %08x %d %d %d %d %d %d %d %d %d %d %d %d\n", v, a1, a2, b1, b2, b3, c1, c2, d1, d1g, d2, gre2, gre);
  }
  printf("%lu\n", h);
  return 0;
}
