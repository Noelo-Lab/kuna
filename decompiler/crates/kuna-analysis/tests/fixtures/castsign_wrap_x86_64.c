/* (kuna castsign) Values the program compares signed, read with strtoul so a
 * test can pass the edges of the range.  The first five do unsigned arithmetic
 * on the value (a decrement, an increment, a difference, a self-update) whose
 * result the program compares signed.  That arithmetic wraps at 2^63 (2^31 for
 * dec_neg32) and would overflow if the value were declared signed, so at -O0,
 * where each is a stack slot, `castsign` must leave those declarations
 * unsigned.  sign_of and sign_of32 only compare, assign and mask the value, so
 * `castsign` declares it signed; peek also indexes a string with it, which at
 * -O1 is a register local only `castsign` re-declares.  The round-trip test in
 * kuna-cli/tests/decompile_all_cli.rs compiles the functions as kuna prints
 * them with gcc and clang at -O0 and -O2, and checks that each build prints
 * what this program prints:
 *
 *   dec_neg      0 1 1 5 5 0 0
 *   cnt_wrap     5 5 5 0 5 5 5
 *   spin         3 0 5
 *   count_down   0 1 2 5 5 0 0
 *   dec_neg32    0 13 15 15 0
 *   sign_of      1 2 0 1
 *   sign_of32    5 6 4 5
 *   peek         99 -1 -2 100
 *
 * Built with:
 *   gcc   -O0 -o castsign_wrap_gcc_O0_x86_64   castsign_wrap_x86_64.c
 *   clang -O0 -o castsign_wrap_clang_O0_x86_64 castsign_wrap_x86_64.c
 *   gcc   -O1 -o castsign_wrap_gcc_O1_x86_64   castsign_wrap_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef __clang__
#define KEEP __attribute__((noinline))
#else
#define KEEP __attribute__((noinline, noipa))
#endif

KEEP long dec_neg(const char *s) {
  unsigned long v = strtoul(s, 0, 0);
  long n = 0;
  while ((long)(v - 1) >= 0 && n < 5) {
    v -= 2;
    n++;
  }
  return n;
}

KEEP long cnt_wrap(const char *s) {
  unsigned long v = strtoul(s, 0, 0);
  long n = 0;
  while ((long)(v + 1) > (long)v && n < 5) {
    v++;
    n++;
  }
  return n;
}

KEEP long spin(const char *a, const char *b) {
  unsigned long now = strtoul(a, 0, 0);
  unsigned long deadline = strtoul(b, 0, 0);
  long n = 0;
  while ((long)(deadline - now) > 0 && n < 5) {
    now++;
    n++;
  }
  return n;
}

KEEP long count_down(const char *s) {
  unsigned long v = strtoul(s, 0, 0);
  long n = 0;
  while ((long)--v >= 0 && n < 5)
    n++;
  return n;
}

KEEP long dec_neg32(const char *s) {
  unsigned int v = strtoul(s, 0, 0);
  long n = 0;
  while ((int)(v - 1) >= 0 && n < 5) {
    v -= 2;
    n++;
  }
  if ((v & 0x10) != 0)
    n += 10;
  return n;
}

KEEP long sign_of(const char *s, const char *t) {
  unsigned long v = strtoul(s, 0, 0);
  if ((long)v < 0)
    v = strtoul(t, 0, 0);
  if ((long)v < -5)
    return 2;
  if ((long)v >= 0)
    return 1;
  return 0;
}

KEEP long sign_of32(const char *s, const char *t) {
  unsigned int v = strtoul(s, 0, 0);
  long r = 0;
  if ((int)v < 0)
    v = strtoul(t, 0, 0);
  if ((v & 0x10) != 0)
    r += 4;
  if ((int)v < -5)
    r += 2;
  if ((int)v >= 0)
    r += 1;
  return r;
}

KEEP long peek(const char *s, const char *t) {
  unsigned long k = strtoul(t, 0, 0);
  if ((long)k < 0)
    return -1;
  if ((long)k >= (long)strlen(s))
    return -2;
  return s[k];
}

static const char *const W64[] = {"0", "1", "2", "0x7fffffffffffffff", "0x8000000000000000",
                                  "0x8000000000000001", "0xffffffffffffffff"};
static const char *const W32[] = {"0", "5", "0x7fffffff", "0x80000000", "0x80000001"};

int main(void) {
  printf("dec_neg     ");
  for (int i = 0; i < 7; i++)
    printf(" %ld", dec_neg(W64[i]));
  printf("\ncnt_wrap    ");
  for (int i = 0; i < 7; i++)
    printf(" %ld", cnt_wrap(W64[i]));
  printf("\nspin         %ld %ld %ld\ncount_down  ", spin("0x7ffffffffffffffe", "0x8000000000000001"),
         spin("0x8000000000000001", "0x7ffffffffffffffe"), spin("0xfffffffffffffffe", "0x10"));
  for (int i = 0; i < 7; i++)
    printf(" %ld", count_down(W64[i]));
  printf("\ndec_neg32   ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", dec_neg32(W32[i]));
  printf("\nsign_of      %ld %ld %ld %ld\nsign_of32    %ld %ld %ld %ld", sign_of(W64[3], W64[0]),
         sign_of(W64[4], "0xfffffffffffffffa"), sign_of(W64[6], "0xfffffffffffffffe"), sign_of(W64[5], W64[3]),
         sign_of32(W32[2], W32[0]), sign_of32(W32[3], "0xfffffffa"), sign_of32("0xffffffff", "0xfffffffe"),
         sign_of32(W32[4], "0x7ffffff0"));
  printf("\npeek         %ld %ld %ld %ld\n", peek("abcd", "2"), peek("abcd", W64[4]), peek("abcd", W64[3]),
         peek("abcd", "3"));
  return 0;
}
