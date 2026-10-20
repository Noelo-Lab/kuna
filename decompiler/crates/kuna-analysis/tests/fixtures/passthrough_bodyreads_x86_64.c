/*
 * passthrough_bodyreads_x86_64 -- an argument the callee's body takes is kept
 * at the call, whether the callee reads it itself, hands it to a declared
 * parameter, or is the calling function itself.
 *
 *     gcc -O2 -fno-inline -fno-pie -no-pie -fcf-protection=none \
 *         passthrough_bodyreads_x86_64.c -o passthrough_bodyreads_x86_64
 *
 * `mix` calls itself with `key` still in edx, untouched. `validfd` reads `fd`
 * only to pass it to fcntl's declared `int fd`, so its bounded entry decode
 * never sees it read: `check` forwards edi to it untouched, and `pick` loads
 * the value into edi and also compares it, so neither keeps the argument
 * unless the callee's own body says it takes it.
 */
#include <fcntl.h>
#include <stdio.h>
__attribute__((noinline)) unsigned mix(unsigned char *p, unsigned n, unsigned key) {
  p[n & 3] ^= (unsigned char)(key + 25);
  if (n == 0) return p[0] & 1;
  return !mix(p, (n - 1) & 7, key);
}
__attribute__((noinline)) int validfd(int fd) { return fcntl(fd, F_GETFD, 0) >= 0; }
__attribute__((noinline)) int check(int fd) { return validfd(fd) ? 7 : 0; }
__attribute__((noinline)) int pick(int *p) { int fd = *p; if (fd > 3) return validfd(fd); return -1; }
int main(int argc, char **argv) {
  unsigned char b[4] = {1, 2, 3, 4};
  int fd = argc + 2;
  return (int)mix(b, (unsigned)argc, (unsigned)argc * 3u) + check(argc) + pick(&fd) + (argv == 0);
}
