/* A global the function stores to or reads, and a store through a pointer
 * that may point at it.  The harness aims every pointer at the global itself,
 * so the printed C must read the global where the binary reads it: after the
 * pointer store when the binary loads it again (`s_` functions, issue #792:
 * a parameter, a constant, a value used once, a short, a byte, a long, a
 * byte-wide pointer store into an int, a store on one branch, a store in a
 * loop), before it when the binary keeps what it read in a register (`l_`
 * functions), the store to the global must stay in front of the pointer
 * store (`w_` functions, `w_inc` with a value read from the global before the
 * pointer store), and a value loaded through the pointer must be read before a
 * later store to the global (`r_order`; `r_cond`, `r_two` and `r_loop` store
 * to a global on a branch, on two branches and in a loop).  A store to the
 * global ahead of a load through the pointer stays ahead of it: in a loop
 * (`e_after`), before a call (`e_call`), when a branch stores again and a
 * register carries both values to the call (`e_reg`), and before a return
 * block that -O0 shares with another path (`w_epi`).  `w_big` and `w_big2`
 * make more pointer stores than heritage guards, so their global keeps the
 * output of `indexaliasguard load`.  `w_dead` and `w_dead2` store to the
 * global, load through the pointer on a branch and store to the global again;
 * at -O0 the first store is lost (#825), so the -O0 harness keeps the
 * source's own copies of them. */
#include <stdio.h>

int gi;
int gj;
short gs;
unsigned char gb;
long gl;
int touched;
int gz0, gz1, gz2, gz3, gz4, gz5, gz6, gz7, gz8, gz9, gz10, gz11;
int gk;
int rbuf[848];

#if defined(__clang__)
#define NOIPA __attribute__((noinline))
#else
#define NOIPA __attribute__((noinline, noipa))
#endif

NOIPA void touch(void) { touched++; }
NOIPA void touch2(int v) { touched += v; }

int s_param(int a, int *p, int k);
int s_const(int *p, int k);
int s_once(unsigned a, int *p, int k);
int s_plain(int a, int *p, int k);
int s_short(int a, short *p, int k);
int s_byte(int a, unsigned char *p, int k);
long s_long(long a, long *p, long k);
int s_narrow(int a, char *p, int k);
int s_branch(int a, int *p, int k, int c);
int s_loop(int a, int *p, int n);
int s_cmp(int a, int *p, int k);
int l_keep(int *p, int k);
int l_keepc(int *p, int k, int c);
int l_reload(int *p, int k);
int l_twice(int *p, int k);
void w_order(int a, int *p, int k);
int w_order2(int a, int *p, int k);
int r_order(int *p, int b);
void w_inc(int *p, int k);
int r_cond(int *p, int b, int c);
int r_two(int *p, int b, int c);
int r_loop(int *p, int n, int b);
int e_after(int *p, int b, int n);
int e_call(int a, int *p);
int e_reg(int *p, int *a, int c);
long w_epi(int c, int *p, int v);
int w_big(int *p, int *q, int *r, int b);
int w_big2(int *p, int *q, int *r, int b);
int w_dead(int *p, int *a, int c);
int w_dead2(int *p, int *a, int c);

