/* gcc -O2 -o zextarg_x86_64 zextarg_x86_64.c */
#include <stdio.h>
#include <stdlib.h>

__attribute__((noinline)) unsigned sinkl(long c) { return (unsigned)(c >> 2) ^ (unsigned)(c < 0); }

__attribute__((noinline)) void zext_arg(unsigned *p, unsigned x0, unsigned x1) {
  unsigned v = x0;
  if (x1 & 1) v = x1 * 3;
  p[0] = (unsigned)((long)(int)v >> 3);
  p[1] = sinkl((long)v);
}

__attribute__((noinline)) void sext_arg(unsigned *p, unsigned x, unsigned y) {
  unsigned v = x * 3 + y;
  if ((int)v < (int)y) p[2] = 1;
  p[1] = sinkl((long)(int)v);
}

int main(int argc, char **argv) {
  unsigned p[3] = {0};
  unsigned a = argc > 1 ? (unsigned)strtoul(argv[1], 0, 0) : 0x80000001u;
  zext_arg(p, a, 0);
  sext_arg(p, a, 1);
  printf("%x %x %x\n", p[0], p[1], p[2]);
  return 0;
}
