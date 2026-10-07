/* gcc 14 -O0, aarch64-linux-gnu: aarch64-linux-gnu-gcc-14 -O0 -c zerofillfwd.c -o zerofillfwd_a64.o */
#include <complex.h>
struct D2 { double a, b; };
extern _Complex double get(const _Complex double *p);
extern int cnt;
_Complex double fa(const _Complex double *p) { _Complex double r = 0; _Complex double *pr = &r; *pr = get(p); cnt++; return r; }
struct D2 mk(double x) { double y = x * x; return (struct D2){ y + 1, y }; }
