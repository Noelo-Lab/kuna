/* Original freestanding calling-convention and PIC probes. */
typedef unsigned long long U;
struct Floats { float a, b; };
struct Doubles { double a, b; };
struct Mixed { double a; U b; };
struct Quad { long double a; };
struct Pair { U a, b; };

U fixture_global = 0x123456789abcdef0ULL;
U read_global(void) { return fixture_global; }
float scalar_after(U a, float b) { return b; }
float scalar_stack(U a, U b, U c, U d, U e, U f, U g, U h, float x) { return x; }
long double quad(long double x) { return x; }
long double quad_after(U a, long double x) { return x; }
U after_quad(U a, long double x, U b) { return b; }
struct Floats floats(struct Floats x) { return x; }
struct Doubles doubles(struct Doubles x) { return x; }
struct Mixed mixed(struct Mixed x) { return x; }
struct Quad quad_struct(struct Quad x) { return x; }
double named_varargs(double x, ...) { return x; }
double unnamed_varargs(U a, ...) {
    __builtin_va_list ap;
    __builtin_va_start(ap, a);
    double x = __builtin_va_arg(ap, double);
    __builtin_va_end(ap);
    return x;
}
U indirect(U (*callee)(U), U x) { return callee(x) + x; }
int signed_indirect(int (*callee)(int), int x) { return callee(x) + x; }
U void_indirect(void (*callee)(U), U x) { callee(x); return x; }
struct Pair pair_indirect(struct Pair (*callee)(U, U), U a, U b) { return callee(a, b); }
long double quad_indirect(long double (*callee)(long double), long double x) { return callee(x); }

struct AlignedDoubles { double a, b; } __attribute__((aligned(16)));
double aligned_second(U a, struct AlignedDoubles x, U c) { return x.b; }
U aligned_follow(U a, struct AlignedDoubles x, U c) { return c; }
U narrow_s8(signed char x) { return (long long)x; }
U narrow_u8(unsigned char x) { return x; }
U narrow_u16(unsigned short x) { return x; }
U narrow_u32(unsigned x) { return x; }
