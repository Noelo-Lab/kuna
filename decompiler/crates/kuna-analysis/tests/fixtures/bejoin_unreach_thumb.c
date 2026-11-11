/* An int callee whose caller does not let the call return, built for
 * big-endian Thumb (armv5t, BE32): gcc lays the literal pool out right after
 * the call, and its word 0x00080123 decodes as `movs r0,r1`, which reads the
 * second return register. */
typedef unsigned long long u64;
typedef unsigned u32;
#define NI __attribute__((noinline))
volatile u32 vs;
int gsink;
NI u32 hi(u64 a, u64 b) { u64 s = a + b; vs = (u32)s; return s >> 32; }
NI u32 u_pool(u64 a, u64 b, int c) {
  if (__builtin_expect(c == 5, 0)) { gsink = 0x00080123; hi(a, b); __builtin_unreachable(); }
  return (u32)a + c;
}
u32 c_hi(u64 a, u64 b) { return hi(a, b) + 1; }
void _start(void) { for (;;); }
