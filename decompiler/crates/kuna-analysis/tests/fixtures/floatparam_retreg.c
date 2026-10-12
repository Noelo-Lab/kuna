/* A callee recovered returning in rax while its caller reads xmm0: v1 returns
   (float)geti(), and main reads the float in xmm0. The return a decompile
   states in another register says nothing of what the call hands back.
   clang -O2 -o floatparam_retreg_clang_O2 floatparam_retreg.c && strip floatparam_retreg_clang_O2 */
#include <stdio.h>
#define NI __attribute__((noinline))
int gj = 7; unsigned gu = 3000000000u; long gl = -5; short gs = -9; unsigned long gul = 0x8000000000000005ul;
NI int geti(void) { return gj; }
NI int geti2(int x) { return gj * x + 1; }
NI unsigned getu(void) { return gu; }
NI long getl(void) { return gl; }
NI short gets_(void) { return gs; }
NI unsigned long getul(void) { return gul; }
NI void use(int *p, float f, float *q) { *q = f; *p = 1; }
NI void used(int *p, double d, double *q) { *q = d; *p = 1; }
NI float v1(void) { return (float)geti(); }
NI float v2(void) { return (float)geti2(3); }
NI float v3(void) { return (float)getu(); }
NI double v4(void) { return (double)getl(); }
NI float v5(void) { return (float)gets_(); }
NI float v6(void) { return (float)getul(); }
NI void v7(int *p, float *q) { use(p, (float)geti(), q); }
NI void v8(int *p, double *q) { used(p, (double)getu(), q); }
NI void v9(int *p, float *q) { use(p, geti(), q); }
NI float v10(void) { float f = geti(); return f * 2.0f; }
NI void v11(float *q) { *q = geti(); }
NI double v12(int c) { return c ? (double)geti() : 1.5; }
NI void v13(int *p, float *q) { use(p, (float)geti2(5), q); }


int main(void) { int k; float q; double d;
 printf("%g %g %g %g %g %g ", v1(), v2(), v3(), v4(), v5(), v6());
 v7(&k, &q); printf("%g ", q); v8(&k, &d); printf("%g ", d); v9(&k, &q); printf("%g ", q); printf("%g ", v10()); v11(&q); printf("%g ", q); printf("%g %g ", v12(1), v12(0)); v13(&k, &q); printf("%g\n", q); return 0; }
