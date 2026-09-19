/* protoorder fixture: stores through a pointer whose callee reads it as floats
 * (x86-64).
 *
 * `dsum`, `norm` and `use` read their argument as `double *`, `struct P *` and
 * `struct M *`.  Each caller writes the pointee with integer bits first: `u1`
 * and `u2` through a union a callee reads as doubles, `s3` and `cp3` by
 * `memcpy` from integer parameters, `cp1` by struct assignment and `cp5` by
 * `memcpy` of an integer into the float member.  A pointer vote from the callee
 * printed those stores as value conversions (`*a0 = (double)(a1 + 1)`,
 * `a0->field_0x4 = (float)v2`), the payload NaN as `NAN`, and gave the
 * integer-register parameters of `s3` a `double` type.  The vote is refused
 * where the caller's stores disagree with the pointee, so every store stays
 * bitwise.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -o protoorder_floatpointee_x86_64 protoorder_floatpointee_x86_64.c
 */
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))
union U { long l[4]; double d[4]; };
struct P { double x, y; };
struct M { int i; float f; double d; };
NI double dsum(double *p) { return p[0] + p[1]; }
NI double norm(struct P *p) { return p->x * p->x + p->y * p->y; }
NI double use(struct M *m) { return m->i + m->f * 2.0f + m->d; }
NI double u1(union U *u, long v) { u->l[0] = v + 1; u->l[1] = v >> 1; return dsum(u->d); }
NI double u2(union U *u) { long t = u->l[2]; u->l[0] = t * 3; u->l[1] = 0x7ff0000000000001L; return dsum(u->d); }
NI double s3(struct P *dst, long a, long b) { memcpy(&dst->x, &a, 8); memcpy(&dst->y, &b, 8); return norm(dst); }
NI double cp1(struct M *dst, const struct M *src) { *dst = *src; return use(dst); }
NI double cp3(struct M *dst, long a, long b) { memcpy(dst, &a, 8); memcpy(&dst->d, &b, 8); return use(dst); }
NI double cp5(struct M *dst, unsigned v) { unsigned w = v * 2 + 1; memcpy(&dst->f, &w, 4); dst->i = v; dst->d = 0; return use(dst); }
int main(int argc, char **argv) {
  union U u = {{1, 2, 3, 4}};
  struct P p;
  struct M a = {1, 2, 3}, b;
  double r = u1(&u, argc) + u2(&u) + s3(&p, 0x4008000000000000L + argc, 0x4010000000000000L);
  r += cp1(&b, &a) + cp3(&b, 0x4000000000000001L, 0x3ff0000000000000L) + cp5(&b, 0x1fc00000u + argc);
  printf("%g %lx %lx\n", r, u.l[0], u.l[1]);
  return 0;
}
