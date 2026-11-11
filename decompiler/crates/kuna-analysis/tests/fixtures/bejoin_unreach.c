/* int callees whose callers do not let the call return: the code laid out
 * after each call belongs to another path and reads the second return
 * register for a value of its own. */
typedef unsigned long long u64;
typedef unsigned u32;
#define NI __attribute__((noinline))
volatile u32 vs;
NI u32 hi(u64 a, u64 b) { vs = 0; return (a + b) >> 32; }
NI u32 hj(u64 a, u64 b) { vs = 1; return (a + b) >> 32; }
NI u32 u_unr(u64 a, u64 b, int c) {
  if (__builtin_expect(c == 5, 1)) { vs = c; hi(a, b); __builtin_unreachable(); }
  return (u32)a + c;
}
NI u32 u_sw(u64 a, u64 b, int c) {
  switch (c) {
  case 0: return (u32)a + 1;
  case 1: vs = 1; hj(a, b); __builtin_unreachable();
  case 2: return (u32)a * 3;
  case 3: return (u32)a - 7;
  case 4: vs = 4; hj(b, a); __builtin_unreachable();
  case 5: return (u32)a ^ 0x55;
  case 6: return (u32)a << 2;
  default: return 0;
  }
}
u32 c_hi(u64 a, u64 b) { return hi(a, b) + hj(a, b); }
void _start(void) { for (;;); }
