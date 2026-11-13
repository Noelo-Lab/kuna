/* Independently compiled expectations for every fields.c probe. */
typedef unsigned long long U;
struct Inner { double d; };
struct Nested { struct Inner a; };
struct ArrayDouble { double a[1]; };
struct FloatDouble { float a; double b; };
struct DoubleFloat { double a; float b; };
struct QuadDouble { long double q; double d; };
struct TwoQuads { long double a, b; };
extern double nested_first(struct Nested);
extern struct Nested nested_identity(struct Nested);
extern double array_first(struct ArrayDouble);
extern struct ArrayDouble array_identity(struct ArrayDouble);
extern struct FloatDouble float_double(struct FloatDouble);
extern struct DoubleFloat double_float(struct DoubleFloat);
extern double quad_double_last(struct QuadDouble);
extern double quad_double_after(U, struct QuadDouble, U);
extern double quad_double_split(U, U, U, U, U, U, struct QuadDouble);
extern struct FloatDouble float_double_split(U, U, U, U, U, U, U, struct FloatDouble);
extern struct TwoQuads two_quads(struct TwoQuads);

static U double_bits(double x) { union { double d; U u; } v = {x}; return v.u; }
static unsigned float_bits(float x) { union { float f; unsigned u; } v = {x}; return v.u; }
int main(void) {
    for (unsigned i = 0; i < 256; ++i) {
        union { unsigned u; float f; } f = {0x3f000000U + i * 12345U};
        union { U u; double d; } d = {0x3fe0000000000000ULL + (U)i * 0x987654321ULL};
        if (i & 1) { f.u |= 0x80000000U; d.u |= 0x8000000000000000ULL; }
        if (i < 2) { f.u = i << 31; d.u = (U)i << 63; }
        struct Nested n = {{d.d}};
        struct ArrayDouble a = {{d.d}};
        struct FloatDouble fd = {f.f, d.d};
        struct DoubleFloat df = {d.d, f.f};
        union { U u[2]; long double q; } q = {{0x3fff123456789abcULL, 0xdef123456789abcdULL + i}};
        struct QuadDouble qd = {q.q, d.d};
        if (double_bits(nested_first(n)) != d.u) return 1;
        if (double_bits(nested_identity(n).a.d) != d.u) return 2;
        if (double_bits(array_first(a)) != d.u) return 3;
        if (double_bits(array_identity(a).a[0]) != d.u) return 4;
        struct FloatDouble rfd = float_double(fd);
        if (float_bits(rfd.a) != f.u || double_bits(rfd.b) != d.u) return 5;
        struct DoubleFloat rdf = double_float(df);
        if (double_bits(rdf.a) != d.u || float_bits(rdf.b) != f.u) return 6;
        if (double_bits(quad_double_last(qd)) != d.u) return 7;
        if (double_bits(quad_double_after(0x1234567887654321ULL, qd, 0xfedcba9876543210ULL)) != d.u) return 8;
        if (double_bits(quad_double_split(11, 22, 33, 44, 55, 66, qd)) != d.u) return 9;
        rfd = float_double_split(11, 22, 33, 44, 55, 66, 77, fd);
        if (float_bits(rfd.a) != f.u || double_bits(rfd.b) != d.u) return 10;
        struct TwoQuads qq = {q.q, -q.q}, rqq = two_quads(qq);
        union { long double q; U u[2]; } qa = {rqq.a}, qb = {rqq.b};
        if (qa.u[0] != q.u[0] || qa.u[1] != q.u[1] || qb.u[0] != (q.u[0] ^ 0x8000000000000000ULL) || qb.u[1] != q.u[1]) return 11;
    }
    return 0;
}
