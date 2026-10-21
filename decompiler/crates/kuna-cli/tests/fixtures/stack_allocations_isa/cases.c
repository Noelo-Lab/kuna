extern unsigned observe(unsigned char *, unsigned);
extern unsigned observe_aligned(unsigned char *, unsigned);
extern unsigned observe_many(unsigned char *, unsigned, unsigned, unsigned,
                             unsigned, unsigned, unsigned, unsigned, unsigned,
                             unsigned, unsigned, unsigned);
#define KEEP __attribute__((noinline))
KEEP unsigned sample_one(unsigned n) {
  unsigned char *p = __builtin_alloca(n);
  p[0] = 7;
  return observe(p, n);
}
KEEP unsigned sample_two(unsigned n) {
  unsigned char *p = __builtin_alloca(n);
  unsigned char *q = __builtin_alloca(n + 17);
  p[0] = 3;
  q[0] = 7;
  return observe(p, n) + observe(q, n + 17);
}
KEEP unsigned sample_branch(unsigned n) {
  unsigned char *p;
  if (n & 1)
    p = __builtin_alloca(n);
  else
    p = __builtin_alloca(n + 17);
  p[0] = 7;
  return observe(p, n);
}
KEEP unsigned sample_loop(unsigned n) {
  unsigned total = 0;
  for (unsigned i = 0; i < 3; i++) {
    unsigned char *p = __builtin_alloca(n + i);
    p[0] = (unsigned char)(7 + i);
    total += observe(p, n + i);
  }
  return total;
}
KEEP unsigned sample_scoped(unsigned n) {
  unsigned total = 0;
  for (unsigned i = 0; i < 3; i++) {
    unsigned char p[n + i];
    p[0] = (unsigned char)(7 + i);
    total += observe(p, n + i);
  }
  return total;
}
KEEP unsigned sample_aligned(unsigned n) {
  unsigned char *p = __builtin_alloca_with_align(n, 512);
  p[0] = 7;
  return observe_aligned(p, n);
}
KEEP unsigned sample_outgoing(unsigned n) {
  unsigned char *p = __builtin_alloca(n);
  p[0] = 7;
  return observe_many(p, n, n + 1, n + 2, n + 3, n + 4, n + 5, n + 6, n + 7,
                      n + 8, n + 9, n + 10);
}
