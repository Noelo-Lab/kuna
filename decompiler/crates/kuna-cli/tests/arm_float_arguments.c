/* Synthetic regression source, licensed with this repository (Apache-2.0). */
extern int printf(const char *, ...);
__attribute__((noinline)) void show2(int n, double d) { printf("%d %lf\n", n, d); }
__attribute__((noinline)) void reverse(double d, int n) { printf("%d %lf\n", n, d); }
void caller(int n, double d) { show2(n, d); }
void reverse_caller(double d, int n) { reverse(d, n); }
__attribute__((noinline)) float narrow(double x) { return x + x; }
float use_narrow(double x) { return narrow(x) + 1.0f; }
__attribute__((noinline)) float cast_only(double x) { return (float)x; }
__attribute__((noinline)) float mixed_input(double x, float y) { return x > 0 ? y : (float)(x*x); }
__attribute__((noinline)) double multiple(int n, double x, double y) { return n + x + y; }
double use_multiple(int n, double x, double y) { return multiple(n, x, y); }
float wrap_narrow(double x) { return narrow(x); }
__attribute__((noinline)) float pair_narrow(double x, double y) { return x + y; }
float use_pair_narrow(double x, double y) { return pair_narrow(x, y) + 1.0f; }
__attribute__((noinline)) float mixed_narrow(double x, float y) { return x + y; }
float use_mixed_narrow(double x, float y) { return mixed_narrow(x, y) + 1.0f; }
