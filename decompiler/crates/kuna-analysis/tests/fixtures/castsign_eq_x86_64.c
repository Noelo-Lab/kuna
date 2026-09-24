/* (kuna castsign) Values the program compares signed and also compares for
 * equality with a constant whose top bit is set.  kuna may print such a constant
 * in decimal with no suffix (`3000000000`, `10000000000000000000`), and a
 * decimal literal of 2^31 or more has a C type wider than `int` (2^63 or more,
 * wider than `long`).  `v == 3000000000` converts `v` to that type: zero-extended
 * while `v` is unsigned, which is what the binary compares, sign-extended once
 * it is declared signed, which makes the comparison false for every input.  So
 * `castsign` must leave `v` unsigned in d_eq32, c_eq64, c_ne64 and c_or64 (the
 * constant meets the value after a `|`).  c_eq7 compares with a small
 * constant, whose type is `int`, and is declared signed.  The round-trip test in
 * kuna-cli/tests/decompile_all_cli.rs compiles the functions as kuna prints them
 * with gcc and clang at -O0 and -O2, and checks that each build prints what this
 * program prints:
 *
 *   d_eq32   2 1 0
 *   c_eq64   2 1 0 1 0
 *   c_ne64   0 1 0 1 0
 *   c_or64   2 1 0 1 0
 *   c_eq7    1 2 0 1 0
 *
 * Built with:
 *   gcc   -O0 -o castsign_eq_gcc_O0_x86_64   castsign_eq_x86_64.c
 *   clang -O0 -o castsign_eq_clang_O0_x86_64 castsign_eq_x86_64.c
 */
#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>

#ifdef __clang__
#define KEEP __attribute__((noinline))
#else
#define KEEP __attribute__((noinline, noipa))
#endif

KEEP long d_eq32(const unsigned int *p, const char *t) {
  unsigned int v = ntohl(*p);
  if (t[0] == 'x')
    v = ntohl(p[1]);
  if (v == 3000000000u)
    return 2;
  if ((int)v < 0)
    return 1;
  return 0;
}

KEEP long c_eq64(const char *s, const char *t) {
  unsigned long v = strtoul(s, 0, 0);
  if (t[0] == 'x')
    v = strtoul(t + 1, 0, 0);
  if (v == 10000000000000000000UL)
    return 2;
  if ((long)v < 0)
    return 1;
  return 0;
}

KEEP long c_ne64(const char *s, const char *t) {
  unsigned long v = strtoul(s, 0, 0);
  if (t[0] == 'x')
    v = strtoul(t + 1, 0, 0);
  if ((long)v < 0 && v != 10000000000000000000UL)
    return 1;
  return 0;
}

KEEP long c_or64(const char *s, const char *t) {
  unsigned long v = strtoul(s, 0, 0);
  if (t[0] == 'x')
    v = strtoul(t + 1, 0, 0);
  if ((v | 0x80000) == 10000000000000000000UL)
    return 2;
  if ((long)v < 0)
    return 1;
  return 0;
}

KEEP long c_eq7(const char *s, const char *t) {
  unsigned long v = strtoul(s, 0, 0);
  if (t[0] == 'x')
    v = strtoul(t + 1, 0, 0);
  if (v == 7)
    return 2;
  if ((long)v < 0)
    return 1;
  return 0;
}

#define F(ret, f) ((ret (*)())(void (*)())f)
static const char *const E[] = {"10000000000000000000", "0x8000000000000001", "5", "0xffffffffffffffff", "0"};
int main(void) {
  unsigned int a[2] = {htonl(3000000000u), 0}, b[2] = {htonl(0x80000001u), 0}, c[2] = {htonl(5), 0};
  printf("d_eq32   %ld %ld %ld\n", F(long, d_eq32)(a, "y"), F(long, d_eq32)(b, "y"), F(long, d_eq32)(c, "y"));
  printf("c_eq64  ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", F(long, c_eq64)(E[i], "y"));
  printf("\nc_ne64  ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", F(long, c_ne64)(E[i], "y"));
  printf("\nc_or64  ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", F(long, c_or64)(E[i], "y"));
  printf("\nc_eq7    %ld %ld %ld %ld %ld\n", F(long, c_eq7)("0x8000000000000000", "y"), F(long, c_eq7)("7", "y"),
         F(long, c_eq7)("6", "y"), F(long, c_eq7)("x", "x0xffffffffffffffff"), F(long, c_eq7)("0", "y"));
  return 0;
}
