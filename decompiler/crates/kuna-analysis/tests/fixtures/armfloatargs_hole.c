/* Synthetic regression source, licensed with this repository (Apache-2.0). */
__attribute__((noinline)) double hole6(double a, double b, double c, float e, double g, double h) { return 4 * a - 7 * b - 2 * c - 6 * e - 4 * g - 7 * h + 3; }
__attribute__((noinline)) double fill6(double a, double b, double c, float e, float g, float h) { return 4 * a - b - 8 * c - 5 * e - 8 * g - h + 4; }
int top(int n) {
  double r = 0; float f = n * 0.5f; double d = n * 0.25;
  r += hole6(d * 3, (double)f, d * 3, f * 2, (double)f, (double)f);
  r += fill6(d * 3, d * 5 - 1, d, (float)d, f * 2, f + 1);
  return (int)(r * 4);
}
