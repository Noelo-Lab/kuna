/* clang 14 -O2 --target=aarch64-linux-gnu -c floatbits.c -o floatbits_a64.o */
__attribute__((noinline)) double getd(int a, const double *g) { return *g * a + 0.5; }
__attribute__((noinline)) float getf(int a, const float *g) { return *g * a + 0.5f; }
__attribute__((noinline)) unsigned long to_bits(int a, const double *g) { double d = getd(a, g); unsigned long u; __builtin_memcpy(&u, &d, 8); return u; }
__attribute__((noinline)) unsigned int to_fbits(int a, const float *g) { float f = getf(a, g); unsigned int u; __builtin_memcpy(&u, &f, 4); return u; }
__attribute__((noinline)) int exponent(int a, const double *g) { double d = getd(a, g); unsigned long u; __builtin_memcpy(&u, &d, 8); return (int)(u >> 52 & 0x7ff); }
