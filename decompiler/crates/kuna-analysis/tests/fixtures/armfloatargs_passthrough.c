/* Synthetic regression source, licensed with this repository (Apache-2.0). */
__attribute__((noinline)) float clampf(float a, float b, int c) { if (c > 3) { a = b * b + 1.0f; a = a * b - 2.0f; a = a / (b + 7.0f); } return a; }
__attribute__((noinline)) double clampd(double a, double b, int c) { if (c > 3) { a = b * b + 1.0; a = a * b - 2.0; a = a / (b + 7.0); } return a; }
__attribute__((noinline)) float pickf(float a, float b, int c) { if (c > 3) a = b; return a; }
__attribute__((noinline)) float gf(int n) { return n * 0.75f + 1.0f; }
__attribute__((noinline)) float hf(int n) { return n * 1.25f - 1.0f; }
__attribute__((noinline)) double gd(int n) { return n * 0.75 + 1.0; }
__attribute__((noinline)) float wf(int n) { float v = gf(n); return clampf(v, 2.5f, n) * 3.0f; }
__attribute__((noinline)) double wd(int n) { double v = gd(n); return clampd(v, 2.5, n) * 3.0; }
__attribute__((noinline)) float wp(int n) { float v = gf(n); return pickf(v, 2.5f, n) * 3.0f; }
__attribute__((noinline)) float wq(int n) { float v = (n & 1) ? gf(n) : hf(n); return pickf(v, 1.5f, n) * 3.0f; }
int top(int n) { return (int)((wf(n) + wd(n) + wp(n) + wq(n)) * 4.0f); }
