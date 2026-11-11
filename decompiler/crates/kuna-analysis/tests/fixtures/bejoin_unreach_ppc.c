/* int callees whose callers do not let the call return, built for PowerPC
 * with the local symbols discarded (-Wl,-x) or with gcc's hot/cold
 * partition: the code right after each call is another function or a rare
 * block the hot part enters, and reads the second return register for a
 * value of its own. */
typedef unsigned long long u64;
typedef unsigned u32;
#define NI __attribute__((noinline))
volatile u32 vs;
int gsink;
NI __attribute__((cold)) void cold_log(int x) { vs = x; }
NI u32 hi(u64 a, u64 b) { vs = 0; return (a + b) >> 32; }
NI u32 u_tail(u64 a, u64 b, int c) {
  if (__builtin_expect(c == 5, 0)) { vs = c; hi(a, b); __builtin_unreachable(); }
  return (u32)a + c;
}
static NI int s_next(int a, int b) { return a * 3 + b; }
NI int u_cold(int a, int b, int c) {
  if (c == 5) { cold_log(a); hi(a, b); __builtin_unreachable(); }
  if (c == 7) { cold_log(b); return b * 13 + gsink; }
  return a + c;
}
u32 c_hi(u64 a, u64 b) { return hi(a, b) + s_next(a, b); }
void _start(void) { for (;;); }
