long long __attribute__((noinline)) f(int x);
long long __attribute__((noinline)) g(int x);
volatile long long sink;
volatile int src = 5;
void __start(void) {
    sink = f(src);
    sink = f(src + 7) + 1;
    sink = g(src * 3);
    for (;;) ;
}
long long __attribute__((noinline)) f(int x) { return g(x + 1); }
long long __attribute__((noinline)) g(int x) { return (long long)x; }
