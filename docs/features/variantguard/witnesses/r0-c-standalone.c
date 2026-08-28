#include <stdio.h>
struct kv { unsigned long code; unsigned long val; };
__attribute__((noinline)) struct kv c_pick(unsigned x) {
    struct kv r;
    if (x > 10) { r.code = 0; r.val = (unsigned long)x * 3; }
    else        { r.code = 1; r.val = (unsigned long)x + 1; }
    return r;
}
__attribute__((noinline)) unsigned long c_kindval(unsigned x) {
    struct kv r = c_pick(x);
    return r.code ? r.val + 100 : r.val;
}
/* a hand-rolled tagged union, the closest C analogue of a Rust enum */
enum k { K_I, K_F };
struct tu { enum k kind; union { long i; double f; } u; };
__attribute__((noinline)) struct tu mk(int x) {
    struct tu t;
    if (x > 3) { t.kind = K_I; t.u.i = x; } else { t.kind = K_F; t.u.f = (double)x; }
    return t;
}
__attribute__((noinline)) double use_tu(int x) {
    struct tu t = mk(x);
    if (t.kind == K_I) return (double)t.u.i;
    return t.u.f + 1.0;
}
int main(int argc, char **argv) { printf("%lu %f\n", c_kindval(argc), use_tu(argc)); return 0; }
