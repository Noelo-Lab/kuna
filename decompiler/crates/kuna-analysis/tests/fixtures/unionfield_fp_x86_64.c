/* (kuna) One 8-byte union member read as a double (`movsd 0x8(%rdi)`) and as a
 * long converted to double (`cvtsi2sdq 0x8(%rdi)`).  `structsynth` types a field
 * from its reads; a `long` field made the double read print
 * `(double)a0->field_0x8`, a value conversion of bits the binary reinterprets,
 * so tag 2 returned 4612811918334230528.0 where the binary returns 2.5.
 *
 * Built with:  gcc -O2 -o unionfield_fp_x86_64 unionfield_fp_x86_64.c
 */
#include <stdio.h>
#define NI __attribute__((noinline, noipa))
struct V { int tag; union { int i; float f; double d; long l; unsigned char b[8]; } u; };
NI double vread(struct V *p) {
  switch (p->tag) {
  case 0: return p->u.i;
  case 1: return p->u.f;
  case 2: return p->u.d;
  case 3: return (double)p->u.l;
  default: return p->u.b[1];
  }
}
int main(void) {
  static struct V v;
  v.u.d = 2.5;
  for (v.tag = 0; v.tag < 5; v.tag++)
    printf("%d %a\n", v.tag, vread(&v));
  return 0;
}
