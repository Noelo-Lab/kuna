/* Original freestanding probes for the BE N32/N64 soft-float ABI. */
struct Small { int x; };
struct Pair { long long a, b; };
struct Twelve { int x, y, z; };
struct Quad { long double x; };
struct Layout { char x; long double q; };
union SmallUnion { int x; float f; };

long long pair_first(struct Pair p) { return p.a; }
long long pair_second(struct Pair p) { return p.b; }
int small_field(struct Small s) { return s.x; }
struct Small small_result(int x) { struct Small s = {x}; return s; }
union SmallUnion union_result(int x) { union SmallUnion s = {x}; return s; }
struct Pair pair_identity(struct Pair p) { return p; }
struct Twelve twelve_identity(struct Twelve p) { return p; }
float float_identity(float f) { return f; }
long long quad_after(long long a, long double b, long long c) { return c; }
long double quad_identity(long double b) { return b; }
struct Quad quad_struct_identity(struct Quad b) { return b; }
__int128 integer_identity(__int128 b) { return b; }
unsigned layout_size(void) { return sizeof(struct Layout); }
unsigned layout_offset(void) { return __builtin_offsetof(struct Layout, q); }

#define SEVEN long long p0, long long p1, long long p2, long long p3, \
    long long p4, long long p5, long long p6
#define EIGHT SEVEN, long long p7
#define NINE EIGHT, long long p8
long long pair_split_first(SEVEN, struct Pair p) { return p.a; }
long long pair_split_second(SEVEN, struct Pair p) { return p.b; }
long long pair_stack_second(EIGHT, struct Pair p) { return p.b; }
long long quad_last_after(SEVEN, long double q, long long value) { return value; }
long long quad_stack_after(NINE, long double q, long long value) { return value; }
float float_stack(EIGHT, float value) { return value; }
