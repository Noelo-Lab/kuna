/* int tables of character codes whose element past the zero something names:
   tail passes &tbl[6], and show passes the count of k, a struct whose codes end
   in a zero.  L"control" is a literal. */
#include <stdio.h>
#include <wchar.h>
#ifdef __clang__
#define NI __attribute__((noinline))
#else
#define NI __attribute__((noipa))
#endif
struct K { int codes[6]; int count; int extra; };
static const int tbl[] = {97, 98, 99, 100, 101, 0, 7, 8};
static const struct K k = {{97, 98, 99, 100, 101, 0}, 5, 9};
NI int sum(const int *p, int n) { int s = 0; for (int i = 0; i < n; i++) s += p[i] * (i + 1); return s; }
NI int use(void) { return sum(tbl, 8); }
NI int tail(void) { return sum(tbl + 6, 2); }
NI int usek(const struct K *p) { return p->codes[1] + p->count * 100 + p->extra * 7; }
NI int show(const int *c) { return *c * 3; }
NI long hashw(const wchar_t *w) { long r = 7; for (long i = 0; w[i]; i++) r = r * 31 + w[i]; return r; }
NI long control(void) { return hashw(L"control"); }
int main(void) { printf("%d %d %d %d %ld\n", use(), tail(), usek(&k), show(&k.count), control()); return 0; }
