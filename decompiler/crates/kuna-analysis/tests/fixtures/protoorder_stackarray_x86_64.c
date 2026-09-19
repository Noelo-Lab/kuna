/* (kuna) `protoorder` fixture -- a stack array whose address feeds callees
 * that read it through narrower pointers (`geti` as `int *`, `gets2` as
 * `short *`).  A pointer vote at the array's address re-lays the frame:
 * `char buf[256]` renders as `char [20]` plus `unsigned int [61]` while the
 * loop still writes all 256 bytes into the first one.  The vote is refused at
 * a frame address, so `buf` stays one `char [264]` slot.
 *
 * Built with:  gcc -O0 -o protoorder_stackarray_x86_64 protoorder_stackarray_x86_64.c && strip protoorder_stackarray_x86_64
 */
#include <stdio.h>
#define NI __attribute__((noinline))
NI int geti(int *p) { return p[0] ^ p[3]; }
NI short gets2(short *p) { return p[1]; }
NI long diffp(char *a, char *b) {  /* a,b bytes; callee sees int* and short* */
  long d = a - b;
  int x = geti((int *)a);
  short y = gets2((short *)b);
  return d + x + y + (a + 5 - b);
}
NI long walk(char *p, long k) {
  long s = 0;
  for (long i = 0; i < k; i += 3) { s += geti((int *)(p + i)); p = p + 1; }
  return s + (long)(p - 7);
}
NI int *nexti(int *p) { return p + 2; }
NI long retarith(char *p) {
  char *r = (char *)nexti((int *)p);
  return (long)(r[1] + r[3]) + (long)(r + 5);
}
int main(int argc, char **argv) {
  char buf[256]; for (int i = 0; i < 256; i++) buf[i] = i * 3;
  printf("%ld %ld %ld\n", diffp(buf + argc + 20, buf + 1), walk(buf, 40 + argc), retarith(buf + argc) - (long)buf);
  return 0;
}
