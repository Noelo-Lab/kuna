/* Original optional execution checks, compiled separately from abi.c. */
typedef unsigned long long U;
struct Floats { float a, b; };
struct Doubles { double a, b; };
struct Mixed { double a; U b; };
struct Quad { long double a; };
struct Pair { U a, b; };
struct AlignedDoubles { double a, b; } __attribute__((aligned(16)));
union F { unsigned bits; float value; };
union D { U bits; double value; };
union Q { U bits[2]; long double value; };
extern U fixture_global;
extern U read_global(void);
extern float scalar_after(U, float);
extern float scalar_stack(U,U,U,U,U,U,U,U,float);
extern long double quad(long double);
extern long double quad_after(U,long double);
extern U after_quad(U,long double,U);
extern struct Floats floats(struct Floats);
extern struct Doubles doubles(struct Doubles);
extern struct Mixed mixed(struct Mixed);
extern struct Quad quad_struct(struct Quad);
extern double named_varargs(double,...);
extern double unnamed_varargs(U,...);
extern U indirect(U (*)(U),U);
extern int signed_indirect(int (*)(int),int);
extern U void_indirect(void (*)(U),U);
extern struct Pair pair_indirect(struct Pair (*)(U,U),U,U);
extern long double quad_indirect(long double (*)(long double),long double);
extern double aligned_second(U,struct AlignedDoubles,U);
extern U aligned_follow(U,struct AlignedDoubles,U);
extern U narrow_s8(signed char);
extern U narrow_u8(unsigned char);
extern U narrow_u16(unsigned short);
extern U narrow_u32(unsigned);

static U seen;
static U transform(U x) { return x ^ 0x123456789abcdef0ULL; }
static int signed_transform(int x) { return x ^ 0x4444; }
static void observe(U x) { seen = x; }
static struct Pair pair(U a,U b) { struct Pair p = {a ^ 0xfedcba9876543210ULL, b + 7}; return p; }
static long double quad_copy(long double x) { return x; }

#define CHECK(condition, code) do { if (!(condition)) return code; } while (0)
#define ARGS x,x+1,x+2,x+3,x+4,x+5,x+6,x+7
int main(void) {
    U x = 0;
    for (unsigned i = 0; i < 256; i++) {
        if (i == 0) x = 0;
        else if (i == 1) x = ~0ULL;
        else if (i == 2) x = 1ULL << 63;
        else x = x * 6364136223846793005ULL + 1442695040888963407ULL;
        union F f = {.bits = i == 0 ? 0x80000000U : 0x3f000000U | ((unsigned)x & 0x7fffffU)}, fr;
        union D d = {.bits = i == 0 ? 0x8000000000000000ULL : 0x3fe0000000000000ULL | (x & 0xfffffffffffffULL)}, dr;
        union Q q = {.bits = {0x3fff123456789abcULL,x}}, qr;
        fixture_global = x;
        CHECK(read_global() == x, 1);
        fr.value = scalar_after(x,f.value); CHECK(fr.bits == f.bits, 2);
        fr.value = scalar_stack(ARGS,f.value); CHECK(fr.bits == f.bits, 3);
        qr.value = quad(q.value); CHECK(qr.bits[0] == q.bits[0] && qr.bits[1] == q.bits[1], 4);
        qr.value = quad_after(x,q.value); CHECK(qr.bits[0] == q.bits[0] && qr.bits[1] == q.bits[1], 5);
        CHECK(after_quad(x,q.value,~x) == ~x, 6);
        struct Floats fs = {f.value,1.25f}, fo = floats(fs);
        fr.value = fo.a; CHECK(fr.bits == f.bits && fo.b == 1.25f, 7);
        struct Doubles ds = {d.value,2.5}, dout = doubles(ds);
        dr.value = dout.a; CHECK(dr.bits == d.bits && dout.b == 2.5, 8);
        struct Mixed ms = {d.value,x}, mo = mixed(ms);
        dr.value = mo.a; CHECK(dr.bits == d.bits && mo.b == x, 9);
        struct Quad qs = {q.value}, qo = quad_struct(qs);
        qr.value = qo.a; CHECK(qr.bits[0] == q.bits[0] && qr.bits[1] == q.bits[1], 10);
        dr.value = named_varargs(d.value,x); CHECK(dr.bits == d.bits, 11);
        dr.value = unnamed_varargs(x,d.value); CHECK(dr.bits == d.bits, 12);
        CHECK(indirect(transform,x) == (x ^ 0x123456789abcdef0ULL) + x, 13);
        int small = (int)i - 128;
        CHECK(signed_indirect(signed_transform,small) == (small ^ 0x4444) + small, 14);
        seen = ~x; CHECK(void_indirect(observe,x) == x && seen == x, 15);
        struct Pair p = pair_indirect(pair,x,~x);
        CHECK(p.a == (x ^ 0xfedcba9876543210ULL) && p.b == ~x + 7, 16);
        qr.value = quad_indirect(quad_copy,q.value); CHECK(qr.bits[0] == q.bits[0] && qr.bits[1] == q.bits[1], 17);
        struct AlignedDoubles ad = {2.5,d.value};
        dr.value = aligned_second(x,ad,~x); CHECK(dr.bits == d.bits, 18);
        CHECK(aligned_follow(x,ad,~x) == ~x, 19);
        CHECK(narrow_s8((signed char)i) == (U)(long long)(i < 128 ? (int)i : (int)i - 256), 20);
        CHECK(narrow_u8((unsigned char)i) == i, 21);
        CHECK(narrow_u16((unsigned short)x) == (x & 0xffff), 22);
        CHECK(narrow_u32((unsigned)x) == (x & 0xffffffff), 23);
    }
    return 0;
}
