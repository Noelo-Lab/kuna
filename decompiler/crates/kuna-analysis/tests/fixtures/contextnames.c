/* Fixture for `contextnames`: loop counters and returned locals.
   gcc -O1 -fno-inline -o contextnames_x86_64 contextnames.c && strip contextnames_x86_64 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((noinline)) int matrix_sum(int *m, int rows, int cols, int depth) {
  int total = 0;
  for (int r = 0; r < rows; r++)
    for (int c = 0; c < cols; c++)
      for (int d = 0; d < depth; d++)
        total += m[(r * cols + c) * depth + d] ^ d;
  return total;
}

__attribute__((noinline)) int two_passes(const char *s, int n) {
  int hits = 0;
  for (int a = 0; a < n; a++)
    if (s[a] == 'x')
      hits++;
  for (int b = n; b > 0; b--)
    if (s[b - 1] == 'y')
      hits += 2;
  return hits;
}

__attribute__((noinline)) size_t walk(const char *p) {
  const char *q = p;
  while (*q)
    q++;
  return (size_t)(q - p);
}

__attribute__((noinline)) long checksum(const unsigned char *buf, long len) {
  long acc = 7;
  if (len < 0)
    return -1;
  for (long k = 0; k < len; k++)
    acc = acc * 31 + buf[k];
  return acc;
}

int main(int argc, char **argv) {
  int m[64];
  for (int x = 0; x < 64; x++)
    m[x] = x * argc;
  printf("%d %d %zu %ld\n", matrix_sum(m, 4, 4, 4), two_passes(argv[0], (int)strlen(argv[0])),
         walk(argv[0]), checksum((const unsigned char *)argv[0], argc));
  return 0;
}
