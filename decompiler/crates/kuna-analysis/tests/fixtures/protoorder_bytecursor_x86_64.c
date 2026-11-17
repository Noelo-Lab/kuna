/* protoorder fixture: wider callee votes on byte cursors (x86-64).
 *
 * `sum_words` reads its argument a word at a time, so its recovered parameter
 * is `int *`.  `scramble` reads and writes its buffer a byte at a time before
 * handing it on; taken as a vote, the callee's type printed every one of those
 * byte accesses through a cast (`((unsigned char *)a0)[v1]`), and the vote is
 * refused.  `xor_words` walks its buffer eight bytes at a time over a
 * loop-carried offset and finishes with a byte tail; its word accesses keep the
 * vote, where refusing it printed the buffer as a `long`.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O0 -o protoorder_bytecursor_x86_64 protoorder_bytecursor_x86_64.c
 */
#include <stdint.h>
#include <stdio.h>
uint32_t sum_words(uint32_t *w, size_t n) {
  uint32_t s = 0;
  for (size_t i = 0; i < n; i++) s += w[i] * (uint32_t)(i + 3);
  return s;
}
int scramble(unsigned char *buf, size_t n) {
  for (size_t i = 0; i < n; i++) buf[i] = buf[i] ^ 0x5a;
  return (int)sum_words((uint32_t *)buf, n / 4) + 1;
}
int xor_words(uint64_t *w, size_t n) {
  unsigned char *b = (unsigned char *)w;
  size_t i = 0;
  for (; i + 8 <= n; i += 8) *(uint64_t *)(b + i) ^= 0x0101010101010101ull;
  for (; i < n; i++) b[i] ^= 1;
  return (int)sum_words((uint32_t *)w, n / 4) + 7;
}
static uint64_t buf[16];
int main(int argc, char **argv) {
  printf("%d\n", scramble((unsigned char *)buf, argc) + xor_words(buf, argc + 8));
  return 0;
}
