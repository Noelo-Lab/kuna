/* An int callee whose caller does not let the call return: ARM gcc lays the
 * literal pool out right after the call, and its words decode as
 * instructions that read the second return register. */
typedef unsigned long long u64;
typedef unsigned u32;
#define NI __attribute__((noinline))
volatile u32 vs;
const char *volatile msg;
NI u32 hi(u64 a, u64 b) { u64 s = a + b; vs = (u32)s; return s >> 32; }
NI u32 u_pool(u64 a, u64 b, int c) {
  if (c == 5) { msg = "five"; hi(a, b); __builtin_unreachable(); }
  msg = "other";
  return (u32)a + c;
}
u32 c_hi(u64 a, u64 b) { return hi(a, b) + 1; }
void _start(void) { for (;;); }
