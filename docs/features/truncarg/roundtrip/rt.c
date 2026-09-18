#include <stdio.h>
#define NI __attribute__((noinline))
NI void sink(unsigned x) { printf("%x\n", x); }
NI void sinki(int x) { printf("%d\n", x); }
NI void f(unsigned v) { sink((unsigned short)(v * 3u)); }
NI void k(unsigned v) { sink((unsigned char)(v + 1u)); }
NI void a2(unsigned v, unsigned u) { sink((unsigned short)(v + u)); }
NI void neg(unsigned v) { sink((unsigned short)-v); }
NI void ci(unsigned v) { sinki((unsigned short)(v * 5u)); }
NI void h(unsigned v) { sink((unsigned short)v); }
NI void bl(unsigned char *p) { sink(p[3]); }
NI void bc(char *p) { sink((unsigned char)*p); }
static const unsigned vals[] = {0u, 1u, 0x5555u, 0x5556u, 0x7777u, 0x7fffu, 0x8000u, 0xffffu, 0x10000u, 0x10001u,
  0x1ffffu, 0x2aaabu, 0x3333u, 0x3334u, 0xfeu, 0xffu, 0x100u, 0x7fu, 0x80u, 0xdeadbeefu, 0x7fffffffu, 0x80000000u, 0xffffffffu};
int main(void) {
  for (unsigned i = 0; i < sizeof vals / sizeof vals[0]; i++) {
    unsigned v = vals[i];
    f(v); k(v); a2(v, v >> 3); neg(v); ci(v); h(v);
  }
  for (unsigned i = 0; i < 256; i++) {
    unsigned char buf[8];
    for (unsigned j = 0; j < sizeof buf; j++) buf[j] = (unsigned char)i;
    ((void (*)(void *))bl)(buf);
    ((void (*)(void *))bc)(buf);
  }
  return 0;
}
