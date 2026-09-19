/* protoorder fixture: a byte-pointer vote on a pointer the caller writes wider
 * (x86-64).
 *
 * `peek` reads one byte through its argument, so its recovered parameter is
 * `unsigned char *`.  `fill` stores the eight bytes of "ustar  " and a four-byte
 * count through the buffer before handing it to `peek`.  Taken as a vote, the
 * callee's type made `fill`'s parameter `unsigned char *`, and SplitDatatype
 * printed the eight-byte store as eight byte stores.  The vote is refused because
 * the caller's own accesses are wider than the pointee.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -o protoorder_narrowvote_x86_64 protoorder_narrowvote_x86_64.c
 */
#include <stdio.h>
#define NI __attribute__((noinline))
NI int peek(unsigned char *p, int k) { return *p * k + 1; }
NI int fill(char *buf, int n) { *(long *)(buf + 16) = 0x2020726174737575L; *(int *)(buf + 24) = n; return peek((unsigned char *)buf, n * 5) + n; }
int main(int argc, char **argv) {
  char b[32] = "x";
  int r = fill(b, argc);
  printf("%d %s\n", r, b + 16);
  return 0;
}
