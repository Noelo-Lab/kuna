/* Calls that pass and return floats in xmm0, read whole or in part around the call.
   gcc 11.4 (Ubuntu 22.04): gcc -O2 -mavx2 -fno-optimize-sibling-calls -o floatcalls_gcc_O2_avx2 floatcalls.c   (not stripped) */
#define NI __attribute__((noinline))
volatile double dsink;
volatile float sink;
NI double scale(double x) { return x * 2.5 + 1.0; }
NI double from_int(int n) { return (double)n * 0.25; }
NI double halfsum(double a, double b) { return (a + b) * 0.5; }
NI double stated_arg(int n) { return scale((double)n); }
NI double whole_result(int n, double y) { double r = from_int(n); return halfsum(y, r); }
NI float fscale(float x) { return x * 3.0f; }
NI float fsum(const float *a, int n) { float s = 0; for (int i = 0; i < n; i++) s += a[i]; return s; }
NI float i2f_div(int a, int b) { return (float)(a % b); }
NI float twice(float x) { return x + x; }
NI void stores(const float *a, int n) { sink = fsum(a, n); sink = fscale((float)n); sink = fsum(a, n - 1) + sink; }
NI float scl(const float *a, int n) { float s = 0; for (int i = 0; i < n; i++) s += a[i] * (float)i; return s; }
NI void kills(int n) {
  float f[16];
  for (int i = 0; i < 16; i++) f[i] = (float)(i * n);
  sink = i2f_div(n, 3);
  sink = scl(f, 10);
  sink = i2f_div(n, 5);
  sink = twice(f[1]) + sink;
}
int main(int argc, char **argv) {
  float f[16];
  for (int i = 0; i < 16; i++) f[i] = (float)(i * argc);
  dsink = stated_arg(argc);
  dsink = whole_result(argc, 2.0);
  stores(f, argc);
  kills(argc);
  return 0;
}
