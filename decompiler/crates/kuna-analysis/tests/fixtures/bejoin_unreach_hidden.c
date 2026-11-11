/* int callees called ahead of __builtin_unreachable(), linked with the local
 * symbols discarded (-Wl,-x): the static function laid out after each call is
 * reached only through a pointer (s_ptr) or a tail call from another function
 * (s_tl), and gcc's hot/cold partition leaves u_cold's rare part, which the
 * hot part enters after the call, with no symbol of its own. */
typedef unsigned long long u64;
typedef unsigned u32;
#define NI __attribute__((noinline))
volatile u32 vs;
int gsink;
void (*volatile gfp)(void);
NI __attribute__((cold)) void cold_log(int x) { vs = x; }
#define HI(n) NI u32 n(u64 a, u64 b) { u64 s = a + b; vs = (u32)s; if (vs == 77) gfp(); return s >> 32; }
HI(hi_p)
HI(hi_t)
HI(hi_c)
NI u32 u_ptr(u64 a, u64 b, int c) {
  if (__builtin_expect(c == 5, 0)) { hi_p(a, b); __builtin_unreachable(); }
  return (u32)a + c;
}
static NI int s_ptr(int a, int b) { return a * 3 + b; }
int (*volatile gp)(int, int) = s_ptr;
NI u32 u_tl(u64 a, u64 b, int c) {
  if (__builtin_expect(c == 7, 0)) { hi_t(a, b); __builtin_unreachable(); }
  return (u32)a + c * 3;
}
static NI int s_tl(int a, int b) { gsink = b; return a * 5 + b; }
int t_tl(int a, int b) { return s_tl(a + 1, b); }
NI int u_cold(int a, int b, int c) {
  if (c == 5) { cold_log(a); hi_c(a, b); __builtin_unreachable(); }
  if (c == 7) { cold_log(b); return b * 13 + gsink; }
  return a + c;
}
u32 c_hi(u64 a, u64 b) { return hi_p(a, b) + hi_t(a, b) + hi_c(a, b) + 1; }
void _start(void) { for (;;); }
