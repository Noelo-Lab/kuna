/* Synthetic calls, aggregates, integer returns and soft controls. */
struct F2 { float a, b; };
struct F4 { float a, b, c, d; };
struct D2 { double a, b; };
extern double g(double);
extern double h(double);
extern int printf(const char *, ...);
volatile double sink;
volatile float sinkf;
__attribute__((noinline)) double scale(double x) { sink = x; return x * 1.5; }
__attribute__((noinline)) float scalef(float x) { sinkf = x; return x * 1.5f; }
__attribute__((noinline)) double fixed(void) { return 1.5; }
__attribute__((noinline)) float fixedf(void) { return 2.5f; }
double fwd(double x) { return scale(x); }
float fwdf(float x) { return scalef(x); }
double wrapper(double x) { return scale(x) + 0.5; }
double twocalls(double x) { scale(x); return scale(x * 3.0) + 1.0; }
float twocallsf(float x) { scalef(x); return scalef(x * 3.0f) + 1.0f; }
float fwdn(double x) { return (float)scale(x); }
double fwdw(float x) { return scalef(x); }
double twoargs(double x, double y) { return x * y; }
double mixargs(int n, double x, float f) { return n * x + f; }
double many(double a, double b, double c, double d, double e, double f, double g, double h, double i) { return a+b+c+d+e+f+g+h+i; }
int to_int(double x) { return (int)(x * 2.0); }
int keep(double *p, double x) { *p = x * 2.0; return 1; }
int keepf(float *p, float x) { *p = x * 2.0f; return 1; }
int cmp(double a, double b) { return a < b; }
double loopsum(const double *p, int n) { double s = 0; for (int i = 0; i < n; i++) s += p[i]; return s; }
double pick(double a, double b, int c) { if (c > 3) return a; if (c < 0) return b; return a + b; }
float lowbits(double x, unsigned lo) { union { double d; unsigned u[2]; } v; v.d = x; v.u[0] = lo; return (float)v.d; }
double setlow(double x, unsigned lo) { union { double d; unsigned u[2]; } v; v.d = x; v.u[0] = lo; return v.d; }
struct F2 mkf2(float x) { struct F2 r = { x, x * 2.0f }; return r; }
struct F4 mkf4(float x) { struct F4 r = { x, x * 2.0f, x * 3.0f, x * 4.0f }; return r; }
struct D2 mkd2(double x) { struct D2 r = { x, x * 2.0 }; return r; }
void show(double x) { printf("%f\n", x); }
double callext(double x) { return g(x) + h(x); }
long long ll(long long a) { return a * 3; }
