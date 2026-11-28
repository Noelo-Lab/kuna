/* armfloatargs variadic-double fixture: ARM hard-float double parameters a
 * function hands on in core registers with the doubleword `vmov rX,rY,dN`.
 *
 * arm-linux-gnueabihf-gcc -O2 -marm -mfpu=vfpv3-d16 -c armfloatargs_variadic.c -o armfloatargs_variadic_arm.o
 * arm-linux-gnueabihf-gcc -O2 -mthumb -mfpu=vfpv3-d16 -c armfloatargs_variadic.c -o armfloatargs_variadic_thumb.o
 *
 * Synthetic regression source, licensed with this repository (Apache-2.0).
 */
extern void consume(const char *, ...);
extern int hash64(unsigned long long);
extern void sinkw(unsigned, unsigned);

/* A double forwarded to a variadic callee, which takes it in r2:r3. */
__attribute__((noinline)) void show2(int n, double d) { consume("%d %lf", n, d); }
void caller(int n, double d) { show2(n, d); }
/* The same in d1, below an unused float in s0. */
__attribute__((noinline)) void show_d1(float f, double d) { consume("%lf", d); }
/* d0 in r2:r3, d1 on the stack. */
__attribute__((noinline)) void show3(double a, double b) { consume("%f %f", a, b); }
/* A double's bits handed to a 64-bit integer parameter in r0:r1. */
__attribute__((noinline)) int pun(double d) { unsigned long long b; __builtin_memcpy(&b, &d, 8); return hash64(b); }
/* Controls: two floats moved one by one, and a double the function computes. */
__attribute__((noinline)) void two_f(float a, float b) { unsigned x, y; __builtin_memcpy(&x, &a, 4); __builtin_memcpy(&y, &b, 4); sinkw(x, y); }
__attribute__((noinline)) void show_f(int n, float f) { consume("%d %f", n, (double)f); }
