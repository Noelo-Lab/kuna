/* (kuna returnuncomputed) A 64-bit value a function builds in its one return
 * register from two 32-bit halves is returned whole, and both arguments that
 * feed it stay parameters.  Each join_ function puts one half in the high 32
 * bits of the return and one in the low 32; join_after_call passes the address
 * of its low half to waitpid first (a pid that does not exist, so the call leaves
 * it alone), and join_hi_lo, whose high half is the first argument, is the
 * control that was always recovered right.  The round-trip test in
 * kuna-cli/tests/decompile_all_cli.rs compiles the functions as kuna prints them
 * and checks that each build prints what this program prints:
 *
 *   1234567800000005 fedcba98ffffffff
 *   2222222211111111 800000007fffffff
 *   6666666655555555 fffffffe00000001
 *   fffffffe00000005 ffffffff80000000
 *   1234567900000005 ffffffff00000000
 *   1234567800000006 fedcba9800000000
 *   1234567800000005 fedcba98ffffffff
 *   0000000512345678 fffffffffedcba98
 */
#include <stdio.h>
#include <sys/wait.h>

#if defined(__clang__)
#define NI __attribute__((noinline))
#else
#define NI __attribute__((noinline, noipa))
#endif
typedef unsigned long u64;

NI u64 join_lo_hi(unsigned lo, unsigned hi) { return ((u64)hi << 32) | lo; }
NI u64 join_third(unsigned a, unsigned b, unsigned c) { return ((u64)c << 32) | b; }
NI u64 join_sixth(unsigned a, unsigned b, unsigned c, unsigned d, unsigned e, unsigned f) { return ((u64)f << 32) | e; }
NI long join_signed(int lo, int hi) { return ((long)hi << 32) | (unsigned)lo; }
NI u64 join_hi_sum(unsigned a, unsigned b) { return ((u64)(a + 1) << 32) | b; }
NI u64 join_lo_sum(unsigned a, unsigned b) { return ((u64)b << 32) | (a + 1); }
NI u64 join_after_call(unsigned init, unsigned hi) {
  int st = init;
  waitpid(0x7ffffff0, &st, WNOHANG);
  return ((u64)hi << 32) | (unsigned)st;
}
NI u64 join_hi_lo(unsigned hi, unsigned lo) { return ((u64)hi << 32) | lo; }

int main(void) {
  printf("%016lx %016lx\n", join_lo_hi(5, 0x12345678), join_lo_hi(0xffffffffu, 0xfedcba98u));
  printf("%016lx %016lx\n", join_third(9, 0x11111111, 0x22222222), join_third(0, 0x7fffffff, 0x80000000u));
  printf("%016lx %016lx\n", join_sixth(1, 2, 3, 4, 0x55555555, 0x66666666), join_sixth(1, 2, 3, 4, 1, 0xfffffffeu));
  printf("%016lx %016lx\n", (u64)join_signed(5, -2), (u64)join_signed((int)0x80000000u, -1));
  printf("%016lx %016lx\n", join_hi_sum(0x12345678, 5), join_hi_sum(0xfffffffeu, 0));
  printf("%016lx %016lx\n", join_lo_sum(5, 0x12345678), join_lo_sum(0xffffffffu, 0xfedcba98u));
  printf("%016lx %016lx\n", join_after_call(5, 0x12345678), join_after_call(0xffffffffu, 0xfedcba98u));
  printf("%016lx %016lx\n", join_hi_lo(5, 0x12345678), join_hi_lo(0xffffffffu, 0xfedcba98u));
  return 0;
}
