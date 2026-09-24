/* (kuna castsign) A variable the source declares unsigned and the program only
 * reads signed.  `castsign` re-declares such a variable when nothing says what
 * it is, but here the DWARF says `unsigned long`, and kuna locks a DWARF local's
 * type: the declaration and its `(long)n` must stay.  Prints `3 0`.
 *
 * Built with:
 *   gcc -O0 -g -o castsign_dwarf_gcc_O0_x86_64 castsign_dwarf_x86_64.c
 */
#include <stdio.h>
#include <string.h>

__attribute__((noinline, noipa)) long tail_blanks(char *s) {
  unsigned long n = strlen(s) - 1;
  long c = 0;
  while ((long)n >= 0 && s[n] == ' ') {
    s[n] = 0;
    n--;
    c++;
  }
  return c;
}

int main(void) {
  char a[] = "ab   ", b[] = "ab";
  printf("%ld %ld\n", tail_blanks(a), tail_blanks(b));
  return 0;
}
