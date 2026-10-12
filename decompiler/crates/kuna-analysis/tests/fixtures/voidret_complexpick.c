/* clang 14 -O2 --target=aarch64-linux-gnu -c voidret_complexpick.c -o voidret_complexpick_a64.o */
#include <complex.h>
double gk;
__attribute__((noinline)) double cpart(double x) { return x * gk + 1.0; }
__attribute__((noinline)) double complex cboth(double x) { return CMPLX(x * gk, x - gk); }
__attribute__((noinline)) double pick(double x) { if (x > 0.25) return cpart(x); return creal(cboth(x)); }
__attribute__((noinline)) double user(double x) { return pick(x) * 2.0; }
