#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((noinline)) unsigned inside(unsigned char *p, unsigned w) {
  unsigned v;
  memcpy(&v, p + 7, 4);
  p[8] = w;
  return v;
}

__attribute__((noinline)) unsigned below(unsigned char *p, unsigned w) {
  unsigned v;
  memcpy(&v, p + 7, 4);
  memcpy(p + 5, &w, 4);
  return v;
}

__attribute__((noinline)) unsigned after(unsigned char *p, unsigned w) {
  unsigned v;
  memcpy(&v, p + 7, 4);
  __asm__ volatile("" ::: "memory");
  p[11] = w;
  return v;
}

__attribute__((noinline)) unsigned before(unsigned char *p, unsigned w) {
  unsigned v;
  memcpy(&v, p + 7, 4);
  __asm__ volatile("" ::: "memory");
  p[6] = w;
  return v;
}

__attribute__((noinline)) unsigned long indexed(unsigned *a, long i, unsigned w) {
  unsigned long v;
  memcpy(&v, a + i, 8);
  a[i + 1] = w;
  return v;
}

__attribute__((noinline)) unsigned next(unsigned *a, long i, unsigned w) {
  unsigned v = a[i];
  __asm__ volatile("" ::: "memory");
  a[i + 1] = w;
  return v;
}

int main(int argc, char **argv) {
  unsigned char b[16];
  for (int i = 0; i < 16; i++) b[i] = (unsigned char)(i * 37 + 0x81);
  unsigned w = argc > 1 ? (unsigned)strtoul(argv[1], 0, 0) : 0x5a;
  unsigned x = inside(b, w), y = below(b, w), z = after(b, w), t = before(b, w);
  unsigned a[8];
  for (int i = 0; i < 8; i++) a[i] = 0x01020304u * (unsigned)(i + 1);
  unsigned long u = indexed(a, 2, w);
  unsigned n = next(a, 4, w);
  printf("%08x %08x %08x %08x %016lx %08x\n", x, y, z, t, u, n);
  return 0;
}
