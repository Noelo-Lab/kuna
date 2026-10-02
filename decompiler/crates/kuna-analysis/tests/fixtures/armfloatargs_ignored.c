/* Synthetic regression source, licensed with this repository (Apache-2.0). */
__attribute__((noinline)) double i4(double unused, float a, float b, double d) { return a - 9 * b - 9 * d + 3; }
__attribute__((noinline)) float w3(float a) { return i4(8.25, a, a, 5.25) * 3 + a; }
__attribute__((noinline)) double i2(double unused, double d) { return d * 5 + 1; }
__attribute__((noinline)) float u1(float x) { return i2(1.0, 2.0) * x; }
__attribute__((noinline)) float u2(float x, float y) { return i2(1.0, (double)y) * x + y; }
__attribute__((noinline)) double i3(double unused, float a, double d) { return a * 3 + d; }
__attribute__((noinline)) float u3(float x, double z) { return i3(0.5, x, z) * 2 + x; }
__attribute__((noinline)) float h2(float a, float b) { return a * 2 - b; }
__attribute__((noinline)) float q2(float a, float b) { return h2(b, b) * a; }
__attribute__((noinline)) float k3(float a, float unused, float c) { return a * c; }
__attribute__((noinline)) float gk(float x) { return x * 1.5f + 2; }
__attribute__((noinline)) float kd(double unused, float a) { return a * 3 + 1; }
__attribute__((noinline)) float wk(float x) { float t = gk(x); return kd(1.0, x) + t; }
int top(int n) {
  float f = n * 0.5f; double d = n * 0.25;
  float r = q2(f, f + 1);
  r += k3(f, f * 9, f + 7);
  return (int)((w3(f) + d + u1(f) + u2(f, f + 1) + u3(f, d + 1) + r + wk(f - 1)) * 4);
}
