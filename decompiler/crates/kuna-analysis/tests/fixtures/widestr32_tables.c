/* int tables of character codes that run on past their zero, none of them
   text: passed is passed whole, spilled is indexed through a pointer an -O0
   build keeps in memory, inner is indexed and passed from its third element,
   peeled is walked by a loop whose first element gcc peels, and keys is held
   by the struct s, indexed and passed.  L"control" is a literal. */
#include <stdio.h>
#include <wchar.h>
#ifdef __clang__
#define NI __attribute__((noinline))
#else
#define NI __attribute__((noipa))
#endif
struct S { const int *codes; int n; };
static const int passed[] = {97, 98, 99, 100, 101, 0, 7, 8};
static const int spilled[] = {97, 98, 99, 100, 101, 0, 120, 121};
static const int inner[] = {97, 98, 99, 100, 101, 102, 103, 104, 0, 7, 9};
static const int peeled[] = {97, 98, 99, 100, 101, 102, 0, 7};
static const int keys[] = {113, 119, 101, 114, 116, 121, 0, 122, 120, 27};
static const struct S s = {keys, 10};
static unsigned h = 5;
NI int sum(const int *p, int n) { int r = 0; for (int i = 0; i < n; i++) r = r * 7 + p[i] * (i + 1); return r; }
NI long hashw(const wchar_t *w) { long r = 7; for (long i = 0; w[i]; i++) r = r * 31 + w[i]; return r; }
NI int use_passed(void) { return sum(passed, 8); }
NI int get(int i) { const int *p = spilled; return p[i]; }
NI int idx(int i) { return inner[i]; }
NI int use_inner(void) { return sum(&inner[2], 9); }
NI void f(int v) { h = h * 33 + v; }
NI void walk(void) { for (int i = 0; i < 8; i++) f(peeled[i]); }
NI const struct S *gs(void) { return &s; }
NI int pick(int i) { return keys[i]; }
NI int use_keys(void) { const struct S *q = gs(); return sum(q->codes, q->n) + sum(keys, 10); }
NI long control(void) { return hashw(L"control"); }
int main(void)
{
    long r = use_passed() + use_inner() + use_keys() + control();
    for (int i = 0; i < 11; i++) r = r * 31 + idx(i);
    for (int i = 0; i < 8; i++) r = r * 31 + get(i);
    for (int i = 0; i < 10; i++) r = r * 7 + pick(i);
    walk();
    printf("%ld %u\n", r, h);
    return 0;
}
