/* clang 14 -O2 --target=aarch64-linux-gnu -c zerofillreturn.c -o zerofillreturn_a64.o */
__attribute__((noinline)) double scale(int a, const double *g) { return *g * 1.5 + a; }
__attribute__((noinline)) double recip(int a) { return a > 0 ? 1.0 / a : 0.0; }
__attribute__((noinline)) double tail(int a, const double *g) { return scale(a + 1, g); }
__attribute__((noinline)) int floor_add(int a, const double *g) { return (int)(scale(a, g) + recip(a)); }
