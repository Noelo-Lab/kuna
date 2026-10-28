unsigned observed_calls, observed_digest;
__attribute__((noinline)) unsigned observe(unsigned char *p, unsigned n) {
  observed_calls++;
  observed_digest = observed_digest * 1664525u + n * 31u + p[0];
  volatile unsigned char clobber[256];
  for (unsigned i = 0; i < 256; i++)
    clobber[i] = (unsigned char)i;
  p[n - 1] = 9;
  return p[0] + p[n - 1] + clobber[7] - 7;
}
unsigned observe_aligned(unsigned char *p, unsigned n) {
  return observe(p, n) + ((__UINTPTR_TYPE__)p & 63);
}
unsigned observe_many(unsigned char *p, unsigned n, unsigned a, unsigned b,
                      unsigned c, unsigned d, unsigned e, unsigned f,
                      unsigned g, unsigned h, unsigned i, unsigned j) {
  return observe(p, n) + a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f + 7 * g +
         8 * h + 9 * i + 10 * j;
}
