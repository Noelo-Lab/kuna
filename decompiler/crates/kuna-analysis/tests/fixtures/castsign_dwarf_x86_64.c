/* (kuna castsign) A variable the source declares unsigned and the program only
 * compares signed.  `castsign` re-declares such a variable when nothing says
 * what it is, but here the DWARF says `unsigned long`, and kuna locks a DWARF
 * local's type: the declaration and its `(long)n` must stay.  Prints `1 2 0`.
 *
 * Built with:
 *   gcc -O0 -g -o castsign_dwarf_gcc_O0_x86_64 castsign_dwarf_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>

__attribute__((noinline, noipa)) long sign_of(const char *s, const char *t) {
  unsigned long n = strtoul(s, 0, 0);
  if ((long)n < 0)
    n = strtoul(t, 0, 0);
  if ((long)n < -5)
    return 2;
  if ((long)n >= 0)
    return 1;
  return 0;
}

int main(void) {
  printf("%ld %ld %ld\n", sign_of("7", "0"), sign_of("-1", "-9"), sign_of("-1", "-2"));
  return 0;
}
