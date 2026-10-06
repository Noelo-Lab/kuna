/* Parameters passed in a floating-point register that the function only
   moves: stored through a pointer or into a field. A double copied into a
   global, or handed back in an integer register, keeps its integer type.
   negsink hands sink the bits of an integer in a float register, k3b hands
   use the bits iget3 returns in an integer register, and twicetrunc reads
   the float trunc16 computes on its bits.
   gcc -O0 -fno-asynchronous-unwind-tables -c -o floatparam_x86_64_O0.o floatparam.c
   clang --target=x86_64-linux-gnu -O2 -fno-asynchronous-unwind-tables -c -o floatparam_x86_64.o floatparam.c
   clang --target=x86_64-linux-gnu -O0 -fno-asynchronous-unwind-tables -c -o floatparam_x86_64_clang_O0.o floatparam.c
   clang --target=aarch64-linux-gnu -O2 -fno-asynchronous-unwind-tables -c -o floatparam_a64.o floatparam.c */
#define NI __attribute__((noinline))
struct rec { int n; double d; };
double gd;
NI void fs(int *p, double b, double *q) { *q = b; *p = 1; }
NI void ff(int *p, float b, float *q) { *q = b; *p = 1; }
NI void fr(struct rec *r, double b) { r->n = 1; r->d = b; }
NI void fy(int *p, double b) { gd = b; *p = 1; }
NI long bits(double b) { long x; __builtin_memcpy(&x, &b, 8); return x; }
NI void sink(float *o, float a, float b) { o[0] = a; o[1] = b; }
NI void negsink(const unsigned *p, float *o) { unsigned u = p[1] ^ 0x80000000u; float f; __builtin_memcpy(&f, &u, 4); sink(o, f, 2.0f); }
NI float trunc16(float x) { unsigned u; __builtin_memcpy(&u, &x, 4); u &= 0xffff0000u; float r; __builtin_memcpy(&r, &u, 4); return r; }
NI float twicetrunc(float x) { return trunc16(x) * 2.0f; }
int gj = 0x40490fdb;
NI int iget3(void) { return gj; }
NI void use(int *p, float f, float *q) { *q = f; *p = 1; }
NI void k3b(int *p, float *q) { int i = iget3(); float f; __builtin_memcpy(&f, &i, 4); use(p, f, q); }
