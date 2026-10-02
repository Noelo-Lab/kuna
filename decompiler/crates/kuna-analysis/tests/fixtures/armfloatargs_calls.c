/* Synthetic regression source, licensed with this repository (Apache-2.0). */
__attribute__((noinline)) double k7(float a, float b, double d, float c) { return a + b * 2 + d * 3 + c * 5; }
__attribute__((noinline)) double x3(float a, float b, float c, double d) { return k7(a, b, d, c) + 3; }
__attribute__((noinline)) double g1(float a, float b, float c, double d) { return a - b * 2 + c * 3 + d * 5; }
__attribute__((noinline)) double v2(float a, float b, float c, double d) { return g1(c, b, a, d) + 1; }
__attribute__((noinline)) double fn3(float a, double d, double e, float b, float c) { return a + d * 2 + e * 3 + b * 5 + c * 13; }
__attribute__((noinline)) double fn5(float a, float unused, double d) { return a * 2 + d * 3; }
__attribute__((noinline)) double kd(double d, float a) { return d * 7 - a; }
__attribute__((noinline)) double yd(float a, double unused, double d) { return kd(d, a) + 1; }
int top(int n) {
  double r = 0;
  float f = n * 0.5f; double d = n * 0.25;
  r += x3(f, f * 9, f * 11, d * 13);
  r += v2(f * 3, f + 5, f - 1, d + 2);
  r += fn3(f, d, d + 1, n * 0.75f, n * 1.25f);
  r += fn5(f, n * 3.0f, d * 2);
  r += yd(f, d * 3, d * 7);
  return (int)(r * 16);
}
