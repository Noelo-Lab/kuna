/* An int callee called ahead of __builtin_unreachable() inside a try block:
 * the C++ landing pad laid out right after the call reads the exception
 * selector in the second return register. */
typedef unsigned long long u64;
typedef unsigned u32;
#define NI __attribute__((noinline))
extern "C" {
volatile u32 vs;
int gsink;
}
extern void may_throw(int);
extern "C" NI u32 hi(u64 a, u64 b) { u64 s = a + b; vs = (u32)s; return s >> 32; }
extern "C" NI u32 u_eh(u64 a, u64 b, int c) {
  try {
    may_throw(c);
    if (c == 5) { hi(a, b); __builtin_unreachable(); }
    may_throw(c + 1);
  } catch (int e) {
    return e + 1;
  } catch (long e) {
    return (u32)e + 2;
  }
  return (u32)a;
}
extern "C" u32 c_hi(u64 a, u64 b) { return hi(a, b) + 1; }
extern "C" void _start(void) { for (;;); }
