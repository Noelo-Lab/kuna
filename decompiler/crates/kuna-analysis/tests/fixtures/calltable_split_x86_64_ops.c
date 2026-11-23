/* (kuna) Fixture calltable_split_x86_64, file 1 of 3: an unchecked indexed
 * call through `ops`, whose table is followed in .rodata by bsw's (file 2).
 * Recipe: kuna-analysis/tests/fixtures/README.md. */
volatile int sink;
int bsw(unsigned x);
int sw(unsigned x);
int vrun(unsigned i, int x);
int fa(int), fb(int), fc(int), fd(int), fe(int), ff(int);
static int h0(int x) { sink = x; return bsw(x) + 1; }
static int h1(int x) { sink = x + 1; return x * 5 + 2; }
static int h2(int x) { sink = x + 2; return x * 7 + 3; }
static int h3(int x) { sink = x + 3; return x * 9 + 4; }
static int (*const ops[4])(int) = {h0, h1, h2, h3};
__attribute__((noinline)) int run(unsigned i, int x) { return ops[i](x) + 1; }
void _start(void) {
    int s = fa(sink) + fb(sink) + fc(sink) + fd(sink) + fe(sink) + ff(sink);
    s += sw(sink) + vrun(sink, s) + run(sink, s);
    __asm__ volatile("syscall" :: "a"(60), "D"(s));
    for (;;) {}
}