#ifndef STOREALIAS_HARNESS
NOIPA int s_param(int a, int *p, int k) { gi = a; *p = k; return gi / 16; }
NOIPA int s_const(int *p, int k) { gi = 5; *p = k; return gi / 16; }
NOIPA int s_once(unsigned a, int *p, int k) { unsigned u = a * 3; gi = u; *p = k; return gi >> 4; }
NOIPA int s_plain(int a, int *p, int k) { gi = a; *p = k; return gi; }
NOIPA int s_short(int a, short *p, int k) { gs = a; *p = k; return gs * 3; }
NOIPA int s_byte(int a, unsigned char *p, int k) { gb = a; *p = k; return gb + 1; }
NOIPA long s_long(long a, long *p, long k) { gl = a; *p = k; return gl * 3; }
NOIPA int s_narrow(int a, char *p, int k) { gi = a; *p = k; return gi; }
NOIPA int s_branch(int a, int *p, int k, int c) { gi = a; if (c) *p = k; return gi * 2; }
NOIPA int s_loop(int a, int *p, int n) { int s = 0; gi = a; for (int i = 0; i < n; i++) { p[i] = i + 9; s += gi; } return s; }
NOIPA int s_cmp(int a, int *p, int k) { gi = a; *p = k; return gi < 0 ? 11 : 22; }
NOIPA int l_keep(int *p, int k) { int a = gi; *p = k; return a; }
NOIPA int l_keepc(int *p, int k, int c) { int a = gi; if (c) *p = k; return a * gi; }
NOIPA int l_reload(int *p, int k) { int a = gi; *p = k; return a * 7 + gi; }
NOIPA int l_twice(int *p, int k) { int a = gi; *p = k; int b = gi; *p = k + 1; return a * 100 + b * 10 + gi; }
NOIPA void w_order(int a, int *p, int k) { gi = a; *p = k; touch(); }
NOIPA int w_order2(int a, int *p, int k) { gi = a; *p = k; touch(); return a; }
NOIPA int r_order(int *p, int b) { int x = *p; gi = b; return x; }
NOIPA void w_inc(int *p, int k) { int x = gi + 1; *p = k; gi = x; }
NOIPA int r_cond(int *p, int b, int c) { int x = *p; if (c) gi = b; return x + c; }
NOIPA int r_two(int *p, int b, int c) { int x = *p; if (c > 5) gi = b; if (c > 7) gj = b; return x - c; }
NOIPA int r_loop(int *p, int n, int b) { int s = 0; for (int i = 0; i < n; i++) { int x = p[i]; gi = b + i; s += x; } return s; }
NOIPA int e_after(int *p, int b, int n) { int s = 0; for (int i = 0; i < n; i++) { gi += b; s += *p; } return s; }
NOIPA int e_call(int a, int *p) { gi = a; int x = *p; touch(); return x * 2 + gi; }
NOIPA int e_reg(int *p, int *a, int c) { int t = a[3]; gi = t; if (c) { t = (*p - a[1]) * 3; gi = t; } gj = t - 7; touch(); return gj; }
NOIPA long w_epi(int c, int *p, int v) { if (c) { gi = v; return *p; } return 0; }
NOIPA int w_big(int *p, int *q, int *r, int b) {
  gz0 = b + 0; gz1 = b + 1; gz2 = b + 2; gz3 = b + 3; gz4 = b + 4; gz5 = b + 5; gz6 = b + 6; gz7 = b + 7; gz8 = b + 8; gz9 = b + 9; gz10 = b + 10; gz11 = b + 11;
  r[0] = b; r[7] = b; r[14] = b; r[21] = b; r[28] = b; r[35] = b; r[42] = b; r[49] = b; r[56] = b; r[63] = b; r[70] = b; r[77] = b; r[84] = b; r[91] = b; r[98] = b; r[105] = b; r[112] = b; r[119] = b; r[126] = b; r[133] = b; r[140] = b; r[147] = b; r[154] = b; r[161] = b; r[168] = b; r[175] = b; r[182] = b; r[189] = b; r[196] = b; r[203] = b; r[210] = b; r[217] = b; r[224] = b; r[231] = b; r[238] = b; r[245] = b; r[252] = b; r[259] = b; r[266] = b; r[273] = b; r[280] = b; r[287] = b; r[294] = b; r[301] = b; r[308] = b; r[315] = b; r[322] = b; r[329] = b; r[336] = b; r[343] = b; r[350] = b; r[357] = b; r[364] = b; r[371] = b; r[378] = b; r[385] = b; r[392] = b; r[399] = b; r[406] = b; r[413] = b; r[420] = b; r[427] = b; r[434] = b; r[441] = b; r[448] = b; r[455] = b; r[462] = b; r[469] = b; r[476] = b; r[483] = b; r[490] = b; r[497] = b; r[504] = b; r[511] = b; r[518] = b; r[525] = b; r[532] = b; r[539] = b; r[546] = b; r[553] = b; r[560] = b; r[567] = b; r[574] = b; r[581] = b; r[588] = b; r[595] = b; r[602] = b; r[609] = b; r[616] = b; r[623] = b; r[630] = b; r[637] = b; r[644] = b; r[651] = b; r[658] = b; r[665] = b; r[672] = b; r[679] = b; r[686] = b; r[693] = b; r[700] = b; r[707] = b; r[714] = b; r[721] = b; r[728] = b; r[735] = b; r[742] = b; r[749] = b; r[756] = b; r[763] = b; r[770] = b; r[777] = b; r[784] = b; r[791] = b; r[798] = b; r[805] = b; r[812] = b; r[819] = b; r[826] = b; r[833] = b;
  int t = gk + b; int x = *p; gk = t; *q = 7; touch(); return x * 100 + gk;
}
NOIPA int w_big2(int *p, int *q, int *r, int b) {
  gz0 = b + 0; gz1 = b + 1; gz2 = b + 2; gz3 = b + 3; gz4 = b + 4; gz5 = b + 5; gz6 = b + 6; gz7 = b + 7; gz8 = b + 8; gz9 = b + 9; gz10 = b + 10; gz11 = b + 11;
  r[0] = b; r[7] = b; r[14] = b; r[21] = b; r[28] = b; r[35] = b; r[42] = b; r[49] = b; r[56] = b; r[63] = b; r[70] = b; r[77] = b; r[84] = b; r[91] = b; r[98] = b; r[105] = b; r[112] = b; r[119] = b; r[126] = b; r[133] = b; r[140] = b; r[147] = b; r[154] = b; r[161] = b; r[168] = b; r[175] = b; r[182] = b; r[189] = b; r[196] = b; r[203] = b; r[210] = b; r[217] = b; r[224] = b; r[231] = b; r[238] = b; r[245] = b; r[252] = b; r[259] = b; r[266] = b; r[273] = b; r[280] = b; r[287] = b; r[294] = b; r[301] = b; r[308] = b; r[315] = b; r[322] = b; r[329] = b; r[336] = b; r[343] = b; r[350] = b; r[357] = b; r[364] = b; r[371] = b; r[378] = b; r[385] = b; r[392] = b; r[399] = b; r[406] = b; r[413] = b; r[420] = b; r[427] = b; r[434] = b; r[441] = b; r[448] = b; r[455] = b; r[462] = b; r[469] = b; r[476] = b; r[483] = b; r[490] = b; r[497] = b; r[504] = b; r[511] = b; r[518] = b; r[525] = b; r[532] = b; r[539] = b; r[546] = b; r[553] = b; r[560] = b; r[567] = b; r[574] = b; r[581] = b; r[588] = b; r[595] = b; r[602] = b; r[609] = b; r[616] = b; r[623] = b; r[630] = b; r[637] = b; r[644] = b; r[651] = b; r[658] = b; r[665] = b; r[672] = b; r[679] = b; r[686] = b; r[693] = b; r[700] = b; r[707] = b; r[714] = b; r[721] = b; r[728] = b; r[735] = b; r[742] = b; r[749] = b; r[756] = b; r[763] = b; r[770] = b; r[777] = b; r[784] = b; r[791] = b; r[798] = b; r[805] = b; r[812] = b; r[819] = b; r[826] = b; r[833] = b;
  int t = gk + b; int x = *p; gk = t; *q = 7; return x * 100 + gk;
}
#endif

