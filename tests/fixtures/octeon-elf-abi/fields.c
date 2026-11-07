/* Original direct-field, nested aggregate and register/stack boundary probes. */
typedef unsigned long long U;
struct Inner { double d; };
struct Nested { struct Inner a; };
struct ArrayDouble { double a[1]; };
struct FloatDouble { float a; double b; };
struct DoubleFloat { double a; float b; };
struct QuadDouble { long double q; double d; };
struct TwoQuads { long double a, b; };

double nested_first(struct Nested x) { return x.a.d; }
struct Nested nested_identity(struct Nested x) { return x; }
double array_first(struct ArrayDouble x) { return x.a[0]; }
struct ArrayDouble array_identity(struct ArrayDouble x) { return x; }
struct FloatDouble float_double(struct FloatDouble x) { return x; }
struct DoubleFloat double_float(struct DoubleFloat x) { return x; }
double quad_double_last(struct QuadDouble x) { return x.d; }
double quad_double_after(U a, struct QuadDouble x, U b) { return x.d; }
double quad_double_split(U a, U b, U c, U d, U e, U f, struct QuadDouble x) { return x.d; }
struct FloatDouble float_double_split(U a, U b, U c, U d, U e, U f, U g, struct FloatDouble x) { return x; }
struct TwoQuads two_quads(struct TwoQuads x) { return x; }
