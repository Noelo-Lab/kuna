#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct S { int i0; char c4, c5, c6, c7, c8, c9, c10, c11; };

__attribute__((noipa)) void sink(struct S *s) {
  s->c8 = (char)(s->c8 + 0x11);
}

__attribute__((noinline)) unsigned intospan(struct S *s, int w) {
  unsigned v;
  memcpy(&v, &s->c7, 4);
  s->c9 = w;
  return v;
}

__attribute__((noinline)) unsigned otherptr(struct S *s, struct S *t, int w) {
  unsigned v;
  memcpy(&v, &s->c7, 4);
  t->c9 = w;
  return v;
}

__attribute__((noinline)) unsigned acrosscall(struct S *s) {
  unsigned v;
  memcpy(&v, &s->c7, 4);
  sink(s);
  return v;
}

__attribute__((noinline)) unsigned plain(struct S *s) {
  unsigned v;
  memcpy(&v, &s->c7, 4);
  return v;
}

int main(int argc, char **argv) {
  struct S s;
  unsigned char *b = (unsigned char *)&s;
  for (int i = 0; i < (int)sizeof s; i++) b[i] = (unsigned char)(i * 37 + 0x81);
  int w = argc > 1 ? (int)strtol(argv[1], 0, 0) : 0x5a;
  unsigned x = intospan(&s, w), y = otherptr(&s, &s, w + 1), z = acrosscall(&s), p = plain(&s);
  printf("%08x %08x %08x %08x\n", x, y, z, p);
  return 0;
}
