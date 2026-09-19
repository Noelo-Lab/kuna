/* protoorder fixture: byte-pointer votes on buffers the caller fills with wide
 * constants (x86-64).
 *
 * `peek` reads one byte through its argument, so its recovered parameter is
 * `unsigned char *`.  `fill_words` stores an eight-byte constant at each word of
 * the buffer in a loop, and `fill_many` stores 520 eight-byte constants at fixed
 * places, more addresses than the vote's access walk follows.  Taken as a vote,
 * the callee's type made either caller's parameter `unsigned char *`, and
 * SplitDatatype printed every eight-byte store as eight byte stores.  Both votes
 * are refused.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -o protoorder_widefill_x86_64 protoorder_widefill_x86_64.c
 */
#include <stdio.h>
#define NI __attribute__((noinline))
#define S(i) *(long *)(buf + 16 + 8 * (i)) = 0x2020726174737575L + (i) * 0x101010101L;
#define S8(i) S(i) S(i + 1) S(i + 2) S(i + 3) S(i + 4) S(i + 5) S(i + 6) S(i + 7)
#define S64(i) S8(i) S8(i + 8) S8(i + 16) S8(i + 24) S8(i + 32) S8(i + 40) S8(i + 48) S8(i + 56)
NI int peek(unsigned char *p, int k) { return *p * k + 1; }
NI int fill_words(char *buf, int n) {
  for (int i = 0; i < n; i++) ((volatile long *)buf)[i] = 0x0102030405060708L;
  return peek((unsigned char *)buf, n * 5) + n;
}
NI int fill_many(char *buf, int n) {
  S64(0) S64(64) S64(128) S64(192) S64(256) S64(320) S64(384) S64(448) S8(512)
  return peek((unsigned char *)buf, n * 5) + n;
}
static char b[8 * 700];
int main(int argc, char **argv) {
  int r = fill_words(b, argc + 3) + fill_many(b, argc);
  printf("%d %d\n", r, b[9]);
  return 0;
}
