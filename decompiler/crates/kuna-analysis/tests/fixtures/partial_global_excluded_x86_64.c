typedef unsigned long U64;
typedef unsigned U32;

union B { U64 q; U32 w[2]; } excluded_g;
U64 *excluded_gp;
extern U64 *excluded_give(void);

U64 memory_root(U32 a, U32 b) {
    U64 *p = excluded_gp;
    excluded_g.w[0] = a;
    excluded_g.w[1] = b;
    U64 x = *p;
    excluded_g.w[0] = b;
    U64 y = *p;
    excluded_g.q = 0;
    return x ^ y;
}

U64 call_root(U32 a, U32 b) {
    U64 *p = excluded_give();
    excluded_g.w[0] = a;
    excluded_g.w[1] = b;
    U64 x = *p;
    excluded_g.w[0] = b;
    U64 y = *p;
    excluded_g.q = 0;
    return x ^ y;
}

U64 memory_cycle(U32 a, U32 b) {
    U64 *p = excluded_gp;
    for (U32 i = 0; i < 3; ++i) p = (i & 1) ? excluded_gp : p;
    excluded_g.w[0] = a;
    excluded_g.w[1] = b;
    U64 x = *p;
    excluded_g.w[0] = b;
    U64 y = *p;
    excluded_g.q = 0;
    return x ^ y;
}

int main(void) { return 0; }