#if !defined(STOREALIAS_HARNESS) || defined(STOREALIAS_KEEP_DEAD)
NOIPA int w_dead(int *p, int *a, int c) { int t = a[3]; gi = t; if (c) { t = *p * 3; } gi = t - 7; touch(); return gi; }
NOIPA int w_dead2(int *p, int *a, int c) { int t = a[3]; gi = t; if (c) { t = (*p - a[1]) * 3; gi = t; } gi = t - 7; touch(); return gi; }
#endif

int main(void) {
  int r, q;
  long lr;
  r = s_param(1, &gi, -77); printf("s_param %d\n", r);
  r = s_const(&gi, 160); printf("s_const %d\n", r);
  r = s_once(1, &gi, -77); printf("s_once %d\n", r);
  r = s_plain(3, &gi, 4); printf("s_plain %d\n", r);
  r = s_short(3, &gs, -9); printf("s_short %d\n", r);
  r = s_byte(3, &gb, 200); printf("s_byte %d\n", r);
  lr = s_long(3, &gl, -11); printf("s_long %ld\n", lr);
  r = s_narrow(0x1234, (char *)&gi, 0x77); printf("s_narrow %d\n", r);
  r = s_branch(3, &gi, 8, 1); q = s_branch(3, &gi, 8, 0); printf("s_branch %d %d\n", r, q);
  r = s_loop(5, &gi, 1); q = s_loop(5, &gi, 0); printf("s_loop %d %d\n", r, q);
  r = s_cmp(5, &gi, -1); printf("s_cmp %d\n", r);
  gi = 4; r = l_keep(&gi, 8); printf("l_keep %d %d\n", r, gi);
  gi = 5; r = l_keepc(&gi, 6, 1); q = l_keepc(&gi, 7, 0); printf("l_keepc %d %d\n", r, q);
  gi = 2; r = l_reload(&gi, 50); printf("l_reload %d\n", r);
  gi = 3; r = l_twice(&gi, 6); printf("l_twice %d\n", r);
  w_order(3, &gi, 9); printf("w_order %d %d\n", gi, touched);
  r = w_order2(4, &gi, 8); printf("w_order2 %d %d %d\n", r, gi, touched);
  gi = 3; r = r_order(&gi, 9); printf("r_order %d %d\n", r, gi);
  gi = 5; w_inc(&gi, 9); printf("w_inc %d\n", gi);
  gi = 4; r = r_cond(&gi, 9, 1); q = r_cond(&gi, 20, 0); printf("r_cond %d %d %d\n", r, q, gi);
  gi = 4; r = r_two(&gi, 9, 6); q = r_two(&gi, 3, 8); printf("r_two %d %d %d %d\n", r, q, gi, gj);
  gi = 4; r = r_loop(&gi, 1, 100); q = r_loop(&gi, 0, 5); printf("r_loop %d %d %d\n", r, q, gi);
  gi = 4; r = e_after(&gi, 2, 3); printf("e_after %d %d\n", r, gi);
  gi = 1; r = e_call(5, &gi); printf("e_call %d %d\n", r, gi);
  {
    int ia[4] = {1, 2, 3, 4};
    gi = 9; r = e_reg(&gi, ia, 1); printf("e_reg %d %d %d\n", r, gi, gj);
    gi = 9; r = w_dead(&gi, ia, 1); printf("w_dead %d %d\n", r, gi);
    gi = 9; r = w_dead2(&gi, ia, 1); printf("w_dead2 %d %d\n", r, gi);
  }
  gi = 4; lr = w_epi(1, &gi, 3); r = (int)w_epi(0, &gi, 5); printf("w_epi %ld %d %d\n", lr, r, gi);
  gk = 4; gj = 5; r = w_big(&gj, &gk, rbuf, 2); printf("w_big %d %d %d\n", r, gk, touched);
  gk = 4; r = w_big2(&gj, &gk, rbuf, 2); printf("w_big2 %d %d %d\n", r, gk, rbuf[833]);
  return 0;
}
