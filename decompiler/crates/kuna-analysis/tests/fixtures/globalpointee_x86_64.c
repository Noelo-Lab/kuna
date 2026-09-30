/* A pointer the binary stores to a global whose declared pointee type differs
 * from its own, and then keeps using from its register: the printed C must
 * dereference the value, not read the global back.  `gc` is `char *` while
 * `p_int` stores an `int *` to it and reads two ints through the register; with
 * the global declared as here, `gc[1]` would read a byte.  The globals are
 * `char *`, `short *`, `int *`, `long *`, `struct rec *` and `void *`, and the
 * values stored to them point at ints, shorts, chars, longs and records.  The
 * `r_` functions read through the value, the `w_` ones write through it, `m_`
 * store before a call (`m_call`), inside a branch (`m_join`) or a loop
 * (`m_loop`), and `g_` read through the global itself: `g_reread` as the source
 * says, `g_alias` after a pointer store that may point at the global, so its
 * load of `gi` must stay a load. */
#include <stdio.h>

struct rec { int a; short b; long c; };

char *gc;
short *gs;
int *gi;
long *gl;
struct rec *gr;
void *gv;
int touched;

#if defined(__clang__)
#define NOIPA __attribute__((noinline))
#else
#define NOIPA __attribute__((noinline, noipa))
#endif

NOIPA void touch(void) { touched++; }

int p_int(int *p, int k);
long r_short(short *p, int k);
int r_char(char *p, int k);
long r_rec(struct rec *p, int k);
long r_long(long *p, int k);
int r_void(int *p, int k);
void w_long(long *p, int k, long v);
void w_short(short *p, int k, short v);
int m_call(int *p, int k);
int m_join(int *p, int k, int c);
void m_write(int *p, int k, int v);
int m_loop(int *p, int n);
int g_reread(int *p, int k);
int g_alias(int *p, int k, int **pp);

#ifndef GLOBALPOINTEE_HARNESS
#define NI __attribute__((noinline))
NI int p_int(int *p, int k) { int *q = p + k; gc = (char *)q; return q[1] + q[2]; }
NI long r_short(short *p, int k) { short *q = p + k; gl = (long *)q; return q[1] + q[3]; }
NI int r_char(char *p, int k) { char *q = p + k; gi = (int *)q; return q[1] + q[5]; }
NI long r_rec(struct rec *p, int k) { struct rec *q = p + k; gi = (int *)q; return q->a + q->b + q[1].c; }
NI long r_long(long *p, int k) { long *q = p + k; gr = (struct rec *)q; return q[0] + q[3]; }
NI int r_void(int *p, int k) { int *q = p + k; gv = q; return q[1] * 3 + q[2]; }
NI void w_long(long *p, int k, long v) { long *q = p + k; gs = (short *)q; q[1] = v; q[2] = v + 1; }
NI void w_short(short *p, int k, short v) { short *q = p + k; gl = (long *)q; q[1] = v; q[3] = v * 2; }
NI int m_call(int *p, int k) { int *q = p + k; gc = (char *)q; int x = q[1] + q[2]; touch(); return x; }
NI int m_join(int *p, int k, int c) { int r = 0; if (c) { int *q = p + k; gc = (char *)q; r = q[1] + q[3]; } touch(); return r; }
NI void m_write(int *p, int k, int v) { int *q = p + k; gs = (short *)q; q[1] = v; q[2] = v + 1; touch(); }
NI int m_loop(int *p, int n) { int s = 0; for (int i = 0; i < n; i++) { int *q = p + i * 2; gc = (char *)q; s += q[1]; } return s; }
NI int g_reread(int *p, int k) { gi = p + k; return gi[1] + gi[2]; }
NI int g_alias(int *p, int k, int **pp) { gi = p + k; *pp = p; return gi[1]; }
#endif

#define OFF(g, base) ((long)((char *)(g) - (char *)(base)))

int main(void) {
  int ia[16];
  short sa[16];
  char ca[32];
  long la[16];
  struct rec ra[4];
  for (int i = 0; i < 16; i++) {
    ia[i] = i * 7 + 1;
    sa[i] = (short)(i * 3 - 5);
    la[i] = i * 1000003L;
  }
  for (int i = 0; i < 32; i++)
    ca[i] = (char)(i * 5 + 1);
  for (int i = 0; i < 4; i++) {
    ra[i].a = i * 11;
    ra[i].b = (short)(i * 13 - 40);
    ra[i].c = i * 100001L;
  }
  long r;
  r = p_int(ia, 2);
  printf("p_int %ld %ld\n", r, OFF(gc, ia));
  r = r_short(sa, 3);
  printf("r_short %ld %ld\n", r, OFF(gl, sa));
  r = r_char(ca, 4);
  printf("r_char %ld %ld\n", r, OFF(gi, ca));
  r = r_rec(ra, 1);
  printf("r_rec %ld %ld\n", r, OFF(gi, ra));
  r = r_long(la, 2);
  printf("r_long %ld %ld\n", r, OFF(gr, la));
  r = r_void(ia, 5);
  printf("r_void %ld %ld\n", r, OFF(gv, ia));
  w_long(la, 3, -9);
  printf("w_long %ld %ld %ld %ld\n", la[3], la[4], la[5], OFF(gs, la));
  w_short(sa, 2, 21);
  printf("w_short %d %d %d %ld\n", sa[2], sa[3], sa[5], OFF(gl, sa));
  r = m_call(ia, 3);
  printf("m_call %ld %ld %d\n", r, OFF(gc, ia), touched);
  r = m_join(ia, 1, 1);
  printf("m_join %ld %ld", r, OFF(gc, ia));
  r = m_join(ia, 6, 0);
  printf(" %ld %ld %d\n", r, OFF(gc, ia), touched);
  m_write(ia, 4, 77);
  printf("m_write %d %d %d %ld %d\n", ia[4], ia[5], ia[6], OFF(gs, ia), touched);
  r = m_loop(ia, 5);
  printf("m_loop %ld %ld\n", r, OFF(gc, ia));
  r = g_reread(ia, 1);
  printf("g_reread %ld %ld\n", r, OFF(gi, ia));
  int *other = 0;
  r = g_alias(ia, 2, &other);
  printf("g_alias %ld %ld %ld", r, OFF(gi, ia), OFF(other, ia));
  r = g_alias(ia, 2, &gi);
  printf(" %ld %ld\n", r, OFF(gi, ia));
  return 0;
}
