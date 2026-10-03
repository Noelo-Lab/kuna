/* A 64-bit value on a 32-bit ABI travels in two registers. Every big-endian
 * ABI here (PowerPC r3:r4, MIPS o32 v0:v1, SPARC o0:o1, ARM BE r0:r1) returns
 * the HIGH word in the first register; the little-endian builds are controls.
 * `same` and `keep_zero` return one register: on SPARC `restore` hands the
 * second argument back in %o1 as well, which is not part of the value. */
long long wide_mul(int a, int b) { return (long long)a * b; }
long long add_one(long long v) { return v + 1; }
static __attribute__((noinline)) long long triple(int x) { return (long long)x * 3; }
long long triple_plus(int x) { return triple(x) + 7; }
unsigned int same(unsigned int a, unsigned int b) { return a == b; }
int keep_zero(int *p, int b) { *p = b; return 0; }
