extern long g(void);
extern long f(void);
long h(void) { long p = g(); long q = f(); return p + q; }
long k(long a) { return f() + a; }
