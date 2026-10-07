/* An == or != on the bits of a float is an integer comparison: -0.0 and +0.0
   differ, a NaN equals itself and a NaN constant matches its own pattern.
   Built as floateq_x86_64_gcc_O2.o (gcc -O2 -c), floateq_x86_64_clang_O2.o
   (clang -O2 -c), floateq_a64_O2.o (clang --target=aarch64-linux-gnu -O2 -c)
   and floateq_m4f_O2.o (clang --target=armv7em-none-eabi -mcpu=cortex-m4
   -mfpu=fpv4-sp-d16 -mfloat-abi=hard -mthumb -O2 -c). */
typedef union { float f; unsigned u; } fu;
typedef union { double d; unsigned long long u; } du;

int fq_negzero(float x) { fu v = { x }; if (v.u == 0x80000000u) return 7; return (int)x; }
int fq_qnan(float x) { fu v = { x }; if ((v.u & 0x7fffffffu) == 0x7fc00000u) return 7; return (int)x; }
int fq_two(float x, float y) { fu a = { x }, b = { y }; if (a.u != b.u) return 3; return (int)(x - y); }
int fq_dnegzero(double x) { du v = { x }; if (v.u == 0x8000000000000000ull) return 5; return (int)x; }
int fq_dqnan(double x) { du v = { x }; if ((v.u & 0x7fffffffffffffffull) == 0x7ff8000000000000ull) return 5; return (int)x; }
int fq_one(float x) { fu v = { x }; if (v.u == 0x3f800000u) return 4; return (int)x; }
int fq_feq(float x, float y) { return x == y; }
int fq_dfne(double x, double y) { return x != y; }
