/* One function that hands the address of each of 700 stack structs to a call.
 * Every struct stays live across every later call, so heritage guards each
 * stack range at each call: ranges x calls INDIRECT ops (#1034).
 * Build: clang -O0 -fno-pic -no-pie -nostdlib -Wl,-e,big -o escaped_locals escaped_locals.c */
struct S { int a; long b; char c[24]; };

__attribute__((noinline)) int cb(void *p) { return *(int *)p; }

__attribute__((noinline)) int reg(const char *n, void *p, int i, int (*f)(void *)) {
    return f(p) + i + *n;
}

#define L(i) { struct S s; s.a = (i); s.b = r; r += reg("n", &s, (i), cb); }
#define L10(i) L(i##0) L(i##1) L(i##2) L(i##3) L(i##4) L(i##5) L(i##6) L(i##7) L(i##8) L(i##9)
#define L100(i) L10(i##0) L10(i##1) L10(i##2) L10(i##3) L10(i##4) \
                L10(i##5) L10(i##6) L10(i##7) L10(i##8) L10(i##9)

int big(void) {
    int r = 0;
    L100(1) L100(2) L100(3) L100(4) L100(5) L100(6) L100(7)
    return r;
}
