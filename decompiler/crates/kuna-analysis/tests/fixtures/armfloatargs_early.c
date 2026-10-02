/* Synthetic regression source, licensed with this repository (Apache-2.0). */
__attribute__((noinline)) float C2(float p0, float p1, float p2, float p3, int k) { if (k > 6) p2 = p0; else if (k < 4) p2 = (p3 + p0) / 2; return p2; }
__attribute__((noinline)) float C9(float p0, float p1, float p2, int k) { if (k > 6) return p0; float t = p2; for (int i = 0; i < k; i++) t = t * p1 + 1; return t; }
__attribute__((noinline)) float GF2(int n) { return n * 0.75f + 5; }
__attribute__((noinline)) float W2(float f, int n) { float v2 = GF2(n); float r = C2(5.5f, f, v2, (float)(n * 0.5), n); return r * 3; }
__attribute__((noinline)) float W9(float f, int n) { float v2 = GF2(n); float r = C9(5.5f, f, v2, n); return r * 3; }
int top(int n) { return (int)((W2(n * 0.25f, n) + W9(n * 0.5f, n)) * 4); }
