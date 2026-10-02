/* Synthetic regression source, licensed with this repository (Apache-2.0). */
__attribute__((noinline)) float scale(int *p, double d, int k) { return (float)(d * k) + *p; }
float use(int *p, int n) { return scale(p, n * 0.5, n) + 1; }
