/* Synthetic regression source, licensed with this repository (Apache-2.0). */
__attribute__((noinline)) double wl(double a, float b) { return a + 6 * b + 3; }
__attribute__((noinline)) float w8(double a) { return wl(a, 3.5f) * 3 + a; }
__attribute__((noinline)) float w9(double a) { return wl(a, 3.5f) * 3; }
__attribute__((noinline)) float w10(double a, float b) { return wl(a, b) * 3 + a; }
int top(int n) {
  double r = 0; float f = n * 0.5f; double d = n * 0.25;
  r += w8(d * 3);
  r += w9(d + 1);
  r += w10(d + 2, f);
  return (int)(r * 4);
}
