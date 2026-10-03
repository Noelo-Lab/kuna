/* Stores to a global before a load through a pointer that may point at it,
 * where the global is stored again after the load: the first store is the only
 * thing the load can read, so it must stay.  `d_loop` stores the global plus
 * one and puts it back around the load in a loop, `d_branch` stores before a
 * branch whose arm loads and stores again, `d_short`, `d_byte` and `d_long` do
 * so for other sizes, `d_index` loads through an indexed pointer in a loop
 * between the stores, `d_two` keeps two globals, `d_iter` stores in a loop and
 * again after it, `d_pre` stores before a loop of loads, `d_cond` and `d_sel`
 * load on one arm, `d_phi` computes the second store from a value read before
 * the first, `d_inc` increments the global around the load, and `d_while`
 * stores twice inside the loop.  `d_load` and `d_twice` load between two
 * straight-line stores and return what they loaded, and `d_mv` loads, stores
 * another global, and stores the loaded value to the first before a load and
 * a call.  `main` aims every pointer at the global.
 * With GLOBALLOADGUARD_HARNESS the functions come from the printed C, except
 * those named by a KEEP_ macro. */
#include <stdio.h>



int gi;
int gj;
short gs;
unsigned char gb;
long gl;

int touched;
int sunk;

#if defined(__clang__)
#define NOIPA __attribute__((noinline))
#else
#define NOIPA __attribute__((noinline, noipa))
#endif

NOIPA void touch(void) { touched++; }

int d_loop(int *p, int n);
int d_branch(int *p, int b, int c);
int d_short(int a, short *p, int c, int b);
int d_byte(int a, unsigned char *p, int c, int b);
long d_long(long a, long *p, int c, long b);
int d_index(int a, int *p, int n, int b);
int d_two(int a, int *p, int *q, int n, int b);
int d_iter(int *p, int n);
int d_pre(int a, int *p, int n, int b);
int d_cond(int a, int *p, int c, int b);
int d_phi(int *a, int *p, int c);
int d_inc(int *p, int n);
int d_while(int *p, int n);
int d_sel(int a, int *p, int c, int b);
int d_load(int a, int *p, int b);
int d_twice(int a, int *p, int b);
int d_mv(int *p, int *q);

#define NI __attribute__((noinline))
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_loop)
NI int d_loop(int *p, int n) { int s = 0; for (int i = 0; i < n; i++) { gi = gi + 1; s += *p; gi = gi - 1; } return s; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_branch)
NI int d_branch(int *p, int b, int c) { int t = b; gi = t; if (c) { t = *p + 1; gi = t; } touch(); return t; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_short)
NI int d_short(int a, short *p, int c, int b) { gs = a; int x = 0; if (c) x = *p; gs = b; return x * (x + 1); }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_byte)
NI int d_byte(int a, unsigned char *p, int c, int b) { gb = a; int x = 0; if (c) x = *p; gb = b; return x * (x + 5); }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_long)
NI long d_long(long a, long *p, int c, long b) { gl = a; long x = 0; if (c) x = *p; gl = b; return x * (x + 2); }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_index)
NI int d_index(int a, int *p, int n, int b) { gi = a; int s = 0; for (int i = 0; i < n; i++) s += p[i]; gi = b; return s; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_two)
NI int d_two(int a, int *p, int *q, int n, int b) { gi = a; gj = a + 1; int s = 0; for (int i = 0; i < n; i++) s += *p * *q; gi = b; gj = b; return s; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_iter)
NI int d_iter(int *p, int n) { int s = 0; for (int i = 0; i < n; i++) { gi = i + 3; s += *p; } gi = -1; return s; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_pre)
NI int d_pre(int a, int *p, int n, int b) { gi = a; int s = 0; for (int i = 0; i < n; i++) s += *p + i; gi = b; return s; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_cond)
NI int d_cond(int a, int *p, int c, int b) { gi = a; int x = 0; if (c) x = *p; gi = b; return x * (x + 1); }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_phi)
NI int d_phi(int *a, int *p, int c) { int t = a[3]; gi = t; if (c) { t = *p * 3; } gi = t - 7; touch(); return gi; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_inc)
NI int d_inc(int *p, int n) { int s = 0; for (int i = 0; i < n; i++) { gi++; s += *p; } return s; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_while)
NI int d_while(int *p, int n) { int s = 0; while (n-- > 0) { gi = n; s += *p * 2; gi = 100; } return s; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_sel)
NI int d_sel(int a, int *p, int c, int b) { gi = a; int x = c ? *p : -1; gi = b; return x * (x + 2); }
#endif

#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_load)
NI int d_load(int a, int *p, int b) { gi = a; int x = *p; gi = b; return x; }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_twice)
NI int d_twice(int a, int *p, int b) { gi = a; int x = *p; gi = b; return x * (x + 3); }
#endif
#if !defined(GLOBALLOADGUARD_HARNESS) || defined(KEEP_d_mv)
NI int d_mv(int *p, int *q) { int x = *p; gj = 0; gi = x; int y = *q; touch(); gi = y + 1; return y; }
#endif

int main(void) {
  int r, s;
  long l;
  gi = 4; r = d_loop(&gi, 3); printf("d_loop %d %d\n", r, gi);
  gi = 9; r = d_branch(&gi, 4, 1); s = gi; gi = 9; printf("d_branch %d %d %d\n", r, s, d_branch(&gi, 4, 0));
  gs = 1; r = d_short(-7, &gs, 1, 9); printf("d_short %d %d\n", r, gs);
  gb = 1; r = d_byte(200, &gb, 1, 9); printf("d_byte %d %d\n", r, gb);
  gl = 1; l = d_long(-3000000000L, &gl, 1, 9); printf("d_long %ld %ld\n", l, gl);
  gi = 1; r = d_index(6, &gi, 1, 9); printf("d_index %d %d\n", r, gi);
  gi = 1; gj = 1; r = d_two(5, &gi, &gj, 2, 2); printf("d_two %d %d %d\n", r, gi, gj);
  gi = 1; r = d_iter(&gi, 4); printf("d_iter %d %d\n", r, gi);
  gi = 1; r = d_pre(5, &gi, 3, 8); printf("d_pre %d %d\n", r, gi);
  gi = 1; r = d_cond(6, &gi, 1, 8); s = gi; gi = 1; printf("d_cond %d %d %d\n", r, s, d_cond(6, &gi, 0, 8));
  { int arr[4] = {0, 0, 0, 4}; gi = 9; r = d_phi(arr, &gi, 1); printf("d_phi %d %d\n", r, gi); }
  gi = 2; r = d_inc(&gi, 3); printf("d_inc %d %d\n", r, gi);
  gi = 2; r = d_while(&gi, 3); printf("d_while %d %d\n", r, gi);
  gi = 1; r = d_sel(7, &gi, 1, 3); s = gi; printf("d_sel %d %d %d\n", r, s, d_sel(7, &gi, 0, 3));
  gi = 3; r = d_load(7, &gi, 9); printf("d_load %d %d\n", r, gi);
  gi = 1; r = d_twice(7, &gi, 9); printf("d_twice %d %d\n", r, gi);
  gi = 1; gj = 5; r = d_mv(&gj, &gi); printf("d_mv %d %d %d\n", r, gi, gj);
  return 0;
}
