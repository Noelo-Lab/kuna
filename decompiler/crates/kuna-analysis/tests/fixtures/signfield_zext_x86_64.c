/* (kuna) One 16-bit field read twice through the same pointer: once into a
 * signed comparison, and, after a call that forces a reload, zero-extended
 * (`movzwl`) into a 32-bit argument.  `structsynth` types the field from its
 * reads; when it took the signed read's type, the printed call sign-extended
 * what the binary zero-extends, and 0x9abc arrived as 4294941372.
 *
 * Built with:  gcc -O2 -o signfield_zext_x86_64 signfield_zext_x86_64.c
 */
#include <stdio.h>
#define NI __attribute__((noinline, noipa))
struct S { const char *p; unsigned int a; short h; };
unsigned acc;
NI void touch(struct S *s) { acc += s->a; }
NI void sink(unsigned int x) { acc = acc * 31u + x / 3u; }
NI int f(struct S *s) {
  int r = s->h < 3;
  touch(s);
  sink((unsigned short)s->h);
  return r + (int)(s->a / 5u) + (s->p != 0);
}
int main(void) {
  static struct S st;
  st.p = "x"; st.a = 2; st.h = (short)0x9abc;
  int r = f(&st);
  printf("%d %u\n", r, acc);
  return 0;
}
