/* (kuna for-loop initializer order) `parity` reads the strtoul result into `n`
 * before the loop; the loop counts `v` down from that same result.  The
 * for-header must not take `v = strtoul(...)` as its initializer, since that
 * would print the `v & 1` read ahead of the call that produces `v`.
 *
 * Prints, for 0 1 2 7 9 13 100:   0 2 1 5 5 5 5
 *
 * Built with:
 *   clang -O0 -o forinit_visible_read_clang_O0_x86_64 forinit_visible_read_x86_64.c
 */
#include <stdio.h>
#include <stdlib.h>

__attribute__((noinline)) long parity(const char *s) {
  unsigned long v = strtoul(s, 0, 0);
  long n = v & 1;
  while ((long)(v - 1) >= 0 && n <= 4) {
    n++;
    v -= 2;
  }
  return n;
}

int main(void) {
  const char *t[] = {"0", "1", "2", "7", "9", "13", "100"};
  for (int i = 0; i < 7; i++)
    printf("%ld ", parity(t[i]));
  printf("\n");
  return 0;
}
